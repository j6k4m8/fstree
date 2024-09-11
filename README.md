# fstree

fstree is a super simple crate that stores a map of data with a tree-like structure.

The primary use-case is to store [a file-system like structure](https://github.com/j6k4m8/rclonedirstat) and store metadata about each file.

for example, you might want to store the file size of every file in a tree and then be able to recursively get subtree size:

```rust
use fstree::FsTree;

let mut tree = FsTree::new();
tree.insert("README.md", 100);
tree.insert("src/main.rs", 200);
tree.insert("src/lib.rs", 300);
```

Map, reduce, match, and traversal functions:

```rust
let total_size = tree.topo_reduce(|acc, size| acc + size, 0);

let bytes_to_kilobytes: FsTree<String, u64> = tree.topo_map(|size| size / 1024);

let has_readme = tree.topo_any(|path, _| path == "README.md");

let mut paths = Vec::new();
tree.topo_traverse(|path, _| paths.push(path.clone()));
```
