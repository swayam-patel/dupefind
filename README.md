# dupefind

Finds duplicate files in a directory tree by content (not filename), using parallel hashing.

## Install / Build

```bash
git clone https://github.com/swayam-patel/dupefind.git
cd dupefind
cargo build --release
```

The binary will be at `target/release/dupefind`.

## Usage

```bash
dupefind <path> [--min-size <bytes>] [--sequential]
```

- `<path>` — directory to scan (required)
- `--min-size <bytes>` — skip files smaller than this (default: 1)
- `--sequential` — hash files one at a time instead of in parallel, for benchmarking

**Example:**

```bash
dupefind ~/Downloads
```

## Example output

```
Found 2 candidate files (size collisions) to hash...
Hashed in 0.001s (parallel via rayon)

Duplicate set (2 files):
  photos_backup/beach_copy.jpg
  photos/beach.jpg

1 duplicate sets found. ~0.00 MB reclaimable.
```