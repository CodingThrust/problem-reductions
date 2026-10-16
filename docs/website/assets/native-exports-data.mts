import { validateNativeExports, type NativeExports } from './native-exports.validator.mjs';
import { validatePathGraphReferences } from './path-graph-data.mts';

export function validateNativeExportData(value: unknown): NativeExports {
  const error = () => new Error('Invalid native registry exports. Regenerate graph and problem schemas together.');
  if (!validateNativeExports(value)) throw error();
  validatePathGraphReferences(value.graph);
  const families = new Set(value.graph.nodes.map(node => node.name));
  const schemas = new Map(value.schemas.map(schema => [schema.name, schema]));
  if (schemas.size !== value.schemas.length || schemas.size !== families.size
      || [...schemas.keys()].some(name => !families.has(name))
      || Object.keys(value.graph.aliases).some(name => !families.has(name))
      || value.graph.nodes.some(node => schemas.get(node.name)?.category !== node.category)) throw error();
  return value;
}
