/// FSTreeMap is a tree structure that represents the file system.
///
/// It is distinct from the _actual_ file system in that it can represent a
/// fully virtual or otherwise imaginary, abstracted file system.
///
/// The tree is a bidirected graph, with each node representing a file or
/// directory. Both files and directories are Nodes, but directories have
/// children, which are also Nodes.
///
/// The File (leaf) is implemented by files, and the Directory trait is
/// implemented by directories. The Node is implemented by both.
/// Only leaves have data, and only directories have children.
///
/// Values are generickized, so you can store any type of data in the tree.

/// Nodes are a container for either files or directories.
trait Node<V> {
    /// Returns the name of the node.
    fn name(&self) -> &str;
}

/// Files are leaves in the tree:
struct FileNode<V> {
    name: String,
    data: V,
}

impl<V> Node<V> for FileNode<V> {
    fn name(&self) -> &str {
        &self.name
    }
}

/// Directories are nodes with children.
struct DirectoryNode<V> {
    name: String,
    children: Vec<Box<dyn Node<V>>>,
}

impl<V> Node<V> for DirectoryNode<V> {
    fn name(&self) -> &str {
        &self.name
    }
}

impl<V> DirectoryNode<V> {
    fn new(name: &str) -> Self {
        DirectoryNode {
            name: name.to_string(),
            children: Vec::<Box<dyn Node<V>>>::new(),
        }
    }

    fn add_child(&mut self, child: Box<dyn Node<V>>) {
        self.children.push(child);
    }

    fn remove_child(&mut self, name: &str) {
        self.children.retain(|child| child.name() != name);
    }

    fn get_child(&self, name: &str) -> Option<&Box<dyn Node<V>>> {
        self.children.iter().find(|child| child.name() == name)
    }

    fn get_children(&self) -> &Vec<Box<dyn Node<V>>> {
        &self.children
    }
}

/// The tree itself.
struct FSTreeMap<V> {
    root: DirectoryNode<V>,
}

/// Implement the tree.
impl<V> FSTreeMap<V> {
    fn new() -> Self {
        FSTreeMap {
            root: DirectoryNode::new("root"),
        }
    }

    fn add_child_to_path(
        &mut self,
        path: Vec<&str>,
        child: Box<dyn Node<V>>,
        create_parents: bool,
    ) {
        // Start at the root.
        let mut current = &mut self.root;
        // For each subdir in the path:
        for name in path {
            // Get the child if it exists, or panic/create:
            let child = match current.get_child(name) {
                Some(child) => child,
                None => {
                    if create_parents {
                        let new_child = Box::new(DirectoryNode::new(name));
                        current.add_child(new_child);
                        current.get_child(name).unwrap()
                    } else {
                        panic!("Path not found.")
                    }
                }
            };

            // If the child is a directory, continue to traverse deeper:
            match child {
                DirectoryNode => current = child,
                FileNode => panic!("Path not found."),
            }
        }
        // Add the child to the directory.
        current.add_child(child);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_file_node() {
        let file = FileNode {
            name: "answer.txt".to_string(),
            data: 42,
        };
        assert_eq!(file.name, "answer.txt");
        assert_eq!(file.data, 42);
    }

    #[test]
    fn can_create_directory_node() {
        let dir: DirectoryNode<i32> = DirectoryNode::new("home");
        assert_eq!(dir.name, "home");
    }

    #[test]
    fn can_add_child_to_directory_node() {
        let mut dir: DirectoryNode<i32> = DirectoryNode::new("home");
        let child = Box::new(FileNode {
            name: "answer.txt".to_string(),
            data: 42,
        });
        dir.add_child(child);
        assert_eq!(dir.get_children().len(), 1);
    }

    #[test]
    fn test_add_child_to_path() {
        let mut tree = FSTreeMap::new();
        let child = Box::new(FileNode {
            name: "answer.txt".to_string(),
            data: 42,
        });
        tree.add_child_to_path(vec!["home", "users", "arthur"], child, true);

        assert_eq!(tree.root.get_child("home").unwrap().name(), "home");
        // assert_eq!(
        //     tree.root
        //         .get_child("home")
        //         .unwrap()
        //         .get_child("users")
        //         .unwrap()
        //         .name(),
        //     "users"
        // );
    }
}
