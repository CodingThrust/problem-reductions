import { readFileSync } from 'node:fs';
import { validateGraphSceneData } from '../docs/website/assets/graph-scene-data.mts';

if (process.argv[2] !== 'graph-scene') throw new Error('Usage: node scripts/validate_website_data.mjs graph-scene < DATA');
/** @type {unknown} */
const value = JSON.parse(readFileSync(0, 'utf8'));
validateGraphSceneData(value);
