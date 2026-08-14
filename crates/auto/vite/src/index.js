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
  return JSON.stringify(value, (_key, val) => typeof val === 'bigint'
    ? (val <= BigInt(Number.MAX_SAFE_INTEGER) && val >= BigInt(Number.MIN_SAFE_INTEGER) ? Number(val) : val.toString())
    : val);
}

const sockets = new Map();
const liveCache = new Map();
const liveSubscriptions = new Map();
const liveRequests = new Map();
const liveListeners = new Map();
let liveRefreshPromise;

function publishLiveValue(key, value) {
  if (value === undefined) return;
  liveCache.set(key, value);
  const listeners = liveListeners.get(key) ?? [];
  console.debug('[openoxide-live] applying value', key, {listeners: listeners.size, items: Array.isArray(value) ? value.length : undefined});
  for (const listener of listeners) listener(value);
}

function queueLiveRefetch(key, request, onValue, onError) {
  let state = liveRequests.get(key);
  if (!state) {
    state = {running: false, pending: false};
    liveRequests.set(key, state);
  }
  state.pending = true;
  if (state.running) return;
  state.running = true;
  void (async () => {
    try {
      while (state.pending) {
        state.pending = false;
        try { onValue(await request()); }
        catch (error) { onError(error); }
      }
    } finally {
      state.running = false;
      if (!state.pending) liveRequests.delete(key);
    }
  })();
}

