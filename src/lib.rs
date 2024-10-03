use std::fmt::Debug;

#[derive(Clone)]
pub enum Node<V> {
    Directory {
        name: String,
        children: Vec<Box<Node<V>>>,
    },
    File {
        name: String,
        size: V,
    },
}
impl<V> Node<V>
where
    V: std::iter::Sum,
    V: Clone,
{
    pub fn get_name(&self) -> &String {
        match self {
            Node::Directory { name, .. } => name,
            Node::File { name, .. } => name,
        }
    }

    pub fn get_child(&self, name: &str) -> Option<&Box<Node<V>>> {
        match self {
            Node::Directory { children, .. } => {
                children.iter().find(|child| child.get_name() == name)
            }
            Node::File { .. } => None,
        }
    }

    pub fn get_mut_child(&mut self, name: &str) -> Option<&mut Box<Node<V>>> {
        match self {
            Node::Directory {
                ref mut children, ..
            } => children.iter_mut().find(|child| child.get_name() == name),
            Node::File { .. } => None,
        }
    }

    pub fn make_directory(&mut self, name: &str) -> &mut Box<Node<V>> {
        match self {
            Node::Directory { children, .. } => {
                let new_node = Node::Directory {
                    name: name.to_string(),
                    children: vec![],
                };
                children.push(Box::new(new_node));
                children.last_mut().unwrap()
            }
            Node::File { .. } => panic!("Cannot make directory on file"),
        }
    }

    // pub fn get_value(&self) -> &V {
    //     match self {
    //         Node::File {size,..} => size,
    //         Node::Directory {  children,.. } => {
    //             &children.iter().map(|child| *child.get_value()).sum()
    //         }
    //     }
    // }

    pub fn value_reduce<T>(&self, accumulator: T, f: fn(T, V) -> T) -> T {
        match self {
            Node::File { size, .. } => f(accumulator, size.clone()),
            Node::Directory { children, .. } => children
                .iter()
                .fold(accumulator, |acc, child| child.value_reduce(acc, f)),
        }
    }
}

pub struct FSTreeMap<V> {
    pub(crate) root: Box<Node<V>>,
}

impl<V> FSTreeMap<V>
where
    V: std::iter::Sum,
    V: Clone,
{
    pub fn new() -> Self {
        FSTreeMap {
            root: Box::new(Node::Directory {
                name: String::from("root"),
                children: vec![],
            }),
        }
    }

    pub fn get_size(&self, path: &str) -> &V {
        match self.get_node(path).unwrap() {
            Node::File { size, .. } => size,
            _ => panic!("no size defined on directories"),
        }
    }

    pub fn get_node(&self, path: &str) -> Option<&Node<V>> {
        let mut current = &self.root;
        for part in path.split('/') {
            match **current {
                Node::Directory { ref children, .. } => {
                    match children.iter().find(|child| {
                        let node = child.as_ref();
                        let name = node.get_name();
                        return *name == String::from(part);
                    }) {
                        Some(child) => {
                            current = child;
                        }
                        None => return None,
                    }
                }
                _ => panic!("Path is not a directory"),
            }
        }
        Some(current)
    }

    pub fn insert(&mut self, path: &str, value: V) {
        let mut current = &mut self.root;
        let full_path_split: Vec<&str> = path.split('/').into_iter().collect();
        let dirpath = &full_path_split[..full_path_split.len() - 1];
        let stem = full_path_split.last().unwrap();
        for part in dirpath {
            current = current.get_mut_child(part).unwrap();
        }

        match **current {
            Node::Directory {
                ref mut children, ..
            } => {
                let new_node = Node::File {
                    name: stem.to_string(),
                    size: value,
                };

                children.push(Box::new(new_node));
            }

            Node::File { .. } => panic!("Path already exists"),
        }
    }

    pub fn remove(&mut self, path: &str) {
        let mut current = &mut self.root;
        let full_path_split: Vec<&str> = path.split('/').into_iter().collect();
        let dirpath = &full_path_split[..full_path_split.len() - 1];
        let stem = full_path_split.last().unwrap();
        for part in dirpath {
            current = current.get_mut_child(part).unwrap();
        }

        match **current {
            Node::Directory {
                ref mut children, ..
            } => {
                children.retain(|child| {
                    let node = child.as_ref();
                    let name = node.get_name();
                    return *name != String::from(*stem);
                });
            }

            Node::File { .. } => panic!("Path is a file"),
        }
    }

    pub fn get_children(&self, path: &str) -> Option<&Vec<Box<Node<V>>>> {
        match self.get_node(path) {
            Some(Node::Directory { children, .. }) => Some(children),
            _ => None,
        }
    }

    pub fn make_directory(&mut self, path: &str) {
        let mut current = &mut self.root;
        let full_path_split: Vec<&str> = path.split('/').into_iter().collect();
        let dirpath = &full_path_split;
        for part in dirpath {
            let maybe_directory = current.get_child(part);
            match maybe_directory {
                Some(..) => {
                    current = current.get_mut_child(part).unwrap();
                }
                None => {
                    current = current.make_directory(part);
                }
            }
        }
    }

    pub fn insert_with_parents(&mut self, path: &str, value: V) {
        let mut current = &mut self.root;
        let full_path_split: Vec<&str> = path.split('/').into_iter().collect();
        let dirpath = &full_path_split[..full_path_split.len() - 1];
        let stem = full_path_split.last().unwrap();
        for part in dirpath {
            let maybe_directory = current.get_child(part);
            match maybe_directory {
                Some(..) => {
                    current = current.get_mut_child(part).unwrap();
                }
                None => {
                    current = current.make_directory(part);
                }
            }
        }

        match **current {
            Node::Directory {
                ref mut children, ..
            } => {
                let new_node = Node::File {
                    name: stem.to_string(),
                    size: value,
                };

                children.push(Box::new(new_node));
            }

            Node::File { .. } => panic!("Path already exists"),
        }
    }
}

impl<V> FSTreeMap<V>
where
    V: Debug,
    V: std::fmt::Display,
{
    pub fn print_tree(&self) {
        self.print_tree_recursive(&self.root, 0);
    }
    fn print_tree_recursive(&self, node: &Node<V>, depth: usize) {
        match node {
            Node::Directory { name, children } => {
                println!("{}{}", " ".repeat(depth), name);
                for child in children {
                    self.print_tree_recursive(child, depth + 1);
                }
            }
            Node::File { name, size } => {
                println!("{}{}: {}", " ".repeat(depth), name, size);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_file_node() {
        let file = Node::File::<usize> {
            name: "answer.txt".to_string(),
            size: 42,
        };
        assert_eq!(file.get_name(), "answer.txt");
        assert_eq!(file.value_reduce(0, |acc, x| acc + x), 42);
    }

    #[test]
    fn can_create_directory_node() {
        let dir = Node::Directory::<usize> {
            name: "home".to_string(),
            children: vec![],
        };
        assert_eq!(dir.get_name(), "home");
    }

    #[test]
    fn test_add_child_to_path() {
        let mut tree = FSTreeMap::new();
        tree.insert("home", 42);

        assert_eq!(tree.root.value_reduce(0, |acc, x| acc + x), 42);
    }

    #[test]
    fn test_print_tree() {
        let mut tree = FSTreeMap::new();
        // tree.make_directory("home");
        tree.insert_with_parents("home/users/arthur/answer.txt", 42);
        tree.insert_with_parents("home/users/arthur/password.txt", 128);
        tree.print_tree();
        assert_eq!(tree.root.value_reduce(0, |acc, x| acc + x), 170);
    }
}
