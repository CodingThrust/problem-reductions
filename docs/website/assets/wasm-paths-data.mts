import { validateWasmPaths, type Endpoint, type WasmPathResult } from './wasm-paths.validator.mjs';

function sameEndpoint(left: Endpoint, right: Endpoint): boolean {
  const entries = Object.entries(left.variant);
  return left.name === right.name && entries.length === Object.keys(right.variant).length
    && entries.every(([key, value]) => Object.hasOwn(right.variant, key) && right.variant[key] === value);
}

export function parseWasmPathResult(text: string): WasmPathResult {
  const value: unknown = JSON.parse(text);
  return validateWasmPathResultData(value);
}

export function validateWasmPathResultData(value: unknown): WasmPathResult {
  const error = () => new Error('Invalid path result contract. Reload the current release.');
  if (!validateWasmPaths(value)) throw error();
  if (value.truncated && !value.paths.length) throw error();
  for (const route of value.paths) {
    if (route.steps !== route.path.length) throw error();
    let previous: Endpoint | undefined;
    for (const step of route.path) {
      if (previous && !sameEndpoint(previous, step.from)) throw error();
      previous = step.to;
    }
  }
  return value;
}
