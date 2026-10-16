import { readFileSync } from 'node:fs';
import { validateGraphSceneData } from '../docs/website/assets/graph-scene-data.mts';
import { validateDetailsIndexData } from '../docs/website/assets/graph-details-data.mts';
import { validatePathGraphData } from '../docs/website/assets/path-graph-data.mts';

/** @type {unknown} */
const value = JSON.parse(readFileSync(0, 'utf8'));
switch (process.argv[2]) {
  case 'graph-scene': validateGraphSceneData(value); break;
  case 'path-graph': validatePathGraphData(value); break;
  case 'graph-details': validateDetailsIndexData(value, new URL(process.argv[3]), new URL(process.argv[4])); break;
  default: throw new Error('Usage: node scripts/validate_website_data.mjs graph-scene | path-graph | graph-details PAGE_URL ARTICLES_URL < DATA');
}
