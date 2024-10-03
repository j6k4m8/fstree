# fstree

> [!WARNING]
> this isn't working like even a little bit

fstree is a super simple crate that stores a map of data with a tree-like structure.

The primary use-case is to store [a file-system like structure](https://github.com/j6k4m8/rclonedirstat) and store metadata about each file.

for example, you might want to store the file size of every file in a tree and then be able to recursively get subtree size:

```rust
use fstree::FsTree;

let mut tree = FSTreeMap::new();
tree.insert_with_parents("home/users/arthur/answer.txt",  42);
tree.insert_with_parents("home/users/arthur/password.txt",  128);
```

View the tree:

```rust
tree.print_tree();
```

```txt
root
 home
  users
   arthur
    answer.txt: 42
    password.txt: 128
```

Map, reduce, match, and traversal functions:

```rust
tree.root.value_reduce(0,|acc,x| acc+x)
```

```txt
170
```

## roadmap

```rust
let bytes_to_kilobytes: FsTree<String, u64> = tree.topo_map(|size| size / 1024);

let has*readme = tree.topo_any(|path, _| path == "README.md");

let mut paths = Vec::new();
tree.topo_traverse(|path, _| paths.push(path.clone()));
```
