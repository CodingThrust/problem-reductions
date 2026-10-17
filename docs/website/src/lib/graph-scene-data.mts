import { validateGraphScene, type GraphSceneData } from '../generated/graph-scene.validator.mjs';
import { nodeKey } from './variant.mts';

export function validateGraphSceneData(value: unknown): GraphSceneData {
  if (!validateGraphScene(value)) throw new Error('Invalid graph scene contract. Reload the current release.');
  const families = new Set(value.nodes.map(node => node.name));
  const variants = new Set<string>();
  const defaults = new Map<string, number>();
  for (const node of value.nodes) {
    const key = nodeKey(node);
    if (variants.has(key)) throw new Error('Duplicate graph variant.');
    variants.add(key);
    if (node.default) defaults.set(node.name, (defaults.get(node.name) ?? 0) + 1);
  }
  if ([...families].some(name => defaults.get(name) !== 1)) throw new Error('Each graph family needs exactly one default.');
  if (value.edges.some(edge => edge.source >= value.nodes.length || edge.target >= value.nodes.length)) {
    throw new Error('Graph edge references an unknown variant.');
  }
  if (Object.keys(value.aliases).some(name => !families.has(name))) throw new Error('Graph alias references an unknown family.');
  if (Object.keys(value.layout).length !== families.size || Object.keys(value.layout).some(name => !families.has(name))) {
    throw new Error('Graph layout must cover exactly the registered families.');
  }
  return value;
}
