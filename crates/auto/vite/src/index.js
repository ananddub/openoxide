import {execFileSync} from 'node:child_process';
import {existsSync, mkdirSync, writeFileSync} from 'node:fs';
import {dirname, resolve} from 'node:path';

const VIRTUAL_ID = 'virtual:openoxide-live';
const RESOLVED_ID = `\0${VIRTUAL_ID}`;

function findManifest(start) {
  let current = start;
  while (current !== dirname(current)) {
    const candidate = resolve(current, 'Cargo.toml');
    if (existsSync(candidate)) return candidate;
    current = dirname(current);
  }
  throw new Error('[openoxide] Cargo.toml not found above Vite root');
}

function loadManifest(root, options) {
  const cargo = options.manifestPath
    ? resolve(root, options.manifestPath)
    : findManifest(root);
  const stdout = execFileSync('cargo', ['run', '--quiet', '--manifest-path', cargo, '--bin', options.manifestBin], {encoding: 'utf8'});
  return JSON.parse(stdout);
}


function groupTree(endpoints) {
  const root = {};
  for (const endpoint of endpoints) {
    const path = endpoint.groupPath ?? [endpoint.group];
    let children = root;
    let node;
    for (const segment of path) {
      node = children[segment] ??= {children: {}, endpoints: []};
      children = node.children;
    }
    node.endpoints.push(endpoint);
  }
  return root;
}

function runtimeGroupObject(node, indent = '  ') {
  const hooks = node.endpoints.map(endpoint => `${indent}${endpoint.member}: ${endpoint.hook},`);
  const children = Object.entries(node.children).map(([name, child]) =>
    `${indent}${name}: Object.freeze({\n${runtimeGroupObject(child, `${indent}  `)}\n${indent}}),`);
  return [...hooks, ...children].join('\n');
}

function groupedRuntimeSource(endpoints) {
  return Object.entries(groupTree(endpoints)).map(([name, node]) => `
export const ${name} = Object.freeze({
${runtimeGroupObject(node)}
});`).join('');
}

function runtimeSource(manifest) {
  const endpoints = manifest.endpoints.map((endpoint) => `
export const ${endpoint.hook} = createLiveHook(${JSON.stringify(endpoint)});`).join('');
  const groups = groupedRuntimeSource(manifest.endpoints);
  return `
import {useEffect, useMemo, useState} from 'react';
import {io} from 'socket.io-client';

if (typeof BigInt !== 'undefined' && !BigInt.prototype.toJSON) {
  BigInt.prototype.toJSON = function () {
    return typeof this === 'bigint' ? (this <= BigInt(Number.MAX_SAFE_INTEGER) && this >= BigInt(Number.MIN_SAFE_INTEGER) ? Number(this) : this.toString()) : this;
  };
}

function safeStringify(value) {
  return JSON.stringify(value, (_key, val) => typeof val === 'bigint' ? val.toString() : val);
}

const sockets = new Map();
const liveCache = new Map();

function accessToken() {
  try { return JSON.parse(localStorage.getItem('openoxide-auth-session') ?? 'null')?.tokens?.access_token; }
  catch { return undefined; }
}

function apiBaseUrl() {
  if (import.meta.env.VITE_API_URL) return import.meta.env.VITE_API_URL;
  if (import.meta.env.DEV) return '/api';
  if (typeof window !== 'undefined' && window.location.hostname) {
    const host = window.location.hostname === 'localhost' ? '127.0.0.1' : window.location.hostname;
    return \`\${window.location.protocol}//\${host}:4000\`;
  }
  return 'http://127.0.0.1:4000';
}
function endpointUrl(metadata, args) {
  let path = metadata.path;
  const query = new URLSearchParams();
  for (const argument of metadata.arguments ?? []) {
    const value = args[argument.index];
    if (argument.kind === 'path') {
      const values = argument.names.length === 1
        ? [value]
        : Array.isArray(value) ? value : argument.names.map(name => value?.[name]);
      argument.names.forEach((name, index) => {
        path = path.replace(\`{\${name}}\`, encodeURIComponent(String(values[index])));
      });
    } else if (argument.kind === 'query' && value != null) {
      Object.entries(value).forEach(([name, item]) => {
        if (item == null) return;
        if (Array.isArray(item)) item.forEach(entry => query.append(name, String(entry)));
        else query.set(name, String(item));
      });
    }
  }
  const suffix = query.toString();
  return \`\${apiBaseUrl()}\${path}\${suffix ? \`?\${suffix}\` : ''}\`;
}
async function refetch(metadata, args) {
  if (!metadata.path) return undefined;
  const url = endpointUrl(metadata, args);
  const token = accessToken();
  const response = await fetch(url, {headers: token ? {authorization: \`Bearer \${token}\`} : {}});
  if (!response.ok) throw new Error(\`GET \${url} failed with \${response.status}\`);
  return response.json();
}
function createLiveHook(metadata) {
  return (...args) => {
    const key = safeStringify(args);
    const fullKey = \`\${metadata.namespace}:\${metadata.endpoint}:\${key}\`;
    const endpoint = useMemo(() => ({...metadata, args}), [key]);
    const [data, setDataState] = useState(() => liveCache.get(fullKey));
    const [connected, setConnected] = useState(false);
    const [error, setError] = useState();

    const setData = (value) => {
      if (value !== undefined) {
        liveCache.set(fullKey, value);
      }
      setDataState(value);
    };

    useEffect(() => {
      const cached = liveCache.get(fullKey);
      if (cached !== undefined) {
        setDataState(cached);
      }

      const token = accessToken();
      if (!token) {
        setConnected(false);
        setError(new Error('Authentication required for live updates'));
        return;
      }
      let socketEntry = sockets.get(endpoint.namespace);
      if (socketEntry && socketEntry.token !== token) {
        socketEntry.socket.disconnect();
        sockets.delete(endpoint.namespace);
        socketEntry = undefined;
      }
      if (!socketEntry) {
        const socket = io(endpoint.namespace, {
          path: '/socket.io',
          auth: cb => cb({token: accessToken()}),
        });
        socketEntry = {socket, token};
        sockets.set(endpoint.namespace, socketEntry);
      }
      const socket = socketEntry.socket;
      const subscription = {endpoint: endpoint.endpoint, args};
      const subscribe = () => { setConnected(true); socket.emit('live:subscribe', subscription); };
      const disconnect = () => setConnected(false);
      const update = (message) => {
        if (message.endpoint === endpoint.endpoint && safeStringify(message.args) === key) setData(message.data);
      };
      const invalidate = (message) => {
        if (message.endpoint !== endpoint.endpoint || safeStringify(message.args) !== key || !metadata.path) return;
        refetch(metadata, args).then(value => { setData(value); setError(undefined); }).catch(setError);
      };
      socket.on('connect', subscribe); socket.on('disconnect', disconnect); socket.on('live:update', update); socket.on('live:invalidate', invalidate);
      if (socket.connected) subscribe();
      if (metadata.path) invalidate(subscription);
      return () => { if (socket.connected) socket.emit('live:unsubscribe', subscription); socket.off('connect', subscribe); socket.off('disconnect', disconnect); socket.off('live:update', update); socket.off('live:invalidate', invalidate); };
    }, [endpoint, key, fullKey]);
    return {data, connected, loading: data === undefined, error};
  };
}
${endpoints}
${groups}
`;
}

