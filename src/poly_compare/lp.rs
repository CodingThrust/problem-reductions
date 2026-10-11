//! Exact Fourier–Motzkin feasibility with back substitution and a finite work cap.
use super::*;

/// Find nonnegative x with A x <= b; every returned point is checked against A,b.
pub(super) fn feasible(
    a: Vec<Vec<BigRational>>,
    b: Vec<BigRational>,
) -> Result<Option<Vec<BigRational>>, Reason> {
    let n = a.first().map_or(0, Vec::len);
    if n > 24 {
        return Err(Reason::ResourceLimit("exact LP variables"));
    }
    let original: Vec<_> = a.into_iter().zip(b).collect();
    let mut rows = original.clone();
    for i in 0..n {
        let mut row = vec![BigRational::zero(); n];
        row[i] = -BigRational::one();
        rows.push((row, BigRational::zero()));
    }
    let mut stages = Vec::new();
    for k in (0..n).rev() {
        let positive: Vec<_> = rows.iter().filter(|(r, _)| r[k].is_positive()).collect();
        let negative: Vec<_> = rows.iter().filter(|(r, _)| r[k].is_negative()).collect();
        if positive.len().saturating_mul(negative.len()) + rows.len() > 512 {
            return Err(Reason::ResourceLimit("exact LP elimination rows"));
        }
        let mut next: BTreeMap<Vec<BigRational>, BigRational> = BTreeMap::new();
        let mut tighten = |row: Vec<BigRational>, rhs: BigRational| {
            next.entry(row)
                .and_modify(|v| *v = v.clone().min(rhs.clone()))
                .or_insert(rhs);
        };
        for (r, b) in rows.iter().filter(|(r, _)| r[k].is_zero()) {
            tighten(r[..k].to_vec(), b.clone());
        }
        for (p, pb) in &positive {
            for (q, qb) in &negative {
                let row = (0..k).map(|i| &p[i] / &p[k] - &q[i] / &q[k]).collect();
                tighten(row, pb / &p[k] - qb / &q[k]);
            }
        }
        stages.push(rows);
        rows = next.into_iter().collect();
        if rows
            .iter()
            .any(|(r, b)| r.iter().all(Zero::is_zero) && b.is_negative())
        {
            return Ok(None);
        }
    }
    let mut point = Vec::new();
    for stage in stages.iter().rev() {
        let k = point.len();
        let mut lower = BigRational::zero();
        let mut upper: Option<BigRational> = None;
        for (r, b) in stage {
            let residual = b - r[..k]
                .iter()
                .zip(&point)
                .map(|(a, x)| a * x)
                .sum::<BigRational>();
            if r[k].is_positive() {
                let bound = residual / &r[k];
                upper = Some(upper.map_or(bound.clone(), |u| u.min(bound)));
            } else if r[k].is_negative() {
                lower = lower.max(residual / &r[k]);
            }
        }
        if upper.is_some_and(|u| lower > u) {
            return Ok(None);
        }
        point.push(lower);
    }
    if !original.iter().all(|(r, b)| {
        r.iter()
            .zip(&point)
            .map(|(a, x)| a * x)
            .sum::<BigRational>()
            <= *b
    }) {
        return Err(Reason::ResourceLimit(
            "LP certificate failed exact verification",
        ));
    }
    Ok(Some(point))
}
