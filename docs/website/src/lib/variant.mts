export const nodeKey = (node: { name: string; variant: Record<string, string> }): string =>
  JSON.stringify([node.name, Object.entries(node.variant).sort(([a], [b]) => a < b ? -1 : a > b ? 1 : 0)]);
