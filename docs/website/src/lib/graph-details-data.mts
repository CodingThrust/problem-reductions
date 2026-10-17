import { validateGraphDetails, type GraphDetailsIndex } from '../generated/graph-details.validator.mjs';

export function validateDetailsIndexData(value: unknown, page: URL, articles: URL): GraphDetailsIndex {
  const error = () => new Error('Invalid documentation index. Reload the current release.');
  if (!validateGraphDetails(value)) throw error();
  for (const entry of Object.values(value.entries)) {
    let url: URL;
    let path: string;
    try {
      url = new URL(entry, page);
      path = decodeURIComponent(url.pathname);
    } catch { throw error(); }
    if (url.protocol !== page.protocol || url.origin !== page.origin || url.username || url.password || url.search || url.hash
        || !path.startsWith(decodeURIComponent(articles.pathname)) || !path.endsWith('.html')
        || path.includes('\\') || path.split('/').some(part => part === '.' || part === '..')) throw error();
  }
  if (Object.values(value.anchors).some(key => !Object.hasOwn(value.entries, key))) throw error();
  return value;
}
