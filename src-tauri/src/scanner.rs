//! Tree building and pruning logic for disk scan results.
//! Designed to be memory efficient and produce nice data for sunburst viz.

use std::collections::HashMap;
use std::path::Path;

use super::DiskNode;

/// Build a hierarchical tree from flat (path, size, is_dir) walk results.
/// Applies simple pruning: if a dir has many small children, group into "other (N files)".
pub fn build_tree_from_entries(
    root: &Path,
    entries: Vec<(std::path::PathBuf, u64, bool)>,
    prune_threshold: usize,
) -> DiskNode {
    // Use a map of path -> (size, is_dir, children_names)
    // Simpler approach: sort entries, build recursively.

    let root_str = root.to_string_lossy().to_string();
    let mut by_parent: HashMap<String, Vec<(String, u64, bool)>> = HashMap::new();

    for (full_path, size, is_dir) in entries {
        let p = full_path.to_string_lossy().to_string();
        if p == root_str {
            continue;
        }
        let parent = full_path
            .parent()
            .map(|pa| pa.to_string_lossy().to_string())
            .unwrap_or_else(|| root_str.clone());

        let name = full_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| p.clone());

        by_parent
            .entry(parent)
            .or_default()
            .push((name, size, is_dir));
    }

    // Recursive builder
    fn build(
        current_path: &str,
        by_parent: &HashMap<String, Vec<(String, u64, bool)>>,
        prune_threshold: usize,
    ) -> DiskNode {
        let children_raw = by_parent.get(current_path).cloned().unwrap_or_default();

        let mut children: Vec<DiskNode> = children_raw
            .into_iter()
            .map(|(name, size, is_dir)| {
                let child_path = if current_path.ends_with('/') || current_path.is_empty() {
                    format!("{}{}", current_path, name)
                } else {
                    format!("{}/{}", current_path, name)
                };
                let mut node = DiskNode {
                    name,
                    path: child_path.clone(),
                    size,
                    is_dir,
                    children: None,
                };
                if is_dir {
                    let sub = build(&child_path, by_parent, prune_threshold);
                    node.size = sub.size; // override with summed size for dirs
                    node.children = Some(sub.children.unwrap_or_default());
                }
                node
            })
            .collect();

        // Compute dir size as sum of children (if not already)
        let summed: u64 = children.iter().map(|c| c.size).sum();

        // Prune: too many small siblings -> aggregate
        if children.len() > prune_threshold {
            let mut kept: Vec<DiskNode> = Vec::new();
            let mut other_size: u64 = 0;
            let mut other_count: usize = 0;

            // Sort desc by size
            children.sort_by(|a, b| b.size.cmp(&a.size));

            for c in children {
                if kept.len() < prune_threshold / 2 {
                    kept.push(c);
                } else {
                    other_size += c.size;
                    other_count += 1;
                }
            }

            if other_count > 0 {
                kept.push(DiskNode {
                    name: format!("other ({} items)", other_count),
                    path: format!("{}/__other__", current_path),
                    size: other_size,
                    is_dir: false,
                    children: None,
                });
            }
            children = kept;
        }

        let total_size = if summed > 0 { summed } else { /* leaf or empty */ 0 };

        DiskNode {
            name: current_path
                .rsplit('/')
                .next()
                .filter(|s| !s.is_empty())
                .unwrap_or(current_path)
                .to_string(),
            path: current_path.to_string(),
            size: total_size,
            is_dir: true,
            children: if children.is_empty() { None } else { Some(children) },
        }
    }

    let root_name = root
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "/".to_string());

    let mut root_node = build(&root_str, &by_parent, prune_threshold);
    root_node.name = root_name;
    root_node.path = root_str;

    // If root had direct size 0, keep summed
    root_node
}