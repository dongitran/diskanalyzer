//! Disk Analyzer - Professional macOS Disk Usage Visualizer (Tauri v2 + Rust)
//! Research-backed: fast scanner (jwalk + sysinfo), sunburst viz, Full Disk Access UX.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use jwalk::WalkDir;
use sysinfo::Disks;
use tauri::{Emitter, State};
use tauri_plugin_shell::ShellExt;

mod scanner;

// --- Types (shared with frontend) ---
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total: u64,
    pub available: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanProgress {
    pub current_path: String,
    pub files_scanned: u64,
    pub bytes_scanned: u64,
    pub percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskNode {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub is_dir: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<DiskNode>>,
}

// Shared scan state for cancellation / progress (simple for v1)
#[derive(Default)]
struct ScanState {
    cancel: AtomicBool,
    files: AtomicU64,
    bytes: AtomicU64,
}

#[tauri::command]
async fn get_volumes() -> Result<Vec<DiskInfo>, String> {
    let mut disks = Disks::new_with_refreshed_list();
    // Refresh again for latest
    disks.refresh_list();

    let mut vols: Vec<DiskInfo> = disks
        .iter()
        .filter(|d| d.is_removable() || d.mount_point().to_string_lossy() == "/" || d.mount_point().to_string_lossy().contains("Data"))
        .map(|d| DiskInfo {
            name: d.name().to_string_lossy().to_string(),
            mount_point: d.mount_point().to_string_lossy().to_string(),
            total: d.total_space(),
            available: d.available_space(),
        })
        .collect();

    // Always include root if not present
    if !vols.iter().any(|v| v.mount_point == "/") {
        vols.push(DiskInfo {
            name: "Root (/)".to_string(),
            mount_point: "/".to_string(),
            total: 0,
            available: 0,
        });
    }

    // Dedup by mount_point
    vols.sort_by(|a, b| a.mount_point.cmp(&b.mount_point));
    vols.dedup_by(|a, b| a.mount_point == b.mount_point);

    Ok(vols)
}

/// Main scan command. Builds a pruned tree for visualization.
/// For very large trees we prune small children into "other".
#[tauri::command]
async fn scan_directory(
    app: tauri::AppHandle,
    path: String,
    max_depth: Option<usize>,
    state: State<'_, Arc<ScanState>>,
) -> Result<DiskNode, String> {
    let root = PathBuf::from(&path);
    if !root.exists() {
        return Err(format!("Path does not exist: {}", path));
    }

    // Reset state
    state.cancel.store(false, Ordering::Relaxed);
    state.files.store(0, Ordering::Relaxed);
    state.bytes.store(0, Ordering::Relaxed);

    let max_d = max_depth.unwrap_or(5);
    // Clone the Arc so the walk closure can share the atomics
    let state_for_walk = state.inner().clone();

    // Use jwalk for fast parallel directory walking (good balance, easy to extend with getattrlistbulk later)
    let mut entries: Vec<(PathBuf, u64, bool)> = Vec::new();

    // Simple walk - in real we would use channels for live progress
    for entry in WalkDir::new(&root)
        .max_depth(max_d)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if state_for_walk.cancel.load(Ordering::Relaxed) {
            break;
        }

        let path = entry.path();
        let is_dir = entry.file_type().is_dir();

        // Get size (for dirs we sum later; for files use metadata)
        let size = if is_dir {
            0 // will be computed in tree build
        } else {
            entry.metadata().map(|m| m.len()).unwrap_or(0)
        };

        state_for_walk.files.fetch_add(1, Ordering::Relaxed);
        state_for_walk.bytes.fetch_add(size, Ordering::Relaxed);

        // Emit progress every ~2000 files (lightweight)
        let scanned = state_for_walk.files.load(Ordering::Relaxed);
        if scanned % 2000 == 0 {
            let _ = app.emit(
                "scan-progress",
                ScanProgress {
                    current_path: path.to_string_lossy().to_string(),
                    files_scanned: scanned,
                    bytes_scanned: state_for_walk.bytes.load(Ordering::Relaxed),
                    percent: 0.0, // real % hard without total upfront
                },
            );
        }

        entries.push((path, size, is_dir));
    }

    // Build tree from flat entries (simple parent-child by path prefix for demo)
    // For production this would be more sophisticated arena + inode dedup + pruning
    let root_node = scanner::build_tree_from_entries(&root, entries, 50); // prune threshold 50 small siblings

    // Final progress
    let _ = app.emit(
        "scan-progress",
        ScanProgress {
            current_path: "complete".to_string(),
            files_scanned: state_for_walk.files.load(Ordering::Relaxed),
            bytes_scanned: state_for_walk.bytes.load(Ordering::Relaxed),
            percent: 100.0,
        },
    );

    Ok(root_node)
}

#[tauri::command]
fn cancel_scan(state: State<'_, Arc<ScanState>>) {
    state.cancel.store(true, Ordering::Relaxed);
}

#[tauri::command]
async fn check_full_disk_access() -> Result<bool, String> {
    // Provided by tauri-plugin-macos-permissions
    // In real: use the plugin command or direct
    // For now return true to not block UI; real check in plugin
    Ok(true)
}

#[tauri::command]
async fn request_full_disk_access(app: tauri::AppHandle) -> Result<(), String> {
    // Opens System Settings or prompts via plugin
    // Fallback: open the privacy pane
    let _ = app
        .shell()
        .command("open")
        .args(["x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles"])
        .spawn();
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let scan_state = Arc::new(ScanState::default());

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_macos_permissions::init())
        .manage(scan_state)
        .invoke_handler(tauri::generate_handler![
            get_volumes,
            scan_directory,
            cancel_scan,
            check_full_disk_access,
            request_full_disk_access
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
