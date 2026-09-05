#[derive(Debug)]
pub(crate) struct SatVariableAllocator {
    reduction: &'static str,
    next: u64,
}

impl SatVariableAllocator {
    pub(crate) fn new(
        reduction: &'static str,
        existing: usize,
    ) -> Result<Self, crate::registry::ConstructionError> {
        if existing > i64::MAX as usize {
            return Err(format!(
                "{reduction} has {existing} source variables; SAT variable numbers are limited to {}",
                i64::MAX
            ).into());
        }
        Ok(Self {
            reduction,
            next: u64::try_from(existing).expect("usize SAT count fits u64") + 1,
        })
    }

    pub(crate) fn allocate(&mut self) -> Result<i64, crate::registry::ConstructionError> {
        let variable = self.next;
        if variable > i64::MAX as u64 {
            return Err(format!(
                "{} cannot allocate 1 auxiliary variable after {}; SAT variable numbers are limited to {}",
                self.reduction,
                self.num_vars(),
                i64::MAX
            ).into());
        }
        self.next += 1;
        Ok(i64::try_from(variable).expect("checked SAT variable fits i64"))
    }

    pub(crate) fn allocate_many(
        &mut self,
        count: usize,
    ) -> Result<Vec<i64>, crate::registry::ConstructionError> {
        if count == 0 {
            return Ok(Vec::new());
        }
        let count = u64::try_from(count).expect("usize allocation count fits u64");
        let last = self
            .next
            .checked_add(count - 1)
            .ok_or_else(|| format!("{} auxiliary variable count overflow", self.reduction))?;
        if last > i64::MAX as u64 {
            return Err(format!(
                "{} cannot allocate {count} auxiliary variables after {}; SAT variable numbers are limited to {}",
                self.reduction,
                self.num_vars(),
                i64::MAX
            ).into());
        }
        let variables = (self.next..=last)
            .map(|variable| i64::try_from(variable).expect("checked SAT variable fits i64"))
            .collect();
        self.next = last + 1;
        Ok(variables)
    }

    pub(crate) fn num_vars(&self) -> usize {
        usize::try_from(self.next - 1).expect("SAT variable count fits usize")
    }
}

#[cfg(test)]
#[path = "../unit_tests/rules/sat_helpers.rs"]
mod tests;
