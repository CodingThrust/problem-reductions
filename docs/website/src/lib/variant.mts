export const nodeKey = (node: { name: string; variant: Record<string, string> }): string =>
  JSON.stringify([node.name, Object.entries(node.variant).sort(([a], [b]) => a < b ? -1 : a > b ? 1 : 0)]);

export function validateGraphReferences(
  data: { nodes: { name: string; variant: Record<string, string> }[]; edges: { source: number; target: number }[] },
  error?: () => Error,
): void {
  if (new Set(data.nodes.map(nodeKey)).size !== data.nodes.length) throw error?.() ?? new Error('Duplicate graph variant.');
  if (data.edges.some(edge => !data.nodes[edge.source] || !data.nodes[edge.target])) throw error?.() ?? new Error('Graph edge references an unknown variant.');
}