function isValidTypeName(name) {
  return typeof name === 'string' && /^[A-Za-z_$][A-Za-z0-9_$]*$/.test(name.trim());
}

function declarationSource(manifest) {
  const types = manifest.types.map((type) => isValidTypeName(type.name) ? `  export type ${type.name} = ${type.definition};` : '').filter(Boolean).join('\n');
  const hooks = manifest.endpoints.map((endpoint) => `  export function ${endpoint.hook}(${endpoint.parameters ?? ''}): {data: ${endpoint.result} | undefined; connected: boolean; loading: boolean; error: Error | undefined};`).join('\n');
  const groups = groupedDeclarationSource(manifest.endpoints);
  return `// @generated by @openoxide/vite; do not edit.\ndeclare module '${VIRTUAL_ID}' {\n${types}\n${hooks}\n${groups}\n}\n`;
}

export function generateLiveDeclarations(options = {}) {
  const root = resolve(options.root ?? process.cwd());
  const resolvedOptions = {
    manifestPath: options.manifestPath,
    manifestBin: options.manifestBin ?? 'openoxide-live-manifest',
    declarations: options.declarations ?? 'src/openoxide-live.generated.d.ts',
  };
  const manifest = loadManifest(root, resolvedOptions);
  const declarations = resolve(root, resolvedOptions.declarations);
  mkdirSync(dirname(declarations), {recursive: true});
  writeFileSync(declarations, declarationSource(manifest));
  return {declarations, endpoints: manifest.endpoints.length};
}

function declarationGroupObject(node, indent = '    ') {
  const hooks = node.endpoints.map(endpoint => `${indent}${endpoint.member}: typeof ${endpoint.hook};`);
  const children = Object.entries(node.children).map(([name, child]) =>
    `${indent}${name}: {\n${declarationGroupObject(child, `${indent}  `)}\n${indent}};`);
  return [...hooks, ...children].join('\n');
}

function groupedDeclarationSource(endpoints) {
  return Object.entries(groupTree(endpoints)).map(([name, node]) =>
    `  export const ${name}: {\n${declarationGroupObject(node)}\n  };`).join('\n');
}

export function openoxide(options = {}) {
  const resolvedOptions = {
    manifestPath: options.manifestPath,
    manifestBin: options.manifestBin ?? 'openoxide-live-manifest',
    declarations: options.declarations ?? 'src/openoxide-live.generated.d.ts',
  };
  let source;
  return {
    name: 'openoxide-live',
    enforce: 'pre',
    configResolved(config) {
      const manifest = loadManifest(config.root, resolvedOptions);
      source = runtimeSource(manifest);
      const declarations = resolve(config.root, resolvedOptions.declarations);
      mkdirSync(dirname(declarations), {recursive: true});
      writeFileSync(declarations, declarationSource(manifest));
    },
    resolveId(id) { if (id === VIRTUAL_ID) return RESOLVED_ID; },
    load(id) { if (id === RESOLVED_ID) return source; },
  };
}
