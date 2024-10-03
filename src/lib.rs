#[derive(Clone)]
pub enum Node {
    Directory {
        name: String,
        children: Vec<Box<Node>>,
    },
    File {
        name: String,
        size: usize,
    },
}
impl Node {
    pub fn get_name(&self) -> &String {
        match self {
            Node::Directory { name, .. } => name,
            Node::File { name, .. } => name,
        }
    }

    pub fn get_child(&self, name: &str) -> Option<&Box<Node>> {
        match self {
            Node::Directory { children, .. } => {
                children.iter().find(|child| child.get_name() == name)
            }
            Node::File { .. } => None,
        }
    }

    pub fn get_mut_child(&mut self, name: &str) -> Option<&mut Box<Node>> {
        match self {
            Node::Directory { ref mut children, .. } => {
                children.iter_mut().find(|child| child.get_name() == name)
            }
            Node::File { .. } => None,
        }
    }

    pub fn get_value(&self) -> usize {
        match self {
            Node::File {size,..} => *size,
            Node::Directory {  children,.. } => {
                children.iter().map(|child| child.get_value()).sum()
            }
        }
    }
}

pub struct FSTreeMap {
    pub(crate) root: Box<Node>,
}

impl FSTreeMap {
    pub fn new() -> Self {
        FSTreeMap {
            root: Box::new(Node::Directory {
                name: String::from("root"),
                children: vec![],
            }),
        }
    }

    pub fn get_size(&self, path: &str) -> usize {
        match self.get_node(path).unwrap() {
            Node::File { size, .. } => *size,
            _ => panic!("no size defined on directories"),
        }
    }

    pub fn get_node(&self, path: &str) -> Option<&Node> {
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

    pub fn insert(&mut self, path: &str, size: usize) {
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
                    size,
                };

                children.push(Box::new(new_node));
            }

            Node::File { .. } => panic!("Path already exists"),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_file_node() {
        let file = Node::File {
            name: "answer.txt".to_string(),
            size: 42,
        };
        assert_eq!(file.get_name(), "answer.txt");
        assert_eq!(file.get_value(), 42);
    }

    #[test]
    fn can_create_directory_node() {
        let dir  = Node::Directory { name: "home".to_string(), children: vec![] };
        assert_eq!(dir.get_name(), "home");
    }

    #[test]
    fn test_add_child_to_path() {
        let mut tree = FSTreeMap::new();
        tree.insert("home",  42);

        assert_eq!(tree.root.get_value(), 42);
    }
}
