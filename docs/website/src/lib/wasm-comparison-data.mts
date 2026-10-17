import { validateWasmComparison, type WasmComparisonResult } from '../generated/wasm-comparison.validator.mjs';

export function validateComparisonParameters(value: WasmComparisonResult, source: readonly string[], target: readonly string[]): void {
  const coordinates = new Set(source), fields = new Set(target);
  for (const profile of value.profiles) {
    const names = Object.keys(profile.fields);
    if (new Set(profile.source_coordinates).size !== coordinates.size
        || profile.source_coordinates.some(name => !coordinates.has(name))
        || names.length !== fields.size || names.some(name => !fields.has(name))) {
      throw new Error('Comparison parameters do not match the registered endpoints. Reload the current release.');
    }
  }
}

export function parseWasmComparison(text: string, pathCount: number): WasmComparisonResult {
  const value: unknown = JSON.parse(text);
  return validateWasmComparisonData(value, pathCount);
}

export function validateWasmComparisonData(value: unknown, pathCount: number): WasmComparisonResult {
  const error = () => new Error('Invalid comparison result contract. Reload the current release.');
  if (!Number.isSafeInteger(pathCount) || pathCount < 0 || !validateWasmComparison(value)) throw error();
  if (value.profiles.length !== pathCount) throw error();
  const total = BigInt(pathCount) * (BigInt(pathCount) - 1n) / 2n;
  if (Object.values(value.counts).reduce((sum, count) => sum + BigInt(count), 0n) !== total) throw error();
  const front = new Set(value.front);
  if (front.size !== value.front.length || (pathCount > 0 && !front.size)
      || value.front.some(index => index > pathCount)) throw error();
  for (const profile of value.profiles) {
    const coordinates = new Set(profile.source_coordinates);
    if (coordinates.size !== profile.source_coordinates.length) throw error();
    for (const field of Object.values(profile.fields)) {
      if ((field.certificate === null) !== (field.accuracy.kind === 'unavailable')) throw error();
      if (field.certificate !== null) for (const variable of field.certificate.matchAll(/[A-Za-z_]\w*/g)) {
        if (!coordinates.has(variable[0])) throw error();
      }
    }
  }
  return value;
}
