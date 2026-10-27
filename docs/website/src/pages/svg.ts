// Shared SVG element factory for the hand-written scenes.
export function make<Tag extends keyof SVGElementTagNameMap>(tag: Tag, attrs: Record<string, string | number> = {}) {
  const node = document.createElementNS('http://www.w3.org/2000/svg', tag);
  for (const [key, value] of Object.entries(attrs)) node.setAttribute(key, String(value));
  return node;
}
