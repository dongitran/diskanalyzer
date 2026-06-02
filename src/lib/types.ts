// Core types for Disk Analyzer (28-disk-analyzer)
// Professional macOS disk usage visualizer

export interface DiskInfo {
  name: string;
  mount_point: string;
  total: number;      // bytes
  available: number;  // bytes
}

export interface ScanProgress {
  current_path: string;
  files_scanned: number;
  bytes_scanned: number;
  percent: number;
}

/**
 * Hierarchical node for sunburst / treemap / tree views.
 * size is physical/allocated bytes where possible.
 */
export interface DiskNode {
  name: string;
  path: string;
  size: number;           // allocated / physical bytes
  is_dir: boolean;
  children?: DiskNode[];
  // Future: file_count, last_modified, type_category (for coloring)
}

export interface ScanOptions {
  path: string;
  maxDepth?: number;
  minSize?: number;       // ignore tiny files for pruning
  includeHidden?: boolean;
}