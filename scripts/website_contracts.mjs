import {readFileSync} from 'node:fs';
import Ajv2020 from 'ajv/dist/2020.js';
import {validateDetailURL} from '../docs/website/src/lib/graph-details-data.mts';
import {nodeKey} from '../docs/website/src/lib/variant.mts';

const root = new URL('../docs/website/', import.meta.url);
const wasmSchemas = JSON.parse(readFileSync(new URL('src/generated/wasm-schemas.json', root), 'utf8'));
const ajv = new Ajv2020({strict: true});
function validator(name) {
  return ajv.compile(wasmSchemas[name] ?? JSON.parse(readFileSync(new URL(name + '.schema.json', root), 'utf8')));
}

const validateAtlas = validator('atlas-data');
const validateGraphScene = validator('graph-scene');
const validateGraphDetails = validator('graph-details');
const validatePathGraph = validator('path-graph');
const validateNativeExports = validator('native-exports');
const validateWasmPaths = validator('wasm-paths');
const validateWasmComparison = validator('wasm-comparison');
export const validateWasmPathRequest = validator('wasm-path-request');
export const validateWasmComparisonRequest = validator('wasm-comparison-request');

export function validateGraphReferences(data, error) {
    if (new Set(data.nodes.map(nodeKey)).size !== data.nodes.length)
        throw error?.() ?? new Error('Duplicate graph variant.');
    if (data.edges.some(edge => !data.nodes[edge.source] || !data.nodes[edge.target]))
        throw error?.() ?? new Error('Graph edge references an unknown variant.');
}

const error = () => new Error('Invalid atlas data. Reload the current release and try again.');
export function validateAtlasSummary(value) {
    if (!validateAtlas(value) || value.kind !== 'summary')
        throw error();
    validateGraphReferences(value, error);
    const families = new Set(value.nodes.map(node => node.name));
    if (value.schemas.length !== families.size || new Set(value.schemas.map(schema => schema.name)).size !== families.size
        || value.schemas.some(schema => !families.has(schema.name)))
        throw error();
    return value;
}
export function validateAtlasRecord(value, family) {
    if (!validateAtlas(value) || value.kind !== 'record')
        throw error();
    validateGraphReferences(value, error);
    const schema = value.schemas[0];
    if ((family !== undefined && schema.name !== family) || !value.nodes.some(node => node.name === schema.name)
        || value.nodes.some(node => node.name === schema.name && node.category !== schema.category)
        || value.edges.some(edge => value.nodes[edge.source].name !== schema.name && value.nodes[edge.target].name !== schema.name))
        throw error();
    return value;
}

export function validateGraphSceneData(value) {
    if (!validateGraphScene(value))
        throw new Error('Invalid graph scene contract. Reload the current release.');
    const families = new Set(value.nodes.map(node => node.name));
    const defaults = new Map();
    for (const node of value.nodes)
        if (node.default)
            defaults.set(node.name, (defaults.get(node.name) ?? 0) + 1);
    if ([...families].some(name => defaults.get(name) !== 1))
        throw new Error('Each graph family needs exactly one default.');
    validateGraphReferences(value);
    if (Object.keys(value.aliases).some(name => !families.has(name)))
        throw new Error('Graph alias references an unknown family.');
    if (Object.keys(value.layout).length !== families.size || Object.keys(value.layout).some(name => !families.has(name))) {
        throw new Error('Graph layout must cover exactly the registered families.');
    }
    return value;
}

export function validatePathGraphData(value) {
    if (!validatePathGraph(value))
        throw new Error('Invalid path metadata contract. Reload the current release.');
    validatePathGraphReferences(value);
    return value;
}
export function validatePathGraphReferences(value) {
    const error = () => new Error('Invalid path metadata contract. Reload the current release.');
    validateGraphReferences(value, error);
    const defaults = new Map();
    for (const node of value.nodes)
        defaults.set(node.name, (defaults.get(node.name) ?? 0) + (node.default ? 1 : 0));
    if ([...defaults.values()].some(count => count !== 1))
        throw error();
    const edges = new Set();
    for (const edge of value.edges) {
        const target = value.nodes[edge.target];
        const key = `${edge.source}:${edge.target}`;
        if (edges.has(key))
            throw error();
        edges.add(key);
        const fields = new Set(edge.parameters.map(parameter => parameter.field));
        if (fields.size !== edge.parameters.length || [...fields].some(field => !target.parameter_names.includes(field))
            || (edge.parameter_contract_error === null && fields.size !== target.parameter_names.length))
            throw error();
    }
}

