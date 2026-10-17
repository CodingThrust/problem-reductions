interface GrowthField {
  certificate: string | null;
  accuracy: { kind: 'exact' | 'upper_bound' | 'unavailable' };
}
interface Growth { degree: bigint | null; lead: string }
type Degree = bigint | null | undefined;

const SUP = '⁰¹²³⁴⁵⁶⁷⁸⁹';
const sup = (degree: bigint) => degree.toString().replace(/\d/g, digit => SUP.charAt(digit.charCodeAt(0) - 48));
const memo = new WeakMap<GrowthField, Growth>();

export function growth(field: GrowthField): Growth {
  const cached = memo.get(field);
  if (cached) return cached;
  let out: Growth = { degree: null, lead: 'unavailable' };
  if (field.certificate != null) {
    const terms = field.certificate.split(' + ').map(term => {
      const vars = [...term.matchAll(/([A-Za-z_]\w*)(?:\^([\d.]+))?/g)].map(match => ({
        // Every successful match includes the mandatory variable capture.
        v: match[1]!.replace(/^num_/, ''), e: match[2] ? BigInt(match[2]) : 1n,
      }));
      return { d: vars.reduce((degree, variable) => degree + variable.e, 0n),
        txt: vars.map(variable => variable.v + (variable.e === 1n ? '' : sup(variable.e))).join('·') || '1' };
    });
    const degree = terms.reduce((max, term) => term.d > max ? term.d : max, 0n);
    const lead = [...new Set(terms.filter(term => term.d === degree).map(term => term.txt))];
    out = { degree, lead: (field.accuracy.kind === 'exact' ? 'Θ(' : 'O(') + lead.slice(0, 3).join(' + ')
      + (lead.length > 3 ? ' + …' : '') + ')' };
  }
  memo.set(field, out);
  return out;
}

export function pathDegree(profile: { fields: Record<string, GrowthField> }): bigint | null {
  let max = 0n;
  for (const field of Object.values(profile.fields)) {
    const degree = growth(field).degree;
    if (degree === null) return null;
    if (degree > max) max = degree;
  }
  return max;
}

export function sizeBadge(degree: Degree): string {
  return degree == null ? '?' : degree === 0n ? '1' : degree === 1n ? 'n' : 'n' + sup(degree);
}

export function compareDegrees(left: Degree, right: Degree): number {
  if (left == null) return right == null ? 0 : 1;
  if (right == null) return -1;
  return left < right ? -1 : left > right ? 1 : 0;
}
