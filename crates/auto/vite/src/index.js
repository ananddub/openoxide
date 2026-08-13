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

function runtimeSource(manifest) {
  const endpoints = manifest.endpoints.map((endpoint) => `
export const ${endpoint.hook} = createLiveHook(${JSON.stringify(endpoint)});`).join('');
  return `
import {useEffect, useMemo, useState} from 'react';
import {io} from 'socket.io-client';
const sockets = new Map();
function accessToken() {
  try { return JSON.parse(localStorage.getItem('openoxide-auth-session') ?? 'null')?.tokens?.access_token; }
  catch { return undefined; }
}
function apiBaseUrl() {
  if (import.meta.env.VITE_API_URL) return import.meta.env.VITE_API_URL;
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
    const key = JSON.stringify(args);
    const endpoint = useMemo(() => ({...metadata, args}), [key]);
    const [data, setData] = useState();
    const [connected, setConnected] = useState(false);
    const [error, setError] = useState();
    useEffect(() => {
      let socket = sockets.get(endpoint.namespace);
      if (!socket) { socket = io(endpoint.namespace, {path: '/socket.io', auth: cb => cb({token: accessToken()})}); sockets.set(endpoint.namespace, socket); }
      const subscription = {endpoint: endpoint.endpoint, args};
      const subscribe = () => { setConnected(true); socket.emit('live:subscribe', subscription); };
      const disconnect = () => setConnected(false);
      const update = (message) => {
        if (message.endpoint === endpoint.endpoint && JSON.stringify(message.args) === key) setData(message.data);
      };
      const invalidate = (message) => {
        if (message.endpoint !== endpoint.endpoint || JSON.stringify(message.args) !== key || !metadata.path) return;
        refetch(metadata, args).then(value => { setData(value); setError(undefined); }).catch(setError);
      };
      socket.on('connect', subscribe); socket.on('disconnect', disconnect); socket.on('live:update', update); socket.on('live:invalidate', invalidate);
      if (socket.connected) subscribe();
      if (metadata.path) invalidate(subscription);
      return () => { if (socket.connected) socket.emit('live:unsubscribe', subscription); socket.off('connect', subscribe); socket.off('disconnect', disconnect); socket.off('live:update', update); socket.off('live:invalidate', invalidate); };
    }, [endpoint, key]);
    return {data, connected, loading: data === undefined, error};
  };
}
${endpoints}
`;
}

function declarationSource(manifest) {
  const types = manifest.types.map((type) => `  export type ${type.name} = ${type.definition};`).join('\n');
  const hooks = manifest.endpoints.map((endpoint) => `  export function ${endpoint.hook}(${endpoint.parameters ?? ''}): {data: ${endpoint.result} | undefined; connected: boolean; loading: boolean; error: Error | undefined};`).join('\n');
  return `// @generated by @openoxide/vite; do not edit.\ndeclare module '${VIRTUAL_ID}' {\n${types}\n${hooks}\n}\n`;
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
