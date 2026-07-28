<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import {
		Globe, FileCode, Folder, FolderOpen, ChevronRight, ChevronDown,
		MousePointerClick, Lock, LockOpen, Save, Loader2, AlertTriangle
	} from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import { Button } from '$lib/components/ui/button';
	import { toastSuccess, toastError } from '$lib/toast';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	// ─── File tree ────────────────────────────────────────────────────────────────
	type TreeNode = { id: string; name: string; type: 'file' | 'dir'; children?: TreeNode[] };

	const tree: TreeNode[] = [
		{
			id: '/etc/traefik', name: 'traefik', type: 'dir', children: [
				{
					id: '/etc/traefik/dynamic', name: 'dynamic', type: 'dir', children: [
						{ id: '/etc/traefik/dynamic/http.yml',        name: 'http.yml',        type: 'file' },
						{ id: '/etc/traefik/dynamic/tls.yml',         name: 'tls.yml',         type: 'file' },
						{ id: '/etc/traefik/dynamic/middlewares.yml', name: 'middlewares.yml', type: 'file' },
					]
				},
				{ id: '/etc/traefik/traefik.yml', name: 'traefik.yml', type: 'file' },
			]
		}
	];

	const fileContents: Record<string, string> = {
		'/etc/traefik/dynamic/http.yml': `# HTTP Routers and Services
http:
  routers:
    my-app:
      rule: "Host(\`app.example.com\`)"
      service: my-app-service
      entryPoints:
        - websecure
      tls: {}
      middlewares:
        - redirect-to-https

    api-router:
      rule: "Host(\`api.example.com\`) && PathPrefix(\`/v1\`)"
      service: api-service
      entryPoints:
        - websecure

  services:
    my-app-service:
      loadBalancer:
        servers:
          - url: "http://app-container:3000"
        healthCheck:
          path: /health
          interval: "30s"
          timeout: "5s"

    api-service:
      loadBalancer:
        servers:
          - url: "http://api-container:8080"
`,
		'/etc/traefik/dynamic/tls.yml': `# TLS Configuration
tls:
  options:
    default:
      minVersion: VersionTLS12
      sniStrict: true
      cipherSuites:
        - TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384
        - TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384
        - TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305
        - TLS_AES_128_GCM_SHA256
        - TLS_AES_256_GCM_SHA384

    modern:
      minVersion: VersionTLS13

  stores:
    default:
      defaultCertificate:
        certFile: /etc/traefik/certs/cert.pem
        keyFile: /etc/traefik/certs/key.pem
`,
		'/etc/traefik/dynamic/middlewares.yml': `# Middleware Definitions
http:
  middlewares:
    redirect-to-https:
      redirectScheme:
        scheme: https
        permanent: true

    rate-limit:
      rateLimit:
        average: 100
        burst: 50
        period: 1m

    basic-auth:
      basicAuth:
        users:
          - "admin:$apr1$H6uskkkW$IgXLP6ewTrSuBkTrqE8wj/"

    compress:
      compress:
        excludedContentTypes:
          - text/event-stream

    secure-headers:
      headers:
        sslRedirect: true
        stsSeconds: 31536000
        stsIncludeSubdomains: true
        contentTypeNosniff: true
        browserXssFilter: true
        referrerPolicy: "strict-origin-when-cross-origin"
`,
		'/etc/traefik/traefik.yml': `# Traefik Static Configuration
api:
  dashboard: true
  insecure: false

log:
  level: INFO
  format: json

accessLog:
  filePath: "/var/log/traefik/access.log"
  format: json

entryPoints:
  web:
    address: ":80"
    http:
      redirections:
        entryPoint:
          to: websecure
          scheme: https

  websecure:
    address: ":443"
    http:
      tls:
        certResolver: letsencrypt

providers:
  file:
    directory: /etc/traefik/dynamic
    watch: true
  docker:
    endpoint: "unix:///var/run/docker.sock"
    exposedByDefault: false
    network: traefik

certificatesResolvers:
  letsencrypt:
    acme:
      email: admin@example.com
      storage: /etc/traefik/acme.json
      httpChallenge:
        entryPoint: web
`,
	};

	// Track mutations in memory
	const savedContents: Record<string, string> = { ...fileContents };

	// ─── Tree state ───────────────────────────────────────────────────────────────
	let expanded    = $state<Set<string>>(new Set(['/etc/traefik', '/etc/traefik/dynamic']));
	let selectedFile = $state<string | null>(null);

	function toggleDir(id: string) {
		const s = new Set(expanded);
		s.has(id) ? s.delete(id) : s.add(id);
		expanded = s;
	}

	// ─── Editor ───────────────────────────────────────────────────────────────────
	let editorEl: HTMLDivElement;
	let monacoInst: any = null;    // monaco namespace
	let editor: any    = null;     // editor instance
	let monacoReady    = $state(false);
	let locked         = $state(true);
	let saving         = $state(false);
	let skipYaml       = $state(false);
	let yamlError      = $state('');

	// API State & Handlers
	let remoteFiles = $state<Array<{ name: string; relative_path: string; size: number; is_readonly: boolean }>>([]);
	let loadingFiles = $state(false);

	async function fetchTraefikFiles() {
		loadingFiles = true;
		try {
			const res = await fetch('/api/traefik/files');
			if (res.ok) {
				remoteFiles = await res.json();
			}
		} catch (e) {
			console.error('Failed to fetch Traefik files:', e);
		} finally {
			loadingFiles = false;
		}
	}

	async function loadFileInEditor(path: string) {
		if (!editor || !monacoInst) return;
		try {
			const res = await fetch(`/api/traefik/files/content?path=${encodeURIComponent(path)}`);
			if (res.ok) {
				const data = await res.json();
				const content = data.content ?? '';
				const isReadOnly = Boolean(data.is_readonly);
				const oldModel = editor.getModel();
				const newModel = monacoInst.editor.createModel(content, 'yaml');
				editor.setModel(newModel);
				oldModel?.dispose();
				editor.updateOptions({ readOnly: isReadOnly || locked });
				locked = isReadOnly || locked;
				yamlError = '';
			} else {
				toastError('Failed to read file content');
			}
		} catch (e) {
			toastError('Network error while reading file');
		}
	}

	function selectFile(path: string) {
		selectedFile = path;
		if (monacoReady) loadFileInEditor(path);
	}

	function toggleLock() {
		locked = !locked;
		editor?.updateOptions({ readOnly: locked });
		if (!locked) editor?.focus();
	}

	function validateYaml(content: string): string {
		const lines = content.split('\n');
		for (let i = 0; i < lines.length; i++) {
			if (lines[i].includes('\t'))
				return `Line ${i + 1}: YAML does not allow tabs — use spaces`;
		}
		return '';
	}

	async function save() {
		if (!selectedFile || !editor) return;
		const content = editor.getValue();
		if (!skipYaml) {
			const err = validateYaml(content);
			if (err) { yamlError = err; return; }
		}
		yamlError = '';
		saving = true;
		try {
			const res = await fetch('/api/traefik/files/content', {
				method: 'PUT',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					path: selectedFile,
					content: content
				})
			});

			if (res.ok) {
				toastSuccess('Traefik configuration saved & applied!');
				// Query Traefik health status
				const healthRes = await fetch('/api/traefik/health');
				if (healthRes.ok) {
					const health = await healthRes.json();
					if (!health.is_healthy && health.configuration_errors?.length) {
						toastError(`Traefik warning: ${health.configuration_errors[0]}`);
					}
				}
			} else {
				const errText = await res.text();
				toastError(`Save error: ${errText || 'Failed to save configuration'}`);
			}
		} catch {
			toastError('Failed to save file');
		} finally {
			saving = false;
		}
	}

	onMount(async () => {
		const loaderModule = await import('@monaco-editor/loader');
		const loader = loaderModule.default;
		loader.config({ paths: { vs: 'https://cdn.jsdelivr.net/npm/monaco-editor@0.52.0/min/vs' } });
		monacoInst = await loader.init();

		editor = monacoInst.editor.create(editorEl, {
			value: '',
			language: 'yaml',
			theme: 'vs-dark',
			readOnly: true,
			fontSize: 13,
			fontFamily: '"Fira Code", "JetBrains Mono", monospace',
			minimap: { enabled: false },
			scrollBeyondLastLine: false,
			lineNumbers: 'on',
			wordWrap: 'on',
			automaticLayout: true,
			padding: { top: 16, bottom: 16 },
			renderLineHighlight: 'line',
			smoothScrolling: true,
			stickyScroll: { enabled: false },
		});

		monacoReady = true;

		await fetchTraefikFiles();
		if (remoteFiles.length > 0) {
			selectFile(remoteFiles[0].relative_path);
		}

		return () => editor?.dispose();
	});
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Globe class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Traefik File System</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto">
		<div class="rounded-xl border border-border bg-card overflow-hidden">

			<!-- Card header -->
			<div class="px-6 pt-5 pb-4 border-b border-border">
				<div class="flex items-center gap-2 mb-1">
					<FileCode class="w-5 h-5 text-muted-foreground" />
					<h1 class="text-xl font-bold">Traefik File System</h1>
				</div>
				<p class="text-sm text-muted-foreground">
					Manage all files and directories in
					<code class="bg-muted px-1.5 py-0.5 rounded text-xs font-mono">/etc/traefik</code>
				</p>
				<div class="flex items-start gap-2 mt-3 rounded-lg border border-yellow-500/30 bg-yellow-500/10 px-3 py-2.5 text-xs text-yellow-600 dark:text-yellow-400">
					<AlertTriangle class="w-3.5 h-3.5 shrink-0 mt-0.5" />
					<span>Adding invalid configuration to existing files can break your Traefik instance, preventing access to your applications.</span>
				</div>
			</div>

			<!-- Two-panel body -->
			<div class="flex flex-col lg:flex-row" style="min-height: 600px">

				<!-- Left: file tree -->
				<div class="lg:w-64 w-full shrink-0 border-b lg:border-b-0 lg:border-r border-border p-3">
					<p class="text-[10px] font-semibold uppercase tracking-widest text-muted-foreground/50 px-2 mb-2">Files</p>

					{#snippet renderTree(nodes: typeof tree, depth: number)}
						{#each nodes as node (node.id)}
							{#if node.type === 'dir'}
								<button
									class="flex items-center gap-1.5 w-full text-left py-1 rounded-md text-sm hover:bg-accent transition-colors text-muted-foreground"
									style="padding-left:{8 + depth * 14}px; padding-right:8px"
									onclick={() => toggleDir(node.id)}
								>
									{#if expanded.has(node.id)}
										<ChevronDown class="w-3.5 h-3.5 shrink-0 opacity-50" />
										<FolderOpen class="w-3.5 h-3.5 shrink-0 text-yellow-400" />
									{:else}
										<ChevronRight class="w-3.5 h-3.5 shrink-0 opacity-50" />
										<Folder class="w-3.5 h-3.5 shrink-0 text-yellow-400" />
									{/if}
									<span class="truncate text-xs">{node.name}</span>
								</button>
								{#if expanded.has(node.id) && node.children}
									{@render renderTree(node.children, depth + 1)}
								{/if}
							{:else}
								<button
									class="flex items-center gap-1.5 w-full text-left py-1 rounded-md text-xs font-mono transition-colors
										{selectedFile === node.id
											? 'bg-primary/10 text-primary font-semibold'
											: 'text-muted-foreground hover:bg-accent hover:text-foreground'}"
									style="padding-left:{8 + depth * 14}px; padding-right:8px"
									onclick={() => selectFile(node.id)}
								>
									<FileCode class="w-3.5 h-3.5 shrink-0 opacity-50" />
									<span class="truncate">{node.name}</span>
								</button>
							{/if}
						{/each}
					{/snippet}

					{@render renderTree(tree, 0)}
				</div>

				<!-- Right: editor panel -->
				<div class="flex-1 flex flex-col min-w-0">

					<!-- Empty state — shown when no file selected -->
					{#if !selectedFile}
						<div class="flex-1 flex flex-col items-center justify-center gap-4 py-20 text-muted-foreground">
							<div class="w-14 h-14 rounded-full bg-muted flex items-center justify-center">
								<MousePointerClick class="w-7 h-7 opacity-60" />
							</div>
							<div class="text-center">
								<p class="text-base font-medium">Select a file to edit</p>
								<p class="text-sm opacity-50 mt-1">Choose a file from the tree on the left</p>
							</div>
						</div>
					{/if}

					<!-- Editor — always in DOM so bind:this works, hidden when no file -->
					<div class="flex flex-col flex-1" class:hidden={!selectedFile}>
						<!-- Toolbar -->
						<div class="flex items-center justify-between px-4 py-2.5 border-b border-border bg-muted/20">
							<div class="min-w-0">
								<p class="text-[10px] text-muted-foreground/60 uppercase tracking-wider">Traefik config</p>
								<p class="text-sm font-mono text-muted-foreground truncate">{selectedFile ?? ''}</p>
							</div>
							<Button variant="secondary" size="sm" class="gap-1.5 h-7 text-xs shrink-0 ml-4" onclick={toggleLock}>
								{#if locked}<Lock class="w-3 h-3" /> Unlock{:else}<LockOpen class="w-3 h-3" /> Lock{/if}
							</Button>
						</div>

						<!-- Monaco container — always rendered -->
						<div class="relative" style="height:460px">
							{#if !monacoReady}
								<div class="absolute inset-0 z-10 flex items-center justify-center gap-3 text-muted-foreground bg-[#1e1e1e]">
									<Loader2 class="w-5 h-5 animate-spin" />
									<span class="text-sm">Loading editor…</span>
								</div>
							{/if}
							<div bind:this={editorEl} class="w-full h-full"></div>
						</div>

						<!-- Footer -->
						<div class="border-t border-border px-4 py-3 flex flex-col gap-3 bg-card">
							<label class="flex items-start gap-2.5 cursor-pointer select-none">
								<input type="checkbox" class="mt-0.5" bind:checked={skipYaml} />
								<div>
									<p class="text-sm font-medium">Skip YAML validation <span class="text-muted-foreground font-normal">(for Go templating)</span></p>
									<p class="text-sm text-muted-foreground mt-0.5">
										Traefik supports Go templates like <code class="bg-muted px-1 py-0.5 rounded text-[10px] font-mono">{'{{range}}'}</code> which fail standard YAML validation.
									</p>
								</div>
							</label>
							{#if yamlError}
								<div class="flex items-center gap-2 rounded-md bg-destructive/10 border border-destructive/20 px-3 py-2 text-xs text-destructive">
									<AlertTriangle class="w-3.5 h-3.5 shrink-0" />{yamlError}
								</div>
							{/if}
							<div class="flex justify-end">
								<Button size="sm" class="gap-1.5" onclick={save} disabled={locked || saving}>
									{#if saving}<Loader2 class="w-3.5 h-3.5 animate-spin" /> Saving…
									{:else}<Save class="w-3.5 h-3.5" /> Save{/if}
								</Button>
							</div>
						</div>
					</div>

				</div>
			</div>
		</div>
	</main>
</PageLayout>
