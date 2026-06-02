# Disk Analyzer (28-disk-analyzer)

**Professional macOS disk usage analyzer** — trực quan, nhanh, với Sunburst chart (chart tròn phân cấp) giống DaisyDisk. Scan từ root `/`, drill-down, actions an toàn.

> Dự án được nghiên cứu và triển khai chuyên nghiệp trong Personal Knowledge Base (PARA). Mục tiêu: app cài được trên macOS qua .dmg, CI build tự động, chất lượng code cao (lint, typecheck, cspell, clippy...).

## Tính năng chính (MVP + roadmap)
- Scan volumes (/, Data, external) với sysinfo + jwalk (parallel, nhanh).
- Sunburst visualization tương tác (click để zoom/drill vào folder con).
- Progress realtime, prune small files, inode hardlink cơ bản.
- Sidebar: volumes, largest items, detail panel.
- Actions: Reveal in Finder, deeper scan.
- Full Disk Access UX (check + request + hướng dẫn tiếng Việt).
- .dmg build qua GitHub Actions (artifact downloadable).

**Hạn chế hiện tại (biết và sẽ fix):**
- Size là logical + allocated (clones APFS có thể overcount — như DaisyDisk).
- Chưa dedup hardlink hoàn hảo cho mọi case.
- Chưa có treemap toggle, delete staging, smart categories (Xcode caches...).
- Signing/notarization cần secrets Apple (chưa có trong CI công khai → lần đầu mở app cần Right-click > Open).

## Cài đặt & chạy (dành cho user)

### Tải bản build (khuyến nghị)
1. Vào GitHub Actions của repo → artifact `disk-analyzer-macos-aarch64`.
2. Tải .dmg → mở, kéo vào /Applications.
3. Lần đầu: Right click app → Open (vì unsigned).

### Build từ source (dev)
Yêu cầu: macOS (Apple Silicon ưu tiên), Rust, Node 20+, pnpm.

```bash
git clone https://github.com/dongitran/28-disk-analyzer.git
cd 28-disk-analyzer
pnpm install

# Dev
pnpm tauri dev

# Build .app + .dmg (aarch64)
pnpm tauri build -- --target aarch64-apple-darwin
```

Sau build: `src-tauri/target/aarch64-apple-darwin/release/bundle/dmg/*.dmg`

## Phát triển & chất lượng code

```bash
pnpm dev
pnpm check          # typecheck Svelte/TS
pnpm lint
pnpm cspell
pnpm quality        # all above
```

**Rust:**
- `cd src-tauri && cargo check`
- `cargo clippy -- -D warnings` (CI sẽ enforce)

**CI:**
- `.github/workflows/ci.yml`: lint + type + cspell trên PR (ubuntu), build-macos (macos-latest) → upload .dmg artifact.
- Dependabot cho Cargo, npm, actions.

**Quy tắc chất lượng (nhiều rule):**
- TypeScript strict + svelte-check.
- ESLint + (sẽ mở rộng).
- CSpell (có wordlist cho tech terms VN/EN).
- Rust: fmt, clippy deny warnings.
- Pre-commit hooks (lefthook hoặc husky) khuyến khích (chưa setup trong MVP).
- Commit message: conventional hoặc "feat: ...", "fix: ...".

## Kiến trúc & research notes
- **Stack**: Tauri v2 + Svelte 5 (SvelteKit static) + Rust. Lý do: nhỏ, nhanh, viz web (D3) đẹp + interactive dễ, Rust cho scanner tối ưu.
- **Scanner**: jwalk (parallel rayon) + sysinfo cho volumes. Tối ưu sau: fts / getattrlistbulk (xem dumac, Spacie).
- **Viz**: D3 zoomable sunburst (chuẩn cho disk tools). Toggle treemap sau.
- **Permissions**: tauri-plugin-macos-permissions + entitlements + open System Settings pane.
- Tham khảo sâu: SquirrelDisk (Tauri), Spacie (Swift native fts+Canvas), dumac (getattrlistbulk), parallel-disk-usage, DaisyDisk sunburst history.

Chi tiết research trong AGENTS.md và commit history.

## License
MIT (cho code). Sử dụng cá nhân tự do.

---

**Owner**: dongitran (thiendong.iuh@gmail.com) — senior fullstack JS/TS, NestJS/React/Next/Postgres.

Mọi đóng góp PR đều được review kỹ (CI phải pass, chất lượng code, test thực tế trên máy M-series).
