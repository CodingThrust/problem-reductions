import { validatePathGraph, type PathGraphData } from '../generated/path-graph.validator.mjs';
import { validateGraphReferences } from './variant.mts';

export function validatePathGraphData(value: unknown): PathGraphData {
  if (!validatePathGraph(value)) throw new Error('Invalid path metadata contract. Reload the current release.');
  validatePathGraphReferences(value);
  return value;
}

export function validatePathGraphReferences(value: Pick<PathGraphData, 'nodes' | 'edges'>): void {
  const error = () => new Error('Invalid path metadata contract. Reload the current release.');
  validateGraphReferences(value, error);
  const defaults = new Map<string, number>();
  for (const node of value.nodes) defaults.set(node.name, (defaults.get(node.name) ?? 0) + (node.default ? 1 : 0));
  if ([...defaults.values()].some(count => count !== 1)) throw error();
  const edges = new Set<string>();
  for (const edge of value.edges) {
    const target = value.nodes[edge.target]!;
    const key = `${edge.source}:${edge.target}`;
    if (edges.has(key)) throw error();
    edges.add(key);
    const fields = new Set(edge.parameters.map(parameter => parameter.field));
    if (fields.size !== edge.parameters.length || [...fields].some(field => !target.parameter_names.includes(field))
        || (edge.parameter_contract_error === null && fields.size !== target.parameter_names.length)) throw error();
  }
}