export function validateNativeExportData(value) {
    const error = () => new Error('Invalid native registry exports. Regenerate graph and problem schemas together.');
    if (!validateNativeExports(value))
        throw error();
    validatePathGraphReferences(value.graph);
    const families = new Set(value.graph.nodes.map(node => node.name));
    const schemas = new Map(value.schemas.map(schema => [schema.name, schema]));
    if (schemas.size !== value.schemas.length || schemas.size !== families.size
        || [...schemas.keys()].some(name => !families.has(name))
        || Object.keys(value.graph.aliases).some(name => !families.has(name))
        || value.graph.nodes.some(node => schemas.get(node.name)?.category !== node.category))
        throw error();
    return value;
}

function sameEndpoint(left, right) {
    const entries = Object.entries(left.variant);
    return left.name === right.name && entries.length === Object.keys(right.variant).length
        && entries.every(([key, value]) => Object.hasOwn(right.variant, key) && right.variant[key] === value);
}
export function parseWasmPathResult(text) {
    const value = JSON.parse(text);
    return validateWasmPathResultData(value);
}
export function validateWasmPathResultData(value) {
    const error = () => new Error('Invalid path result contract. Reload the current release.');
    if (!validateWasmPaths(value))
        throw error();
    if (value.truncated && !value.paths.length)
        throw error();
    for (const route of value.paths) {
        if (route.steps !== route.path.length)
            throw error();
        let previous;
        for (const step of route.path) {
            if (previous && !sameEndpoint(previous, step.from))
                throw error();
            previous = step.to;
        }
    }
    return value;
}

export function validateComparisonParameters(value, source, target) {
    const coordinates = new Set(source), fields = new Set(target);
    for (const profile of value.profiles) {
        const names = Object.keys(profile.fields);
        if (new Set(profile.source_coordinates).size !== coordinates.size
            || profile.source_coordinates.some(name => !coordinates.has(name))
            || names.length !== fields.size || names.some(name => !fields.has(name))) {
            throw new Error('Comparison parameters do not match the registered endpoints. Reload the current release.');
        }
    }
}
export function parseWasmComparison(text, pathCount) {
    const value = JSON.parse(text);
    return validateWasmComparisonData(value, pathCount);
}
export function validateWasmComparisonData(value, pathCount) {
    const error = () => new Error('Invalid comparison result contract. Reload the current release.');
    if (!Number.isSafeInteger(pathCount) || pathCount < 0 || !validateWasmComparison(value))
        throw error();
    if (value.profiles.length !== pathCount)
        throw error();
    const total = BigInt(pathCount) * (BigInt(pathCount) - 1n) / 2n;
    if (Object.values(value.counts).reduce((sum, count) => sum + BigInt(count), 0n) !== total)
        throw error();
    const front = new Set(value.front);
    if (front.size !== value.front.length || (pathCount > 0 && !front.size)
        || value.front.some(index => index > pathCount))
        throw error();
    for (const profile of value.profiles) {
        const coordinates = new Set(profile.source_coordinates);
        if (coordinates.size !== profile.source_coordinates.length)
            throw error();
        for (const field of Object.values(profile.fields)) {
            if ((field.certificate === null) !== (field.accuracy.kind === 'unavailable'))
                throw error();
            if (field.certificate !== null)
                for (const variable of field.certificate.matchAll(/[A-Za-z_]\w*/g)) {
                    if (!coordinates.has(variable[0]))
                        throw error();
                }
        }
    }
    return value;
}

export function validateDetailsIndexData(value, page, articles) {
    const error = () => new Error('Invalid documentation index. Reload the current release.');
    if (!validateGraphDetails(value))
        throw error();
    for (const entry of Object.values(value.entries))
        validateDetailURL(entry, page, articles);
    if (Object.values(value.anchors).some(key => !Object.hasOwn(value.entries, key)))
        throw error();
    return value;
}
