import type { WasmPathResult } from '../generated/wasm-paths.mjs';
import type { WasmComparisonResult } from '../generated/wasm-comparison.mjs';

export type PathRequest = { source: number; target: number; limit: number } & (
  { kind: 'search' } | { kind: 'compare'; cached: WasmPathResult }
);
export type PathReply =
  | { phase: 'paths'; result: WasmPathResult }
  | { phase: 'comparison'; result: WasmComparisonResult }
  | { phase: 'paths' | 'comparison'; error: string };

const error = () => new Error('Invalid path worker contract. Reload the current release.');
export function validatePathRequest(value: PathRequest): PathRequest {
  if (![value.source, value.target].every(index => Number.isSafeInteger(index) && index >= 0)
      || !Number.isInteger(value.limit) || value.limit < 1 || value.limit > 999) throw error();
  return value;
}

export function validatePathReply(value: PathReply, request: PathRequest, result?: WasmPathResult): PathReply {
  if ('error' in value) {
    if (value.phase === 'comparison' && !result?.paths.length) throw error();
  } else if (value.phase === 'paths') {
    if (request.kind !== 'search' || result !== undefined) throw error();
  } else if (!result?.paths.length) throw error();
  return value;
}
