import { validateNativeExportData } from '../docs/website/assets/native-exports-data.mts';
import { readFileSync } from 'node:fs';
import { validateAtlasSummary, validateAtlasRecord } from '../docs/website/assets/atlas-data.mts';
import { validateGraphSceneData } from '../docs/website/assets/graph-scene-data.mts';
import { validateDetailsIndexData } from '../docs/website/assets/graph-details-data.mts';
import { validatePathGraphData } from '../docs/website/assets/path-graph-data.mts';

/** @type {unknown} */
const value = JSON.parse(readFileSync(0, 'utf8'));
switch (process.argv[2]) {
  case 'native-exports': validateNativeExportData(value); break;
  case 'atlas-summary': validateAtlasSummary(value); break;
  case 'atlas-records':
    if (!Array.isArray(value)) throw new Error('Expected an array of Atlas records');
    for (const record of value) validateAtlasRecord(record);
    break;
  case 'graph-scene': validateGraphSceneData(value); break;
  case 'path-graph': validatePathGraphData(value); break;
  case 'graph-details': validateDetailsIndexData(value, new URL(process.argv[3]), new URL(process.argv[4])); break;
  default: throw new Error('Usage: node scripts/validate_website_data.mjs native-exports | atlas-summary | atlas-records | graph-scene | path-graph | graph-details PAGE_URL ARTICLES_URL < DATA');
}
