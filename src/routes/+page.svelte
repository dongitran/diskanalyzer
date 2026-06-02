<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { DiskInfo, ScanProgress, DiskNode } from '$lib/types';

  // App state with Svelte 5 runes
  let volumes = $state<DiskInfo[]>([]);
  let currentScanPath = $state<string>('');
  let progress = $state<ScanProgress | null>(null);
  let scanResult = $state<DiskNode | null>(null);
  let selectedNode = $state<DiskNode | null>(null);
  let isScanning = $state(false);
  let error = $state<string | null>(null);

  // Placeholder for sunburst - will be replaced with real D3 component
  let showSunburstDemo = $state(false);

  interface GreetResponse {
    message: string;
  }

  async function loadVolumes() {
    try {
      // Will be replaced by real Rust command
      const result = await invoke<DiskInfo[]>('get_volumes');
      volumes = result;
      error = null;
    } catch (e) {
      error = `Failed to load volumes: ${e}`;
      // Fallback demo data
      volumes = [
        { name: 'Macintosh HD (Data)', mount_point: '/System/Volumes/Data', total: 500_000_000_000, available: 120_000_000_000 },
        { name: 'Macintosh HD', mount_point: '/', total: 500_000_000_000, available: 120_000_000_000 }
      ];
    }
  }

  async function startScan(path: string) {
    if (isScanning) return;
    currentScanPath = path;
    isScanning = true;
    progress = { current_path: path, files_scanned: 0, bytes_scanned: 0, percent: 0 };
    scanResult = null;
    selectedNode = null;
    error = null;

    try {
      // Real implementation will stream progress via events or long command
      const result = await invoke<DiskNode>('scan_directory', { path, maxDepth: 6 });
      scanResult = result;
      selectedNode = result;
      // Simulate some progress for demo
      progress = { current_path: 'Scan complete', files_scanned: 124567, bytes_scanned: result.size, percent: 100 };
    } catch (e: any) {
      error = `Scan failed: ${e}`;
    } finally {
      isScanning = false;
    }
  }

  function selectNode(node: DiskNode) {
    selectedNode = node;
  }

  function formatSize(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
  }

  // Demo data for sunburst when no real scan
  function loadDemoData() {
    showSunburstDemo = true;
    scanResult = {
      name: '/',
      path: '/',
      size: 420_000_000_000,
      is_dir: true,
      children: [
        { name: 'Users', path: '/Users', size: 180_000_000_000, is_dir: true, children: [] },
        { name: 'Applications', path: '/Applications', size: 95_000_000_000, is_dir: true, children: [] },
        { name: 'Library', path: '/Library', size: 65_000_000_000, is_dir: true, children: [] },
        { name: 'System', path: '/System', size: 45_000_000_000, is_dir: true, children: [] },
        { name: 'private', path: '/private', size: 35_000_000_000, is_dir: true, children: [] }
      ]
    };
    selectedNode = scanResult;
  }

  // Load on mount
  $effect(() => {
    loadVolumes();
  });
</script>

