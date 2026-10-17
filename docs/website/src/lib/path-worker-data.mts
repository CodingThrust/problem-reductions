import { validatePathRequestEnvelope, type PathRequestEnvelope } from '../generated/path-worker-request.validator.mjs';
import { validatePathReplyEnvelope } from '../generated/path-worker-reply.validator.mjs';
import type { WasmPathResult } from '../generated/wasm-paths.validator.mjs';
import type { WasmComparisonResult } from '../generated/wasm-comparison.validator.mjs';

export type PathRequest = Extract<PathRequestEnvelope, { kind: 'search' }>
  | (Extract<PathRequestEnvelope, { kind: 'compare' }> & { cached: WasmPathResult });
const error = () => new Error('Invalid path worker contract. Reload the current release.');
export function validatePathRequest(value: unknown): PathRequest {
  if (!validatePathRequestEnvelope(value)) throw error();
  // Cached results were produced by this release's engine; Rust validates routes when comparing.
  return value as PathRequest;
}

export function validatePathReply(value: unknown, request: PathRequest, result?: WasmPathResult) {
  if (!validatePathReplyEnvelope(value)) throw error();
  if ('error' in value) {
    if (value.phase === 'comparison' && !result?.paths.length) throw error();
    return value;
  }
  if (value.phase === 'paths') {
    if (request.kind !== 'search' || result !== undefined) throw error();
    return { ...value, result: value.result as WasmPathResult };
  }
  if (!result?.paths.length) throw error();
  return { ...value, result: value.result as WasmComparisonResult };
}
