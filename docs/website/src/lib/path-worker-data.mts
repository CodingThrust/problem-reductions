import { validatePathRequestEnvelope, type PathRequestEnvelope } from '../generated/path-worker-request.validator.mjs';
import { validatePathReplyEnvelope } from '../generated/path-worker-reply.validator.mjs';
import { validateWasmPathResultData } from './wasm-paths-data.mts';
import { validateWasmComparisonData } from './wasm-comparison-data.mts';
import type { Endpoint, WasmPathResult } from '../generated/wasm-paths.validator.mjs';
import type { GraphSceneData } from '../generated/graph-scene.validator.mjs';
import type { PathGraphData } from '../generated/path-graph.validator.mjs';

type RouteGraph = Pick<GraphSceneData, 'nodes' | 'edges'> | PathGraphData;
export type PathRequest = Extract<PathRequestEnvelope, { kind: 'search' }>
  | (Extract<PathRequestEnvelope, { kind: 'compare' }> & { cached: WasmPathResult });
const error = () => new Error('Invalid path worker contract. Reload the current release.');
const endpointKey = (node: Endpoint) => JSON.stringify([node.name,
  Object.entries(node.variant).sort(([a], [b]) => a < b ? -1 : a > b ? 1 : 0)]);

export function validatePathRoutes(result: WasmPathResult, graph: RouteGraph, request: PathRequest): void {
  if (result.paths.length > request.limit || (result.truncated && result.paths.length !== request.limit)) throw error();
  const keys = graph.nodes.map(endpointKey), nodes = new Map(keys.map((key, index) => [key, index]));
  const links = new Set(graph.edges.filter(edge => edge.witness && !edge.turing)
    .map(edge => `${edge.source}:${edge.target}`));
  const routes = new Set<string>();
  for (const route of result.paths) {
    let at = request.source;
    const visited = new Set([at]), indices = [at];
    for (const step of route.path) {
      const next = nodes.get(endpointKey(step.to));
      if (endpointKey(step.from) !== keys[at] || next === undefined
          || !links.has(`${at}:${next}`) || visited.has(next)) throw error();
      at = next; visited.add(at); indices.push(at);
    }
    const key = indices.join(',');
    if (at !== request.target || routes.has(key)) throw error();
    routes.add(key);
  }
}

export function validatePathRequest(value: unknown, graph: RouteGraph): PathRequest {
  if (!validatePathRequestEnvelope(value) || value.source >= graph.nodes.length || value.target >= graph.nodes.length) throw error();
  if (value.kind === 'search') return value;
  const cached = validateWasmPathResultData(value.cached);
  if (!cached.paths.length) throw error();
  const request = { ...value, cached };
  validatePathRoutes(cached, graph, request);
  return request;
}

export function validatePathReply(value: unknown, graph: RouteGraph, request: PathRequest, result?: WasmPathResult) {
  if (!validatePathReplyEnvelope(value)) throw error();
  if ('error' in value) {
    if (value.phase === 'comparison' && !result?.paths.length) throw error();
    return value;
  }
  if (value.phase === 'paths') {
    if (request.kind !== 'search' || result !== undefined) throw error();
    const paths = validateWasmPathResultData(value.result);
    validatePathRoutes(paths, graph, request);
    return { ...value, result: paths };
  }
  if (!result?.paths.length) throw error();
  return { ...value, result: validateWasmComparisonData(value.result, result.paths.length, 'summary') };
}
