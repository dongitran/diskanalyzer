# AGENTS.md — diskanalyzer (macOS Disk Usage Visualizer)

## Mục đích
App desktop macOS native-feel, scan dung lượng disk từ root `/`, trực quan với **Sunburst (chart tròn phân cấp)**, drill-down click, actions an toàn. Tương đương DaisyDisk nhưng open / nghiên cứu, buildable, CI .dmg installable.

**Đối tượng**: Senior dev (JS/TS + Rust) muốn tool nhanh, đẹp, chính xác cho cleanup.

## Ngôn ngữ & phong cách
- Code: **English** (identifiers, comments, types, logs).
- Docs / README / UI strings: **Tiếng Việt + English** (hướng dẫn permissions, error messages quan trọng bằng VN).
- Commit: `feat: ...`, `fix: ...`, `docs: ...`, `ci: ...` (ngắn gọn).
- PR: Mô tả rõ, ảnh chụp nếu UI, "CI must pass".

## Cấu trúc project (quan trọng)
```
diskanalyzer/
├── src/                  # SvelteKit frontend (TS, Svelte 5 runes)
│   ├── lib/types.ts
│   └── routes/+page.svelte   # Main UI (topbar, sidebar volumes, sunburst, details)
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs        # Tauri commands (get_volumes, scan_directory, permissions)
│   │   ├── main.rs
│   │   └── scanner.rs    # Tree builder + pruning
│   ├── Cargo.toml
│   ├── tauri.conf.json   # productName, identifier=com.dongitran.diskanalyzer, entitlements
│   ├── entitlements.mac.plist
│   └── icons/
├── .github/workflows/ci.yml   # lint + build-macos (tauri-action, upload .dmg artifact)
├── package.json (scripts: check, lint, cspell, quality, tauri build)
├── tsconfig.json / cspell.json / eslint.config.js
├── AGENTS.md
└── README.md (bilingual, install, dev, limitations)
```

## Quy trình làm việc với AI / agent
1. **Luôn** chạy `pnpm quality` (typecheck + cspell) + `cd src-tauri && cargo check` trước khi commit.
2. Thay đổi lớn (scanner, viz, permissions) → test thực tế: `pnpm tauri dev`, scan ~/ + / (với Full Disk Access), drill, progress.
3. CI: Mọi push/PR phải pass `lint-and-quality` + `build-macos` (artifact .dmg phải sinh ra).
4. Khi fix CI fail: dùng `gh run list`, `gh run view --log`, fix, push lại. Lặp đến khi xanh.
5. Tránh: hardcode path, thiếu error handling permission, memory leak (giữ tree lớn), không prune small files.

## Chất lượng code (nhiều rule)
- **Frontend**: Svelte 5 + TS strict, svelte-check, ESLint (mở rộng dần), Prettier khuyến khích.
- **Rust**: clippy -D warnings, cargo fmt, no unwrap trong prod path (dùng ? + context).
- **Spelling**: cspell toàn project (wordlist có tech + VN terms).
- **Security**: Không hardcode secrets, entitlements đúng, capabilities tối thiểu.
- **Perf**: Scanner phải nhanh (jwalk + pruning), UI không block (progress event).
- **macOS specific**: entitlements, TCC Full Disk Access flow, reveal via `open -R`, bundle identifier đúng.
- Pre-commit: Khuyến khích lefthook (chưa bắt buộc trong MVP).

## Phát hành & Distribution
- GitHub Actions build .dmg cho aarch64 (M-series).
- Artifact retention 14 ngày.
- Để "cài được dễ": document "Right click > Open" cho unsigned build.
- Sau: thêm signing/notarization (secrets Apple ID + certificate) → release proper.

## Known issues & research notes (cập nhật thường xuyên)
- APFS clones: overcount (giống DaisyDisk). Ưu tiên allocated size.
- Full scan / : cần user grant Full Disk Access thủ công.
- jwalk đủ nhanh cho MVP; sau upgrade getattrlistbulk / fts (xem dumac, Spacie source).
- Sunburst: D3 zoomable (xem Observable Mike Bostock). Treemap là nice-to-have.
- Tham khảo chính: SquirrelDisk (Tauri sunburst), Spacie (Swift fts+Canvas+cache+FSEvents), VizDisk, parallel-disk-usage crate.

## Lệnh hữu ích
```bash
pnpm tauri dev
pnpm tauri build -- --target aarch64-apple-darwin
cd src-tauri && cargo clippy -- -D warnings
pnpm quality
```

## Git & remote
- Repo: github.com/dongitran/diskanalyzer (tài khoản dongitran).
- Khi push từ local: đảm bảo git config user.name/email = dongitran's.
- Dùng gh cli (active account dongitran) để tạo repo, release, theo dõi run.

---

Follow đúng AGENTS.md này + root brain AGENTS.md. Mọi thay đổi lớn cần giải thích ngắn gọn trong commit/PR.

Chúc may mắn, làm đến khi CI build được .dmg sạch và user cài được trên máy thật!