function accessToken() {
  try { return JSON.parse(localStorage.getItem('openoxide-auth-session') ?? 'null')?.tokens?.access_token; }
  catch { return undefined; }
}
async function refreshAccessToken() {
  if (liveRefreshPromise) return liveRefreshPromise;
  liveRefreshPromise = (async () => {
    try {
      const session = JSON.parse(localStorage.getItem('openoxide-auth-session') ?? 'null');
      const refreshToken = session?.tokens?.refresh_token;
      if (!refreshToken) return undefined;
      const response = await fetch(\`\${apiBaseUrl()}/auth/refresh\`, {
        method: 'POST',
        headers: {'content-type': 'application/json'},
        body: JSON.stringify({refresh_token: refreshToken}),
      });
      if (!response.ok) return undefined;
      const nextSession = await response.json();
      localStorage.setItem('openoxide-auth-session', JSON.stringify(nextSession));
      return nextSession?.tokens?.access_token;
    } catch {
      return undefined;
    } finally {
      liveRefreshPromise = undefined;
    }
  })();
  return liveRefreshPromise;
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
function socketBaseUrl() {
  if (import.meta.env.VITE_SOCKET_URL) return import.meta.env.VITE_SOCKET_URL;
  // Avoid Vite proxy failures for Socket.IO's polling/upgrade transport.
  // Production keeps a relative URL so the deployed reverse proxy remains in control.
  if (import.meta.env.DEV) return 'http://127.0.0.1:4000';
  return '';
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
  const request = token => fetch(url, {
    cache: 'no-store',
    headers: token ? {authorization: \`Bearer \${token}\`} : {},
  });
  const token = accessToken();
  let response = await request(token);
  if (response.status === 401) {
    const latestToken = accessToken();
    if (latestToken && latestToken !== token) response = await request(latestToken);
    if (response.status === 401) {
      const refreshedToken = await refreshAccessToken();
      if (refreshedToken) response = await request(refreshedToken);
    }
  }
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
      publishLiveValue(fullKey, value);
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
      let listeners = liveListeners.get(fullKey);
      if (!listeners) {
        listeners = new Set();
        liveListeners.set(fullKey, listeners);
      }
      listeners.add(setDataState);
      let socketEntry = sockets.get(endpoint.namespace);
      if (!socketEntry) {
        const socket = io(\`\${socketBaseUrl()}\${endpoint.namespace}\`, {
          path: '/socket.io',
          transports: ['websocket', 'polling'],
          tryAllTransports: true,
          upgrade: false,
          auth: cb => cb({token: accessToken()}),
          reconnection: true,
          reconnectionAttempts: Infinity,
          reconnectionDelay: 500,
          reconnectionDelayMax: 5000,
        });
        socketEntry = {socket, ready: false, refreshedForToken: undefined};
        const entry = socketEntry;
        socket.on('connect', () => { entry.ready = false; });
        socket.on('socket:ready', () => {
          entry.ready = true;
          for (const active of liveSubscriptions.values()) {
            if (active.namespace === endpoint.namespace) socket.emit('live:subscribe', active.subscription);
          }
        });
        socket.on('disconnect', reason => {
          entry.ready = false;
          if (reason === 'io server disconnect') socket.connect();
        });
        socket.on('connect_error', async error => {
          entry.ready = false;
          console.error('[openoxide-live] socket connection failed', endpoint.namespace, error);
          const failedToken = accessToken();
          if (failedToken && entry.refreshedForToken !== failedToken) {
            entry.refreshedForToken = failedToken;
            await refreshAccessToken();
          }
          if (!socket.connected) socket.connect();
        });
        const recover = () => {
          if (!socket.connected) socket.connect();
        };
        window.addEventListener('online', recover);
        document.addEventListener('visibilitychange', () => {
          if (document.visibilityState === 'visible') recover();
        });
        sockets.set(endpoint.namespace, socketEntry);
      }
      const socket = socketEntry.socket;
      const subscription = {endpoint: endpoint.endpoint, args};
      const subscribe = () => {
        let active = liveSubscriptions.get(fullKey);
        if (active) {
          active.count++;
        } else {
          active = {namespace: endpoint.namespace, subscription, count: 1};
          liveSubscriptions.set(fullKey, active);
          if (socketEntry.ready) socket.emit('live:subscribe', subscription);
        }
        setConnected(socketEntry.ready);
      };
      const disconnect = () => setConnected(false);
      const ready = () => setConnected(false);
      const subscribed = (message) => {
        if (message.endpoint === endpoint.endpoint && safeStringify(message.args) === key) {
          setConnected(true);
          setError(undefined);
          if (metadata.path) {
            queueLiveRefetch(fullKey, () => refetch(metadata, args), value => {
              console.debug('[openoxide-live] resubscribed and hydrated', fullKey, {items: Array.isArray(value) ? value.length : undefined});
              publishLiveValue(fullKey, value);
            }, cause => {
              console.error('[openoxide-live] resubscribe hydration failed', fullKey, cause);
              setError(cause);
            });
          }
        }
      };
      const update = (message) => {
        if (message.endpoint === endpoint.endpoint && safeStringify(message.args) === key) {
          console.debug('[openoxide-live] received update', fullKey, {items: Array.isArray(message.data) ? message.data.length : undefined});
          setData(message.data);
        }
      };
      const invalidate = (message) => {
        if (message.endpoint !== endpoint.endpoint || safeStringify(message.args) !== key || !metadata.path) return;
        console.debug('[openoxide-live] invalidated', fullKey);
        queueLiveRefetch(fullKey, () => refetch(metadata, args), value => {
          console.debug('[openoxide-live] refetched', fullKey, {items: Array.isArray(value) ? value.length : undefined});
          publishLiveValue(fullKey, value);
          setError(undefined);
        }, error => {
          console.error('[openoxide-live] invalidation refetch failed', fullKey, error);
          setError(error);
        });
      };
      socket.on('socket:ready', ready); socket.on('disconnect', disconnect); socket.on('live:subscribed', subscribed); socket.on('live:update', update); socket.on('live:invalidate', invalidate);
      subscribe();
      if (metadata.path) invalidate(subscription);
      return () => {
        const listeners = liveListeners.get(fullKey);
        listeners?.delete(setDataState);
        if (listeners?.size === 0) liveListeners.delete(fullKey);
        const active = liveSubscriptions.get(fullKey);
        if (active && --active.count === 0) {
          liveSubscriptions.delete(fullKey);
          if (socketEntry.ready) socket.emit('live:unsubscribe', subscription);
        }
        socket.off('socket:ready', ready); socket.off('disconnect', disconnect); socket.off('live:subscribed', subscribed); socket.off('live:update', update); socket.off('live:invalidate', invalidate);
      };
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
