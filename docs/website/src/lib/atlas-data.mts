import { validateAtlas, type AtlasSummary, type AtlasRecord } from '../generated/atlas-data.validator.mjs';
import { validateGraphReferences } from './variant.mts';

const error = () => new Error('Invalid atlas data. Reload the current release and try again.');

export function validateAtlasSummary(value: unknown): AtlasSummary {
  if (!validateAtlas(value) || value.kind !== 'summary') throw error();
  validateGraphReferences(value, error);
  const families = new Set(value.nodes.map(node => node.name));
  if (value.schemas.length !== families.size || new Set(value.schemas.map(schema => schema.name)).size !== families.size
      || value.schemas.some(schema => !families.has(schema.name))) throw error();
  return value;
}

export function validateAtlasRecord(value: unknown, family?: string): AtlasRecord {
  if (!validateAtlas(value) || value.kind !== 'record') throw error();
  validateGraphReferences(value, error);
  const schema = value.schemas[0];
  if ((family !== undefined && schema.name !== family) || !value.nodes.some(node => node.name === schema.name)
      || value.nodes.some(node => node.name === schema.name && node.category !== schema.category)
      || value.edges.some(edge => value.nodes[edge.source]!.name !== schema.name && value.nodes[edge.target]!.name !== schema.name)) throw error();
  return value;
}
