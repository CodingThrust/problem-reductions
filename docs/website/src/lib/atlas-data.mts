import { validateAtlas, type AtlasData, type AtlasSummary, type AtlasRecord } from '../generated/atlas-data.validator.mjs';
import { nodeKey } from './variant.mts';

const error = () => new Error('Invalid atlas data. Reload the current release and try again.');

function validateReferences(data: AtlasData): void {
  const variants = new Set<string>();
  for (const node of data.nodes) {
    const key = nodeKey(node);
    if (variants.has(key)) throw error();
    variants.add(key);
  }
  if (data.edges.some(edge => edge.source >= data.nodes.length || edge.target >= data.nodes.length)) throw error();
}

export function validateAtlasSummary(value: unknown): AtlasSummary {
  if (!validateAtlas(value) || value.kind !== 'summary') throw error();
  validateReferences(value);
  const families = new Set(value.nodes.map(node => node.name));
  if (value.schemas.length !== families.size || new Set(value.schemas.map(schema => schema.name)).size !== families.size
      || value.schemas.some(schema => !families.has(schema.name))) throw error();
  return value;
}

export function validateAtlasRecord(value: unknown, family?: string): AtlasRecord {
  if (!validateAtlas(value) || value.kind !== 'record') throw error();
  validateReferences(value);
  const schema = value.schemas[0];
  if ((family !== undefined && schema.name !== family) || !value.nodes.some(node => node.name === schema.name)
      || value.nodes.some(node => node.name === schema.name && node.category !== schema.category)
      || value.edges.some(edge => value.nodes[edge.source]!.name !== schema.name && value.nodes[edge.target]!.name !== schema.name)) throw error();
  return value;
}