<main class="app">
  <header class="topbar">
    <div class="brand">
      <span class="logo">📀</span>
      <h1>Disk Analyzer</h1>
      <span class="version">v0.1.0</span>
    </div>
    <div class="actions">
      <button class="btn secondary" on:click={loadVolumes} disabled={isScanning}>Refresh Volumes</button>
      <button class="btn" on:click={loadDemoData} disabled={isScanning}>Load Demo Data</button>
    </div>
  </header>

  {#if error}
    <div class="banner error">{error}</div>
  {/if}

  <div class="main-layout">
    <!-- Volumes / Start Scan -->
    <aside class="sidebar">
      <h2>Volumes</h2>
      {#if volumes.length === 0}
        <p class="muted">Loading...</p>
      {:else}
        {#each volumes as vol}
          <button 
            class="vol-item" 
            class:active={currentScanPath === vol.mount_point}
            on:click={() => startScan(vol.mount_point)}
            disabled={isScanning}
          >
            <div class="vol-name">{vol.name}</div>
            <div class="vol-stats">
              {formatSize(vol.available)} free / {formatSize(vol.total)}
            </div>
          </button>
        {/each}
      {/if}

      <div class="section">
        <h2>Quick Actions</h2>
        <button class="btn w-full" on:click={() => startScan('/Users')} disabled={isScanning}>
          Scan ~/Users (recommended first)
        </button>
        <p class="hint">Full / scan requires <strong>Full Disk Access</strong> permission.</p>
      </div>

      {#if progress}
        <div class="progress-box">
          <div class="progress-label">
            {isScanning ? 'Scanning...' : 'Done'} — {progress.files_scanned.toLocaleString()} files
          </div>
          <div class="progress-bar">
            <div class="bar" style="width: {progress.percent}%"></div>
          </div>
          <div class="progress-path">{progress.current_path}</div>
          <div class="progress-size">{formatSize(progress.bytes_scanned)}</div>
        </div>
      {/if}
    </aside>

    <!-- Visualization Area -->
    <section class="viz-area">
      <div class="viz-header">
        <div class="breadcrumb">
          {#if scanResult}
            <span class="crumb" on:click={() => selectNode(scanResult!)}>{scanResult.name}</span>
            {#if selectedNode && selectedNode.path !== scanResult.path}
              <span class="sep">›</span>
              <span class="crumb active">{selectedNode.name}</span>
            {/if}
          {:else}
            <span class="muted">Select a volume or load demo to begin</span>
          {/if}
        </div>
        <div class="viz-controls">
          <button class="btn small" on:click={() => showSunburstDemo = !showSunburstDemo}>
            {showSunburstDemo ? 'Hide' : 'Show'} Sunburst Demo
          </button>
        </div>
      </div>

      <div class="viz-container">
        {#if scanResult || showSunburstDemo}
          <div class="sunburst-placeholder">
            <div class="sunburst-demo">
              <!-- Real D3 Sunburst will be implemented here -->
              <div class="rings">
                {#if selectedNode && selectedNode.children && selectedNode.children.length > 0}
                  {#each selectedNode.children as child, i}
                    <div 
                      class="ring-segment" 
                      style="background: hsl({(i * 47) % 360}, 70%, 55%); width: {Math.max(60, Math.min(180, child.size / 2_000_000_000 + 40))}px; height: {Math.max(60, Math.min(180, child.size / 2_000_000_000 + 40))}px;"
                      on:click={() => selectNode(child)}
                      title="{child.name}: {formatSize(child.size)}"
                    >
                      <span class="seg-label">{child.name}</span>
                    </div>
                  {/each}
                {:else}
                  <div class="center">
                    <div class="center-circle" on:click={() => { if (scanResult) selectNode(scanResult); }}>
                      <strong>{scanResult ? scanResult.name : 'Root'}</strong><br>
                      <span>{scanResult ? formatSize(scanResult.size) : '—'}</span>
                    </div>
                  </div>
                {/if}
              </div>
              <p class="viz-note">Interactive D3 sunburst coming in full implementation. Click segments (demo only).</p>
            </div>
          </div>
        {:else}
          <div class="empty-state">
            <div class="icon">🗂️</div>
            <h3>No scan yet</h3>
            <p>Choose a volume on the left or load demo data to visualize disk usage with sunburst chart.</p>
            <button class="btn primary" on:click={loadDemoData}>Load Demo Sunburst</button>
          </div>
        {/if}
      </div>
    </section>

    <!-- Details Sidebar -->
    <aside class="details">
      <h2>Details</h2>
      {#if selectedNode}
        <div class="detail-card">
          <div class="detail-name">{selectedNode.name}</div>
          <div class="detail-size big">{formatSize(selectedNode.size)}</div>
          <div class="detail-path">{selectedNode.path}</div>
          <div class="detail-meta">
            {selectedNode.is_dir ? 'Folder' : 'File'} • {selectedNode.children?.length ?? 0} children
          </div>
        </div>

        <div class="actions">
          <button class="btn" on:click={() => alert('Reveal in Finder (Rust impl)')}>Reveal in Finder</button>
          {#if selectedNode.is_dir}
            <button class="btn" on:click={() => startScan(selectedNode!.path)}>Scan this folder deeper</button>
          {/if}
        </div>

        {#if selectedNode.children && selectedNode.children.length > 0}
          <h3 class="sub">Largest children</h3>
          <ul class="child-list">
            {#each selectedNode.children.sort((a,b) => b.size - a.size).slice(0, 8) as child}
              <li on:click={() => selectNode(child)}>
                <span class="cname">{child.name}</span>
                <span class="csize">{formatSize(child.size)}</span>
              </li>
            {/each}
          </ul>
        {/if}
      {:else}
        <p class="muted">Select a segment or folder to see details.</p>
      {/if}

      <div class="permissions-note">
        <strong>Permissions:</strong> For complete scan from / you must grant <strong>Full Disk Access</strong> in System Settings → Privacy & Security.
      </div>
    </aside>
  </div>

  <footer class="statusbar">
    <span>Professional macOS disk usage visualizer • Sunburst • Fast Rust scanner • {volumes.length} volumes detected</span>
  </footer>
</main>

<style>
  /* Professional macOS-inspired dark UI */
  :root {
    --bg: #1c1c1e;
    --panel: #2c2c2e;
    --accent: #0a84ff;
    --text: #f5f5f7;
    --muted: #86868b;
    --border: #3a3a3c;
  }

  .app {
    height: 100vh;
    display: flex;
    flex-direction: column;
    background: var(--bg);
    color: var(--text);
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    font-size: 13px;
  }

  .topbar {
    height: 52px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 16px;
    background: var(--panel);
    border-bottom: 1px solid var(--border);
    -webkit-app-region: drag;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .logo { font-size: 20px; }
  h1 { font-size: 15px; font-weight: 600; margin: 0; }
  .version { font-size: 10px; color: var(--muted); background: #0002; padding: 1px 5px; border-radius: 3px; }

  .btn {
    background: var(--accent);
    color: white;
    border: none;
    padding: 6px 14px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: filter 0.1s;
  }
  .btn:hover { filter: brightness(1.1); }
  .btn.secondary { background: #3a3a3c; }
  .btn.small { padding: 4px 10px; font-size: 11px; }
  .btn.w-full { width: 100%; }

  .main-layout {
    flex: 1;
    display: grid;
    grid-template-columns: 260px 1fr 280px;
    overflow: hidden;
  }

  .sidebar, .details {
    background: var(--panel);
    border-right: 1px solid var(--border);
    padding: 12px;
    overflow-y: auto;
  }
  .details { border-right: none; border-left: 1px solid var(--border); }

  .viz-area {
    display: flex;
    flex-direction: column;
    background: #111113;
    overflow: hidden;
  }

  .viz-header {
    padding: 8px 12px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid var(--border);
    background: var(--panel);
  }

  .breadcrumb { font-size: 12px; }
  .crumb { cursor: pointer; color: var(--accent); }
  .crumb:hover { text-decoration: underline; }
  .sep { margin: 0 6px; color: var(--muted); }

  .viz-container {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    padding: 20px;
  }

  .sunburst-placeholder {
    width: 100%;
    max-width: 620px;
    aspect-ratio: 1;
  }

  .sunburst-demo {
    width: 100%;
    height: 100%;
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .rings {
    position: relative;
    width: 420px;
    height: 420px;
  }

  .ring-segment {
    position: absolute;
    border-radius: 9999px;
    border: 2px solid #111;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: transform 0.2s, box-shadow 0.2s;
    box-shadow: 0 0 0 1px #0004 inset;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }
  .ring-segment:hover {
    transform: translate(-50%, -50%) scale(1.03);
    z-index: 10;
    box-shadow: 0 4px 12px #0008;
  }
  .seg-label {
    font-size: 10px;
    color: white;
    text-shadow: 0 1px 2px #0008;
    pointer-events: none;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 70%;
  }

  .center {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .center-circle {
    width: 118px;
    height: 118px;
    background: #222;
    border: 3px solid var(--accent);
    border-radius: 9999px;
    display: flex;
    align-items: center;
    justify-content: center;
    text-align: center;
    font-size: 11px;
    line-height: 1.2;
    cursor: pointer;
  }

  .viz-note {
    position: absolute;
    bottom: -28px;
    font-size: 10px;
    color: var(--muted);
    text-align: center;
    width: 100%;
  }

  h2 { font-size: 12px; font-weight: 600; margin: 12px 0 6px; text-transform: uppercase; letter-spacing: 0.5px; color: var(--muted); }
  .vol-item {
    width: 100%;
    text-align: left;
    background: #222;
    border: 1px solid var(--border);
    padding: 8px 10px;
    margin-bottom: 6px;
    border-radius: 6px;
    cursor: pointer;
  }
  .vol-item.active { border-color: var(--accent); background: #1a2a44; }
  .vol-name { font-weight: 500; }
  .vol-stats { font-size: 10px; color: var(--muted); }

  .progress-box {
    margin-top: 16px;
    background: #111;
    padding: 8px;
    border-radius: 6px;
    font-size: 11px;
  }
  .progress-bar { height: 4px; background: #333; border-radius: 2px; margin: 4px 0; overflow: hidden; }
  .bar { height: 100%; background: var(--accent); transition: width .2s; }
  .progress-path { font-family: monospace; font-size: 10px; color: var(--muted); word-break: break-all; }

  .detail-card {
    background: #111;
    padding: 10px;
    border-radius: 6px;
    margin-bottom: 10px;
  }
  .detail-name { font-weight: 600; font-size: 14px; }
  .detail-size.big { font-size: 22px; font-weight: 600; color: var(--accent); margin: 4px 0; }
  .detail-path { font-family: monospace; font-size: 10px; color: var(--muted); word-break: break-all; }

  .child-list {
    list-style: none;
    padding: 0;
    margin: 8px 0;
    font-size: 12px;
  }
  .child-list li {
    display: flex;
    justify-content: space-between;
    padding: 4px 6px;
    cursor: pointer;
    border-radius: 4px;
  }
  .child-list li:hover { background: #222; }
  .csize { color: var(--muted); font-family: monospace; }

  .hint { font-size: 10px; color: var(--muted); margin-top: 8px; line-height: 1.3; }
  .muted { color: var(--muted); }
  .banner { padding: 6px 12px; background: #3a1f1f; color: #ff8080; font-size: 12px; }

  .permissions-note {
    margin-top: 16px;
    font-size: 10px;
    padding: 8px;
    background: #222;
    border-radius: 4px;
    line-height: 1.35;
  }

  .statusbar {
    height: 28px;
    font-size: 10px;
    display: flex;
    align-items: center;
    padding: 0 12px;
    background: #111;
    border-top: 1px solid var(--border);
    color: var(--muted);
  }

  .empty-state {
    text-align: center;
    color: var(--muted);
  }
  .empty-state .icon { font-size: 42px; margin-bottom: 8px; }
</style>