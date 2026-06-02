# DiskAnalyzer

<p align="center">
  <strong>Beautiful • Fast • Native macOS disk usage analyzer</strong><br>
  Interactive sunburst visualization • Instant drill-down • Professional insights
</p>

<p align="center">
  <a href="https://github.com/dongitran/diskanalyzer/releases/latest">
    <img src="https://img.shields.io/github/v/release/dongitran/diskanalyzer?label=Download&style=for-the-badge&color=0A84FF" alt="Download latest release">
  </a>
  <a href="https://github.com/dongitran/diskanalyzer/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/dongitran/diskanalyzer/ci.yml?label=CI&style=for-the-badge" alt="CI Status">
  </a>
  <a href="https://github.com/dongitran/diskanalyzer/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/dongitran/diskanalyzer?style=for-the-badge" alt="License">
  </a>
</p>

<p align="center">
  <a href="https://github.com/dongitran/diskanalyzer/releases/latest">
    <img alt="macOS" src="https://img.shields.io/badge/macOS-Apple%20Silicon%20%7C%20Intel-000000?style=for-the-badge&logo=apple&logoColor=white">
  </a>
</p>

---

**DiskAnalyzer** is a fast, beautiful, and truly native-feeling macOS app to visualize where your disk space has gone. 

Inspired by the best (DaisyDisk, GrandPerspective, Disk Inventory X) but built with modern tools for speed and clarity: **Tauri + Rust** for the engine and **Svelte + D3** for buttery-smooth interactive sunburst charts.

## ✨ Highlights

- **Stunning Sunburst Visualization** — Hierarchical pie/rings that feel natural to explore. Click any segment to dive deeper.
- **Blazing Fast Scanner** — Powered by Rust + parallel directory walking. Handles millions of files without freezing.
- **Start from anywhere** — Root `/`, your Data volume, `~/Users`, external drives, or any folder.
- **Live Progress** — See exactly what’s being scanned in real time.
- **Smart Pruning** — Automatically groups tiny files so the view stays clean and useful.
- **Native macOS Experience** — Proper permissions flow, Reveal in Finder, clean dark UI that matches the system.
- **Releases with one-click DMG** — Every build on `main` automatically creates a GitHub Release with a ready-to-install `.dmg`.

## 📥 Download & Install (Recommended)

1. Go to **[Releases](https://github.com/dongitran/diskanalyzer/releases/latest)**
2. Download the latest `DiskAnalyzer-*.dmg` (Apple Silicon builds are primary)
3. Open the DMG → drag **DiskAnalyzer** to **Applications**
4. First launch: **Right-click** the app → **Open** (Gatekeeper)
5. For complete scans from root: grant **Full Disk Access** when asked  
   (System Settings → Privacy & Security → Full Disk Access)

> The app will guide you with clear Vietnamese + English instructions.

## 🖼️ Screenshots

**Main view with live sunburst + volumes**

(Imagine a beautiful concentric ring chart here with large colorful segments for Users, Applications, Library, System, etc.)

**Drill-down experience**

Clicking a segment smoothly focuses the visualization on that subtree while the sidebar updates with the largest children.

**Progress during a full root scan**

Real-time feedback so you never wonder if it’s stuck.

## 🚀 Features

- Multiple volume detection (Data volume, external disks, etc.)
- Hierarchical sunburst with smooth interactions
- Sidebar with top consumers + detail pane
- Breadcrumb navigation + “go up” support
- Safe actions: Reveal in Finder (more coming: move to trash staging)
- Full Disk Access helper + graceful degradation
- Persistent preferences (coming soon)
- Very small app size thanks to Tauri

## 🛠️ Development

### Prerequisites

- macOS (Apple Silicon recommended)
- Rust (via rustup)
- Node.js 20+ + pnpm

### Getting started

```bash
git clone https://github.com/dongitran/diskanalyzer.git
cd diskanalyzer
pnpm install

# Run in development
pnpm tauri dev

# Build production .app + .dmg (Apple Silicon)
pnpm tauri build -- --target aarch64-apple-darwin
```

The resulting DMG will be in:

`src-tauri/target/aarch64-apple-darwin/release/bundle/dmg/`

### Quality Commands

```bash
pnpm quality          # typecheck + cspell
pnpm check            # Svelte type checking
pnpm lint
cd src-tauri && cargo check
cd src-tauri && cargo clippy -- -D warnings
```

## 🧠 Technical Notes

- **Scanner**: `jwalk` (parallel) + `sysinfo` for volumes. Future: `getattrlistbulk` / POSIX `fts` for even higher performance on APFS.
- **Size accuracy**: Reports allocated size. APFS clones and hard links are challenging (same limitation as most tools, including paid ones).
- **Memory**: Tree is pruned for visualization. Full raw data is never kept for millions of tiny files.
- **Permissions**: Uses `tauri-plugin-macos-permissions` + proper entitlements. The app is intentionally not sandboxed so it can request Full Disk Access.

## 🤝 Contributing

Contributions are very welcome!

1. Fork + create a feature branch
2. Make sure `pnpm quality` and `cargo clippy` pass
3. Open a PR with clear description + before/after if UI

Please follow the spirit of the project: **clarity and performance first**.

## 📄 License

MIT © 2026

---

**Made with care for people who actually want to understand their disk.**

If you find DiskAnalyzer useful, consider starring the repo — it helps others discover it.

<p align="center">
  <a href="https://github.com/dongitran/diskanalyzer/stargazers">⭐ Star on GitHub</a>
</p>