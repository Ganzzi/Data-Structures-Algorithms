use hash_map::hash_map::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    children: HashMap<u8, Box<Node>>,
    is_end: bool,
}

pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: Node {
                children: HashMap::new(),
                is_end: false,
            },
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;

        for c in word.as_bytes() {
            if node.children.contains(c) {
                node = node.children.get_mut(c).unwrap();
            } else {
                let new_node = Node {
                    children: HashMap::new(),
                    is_end: false,
                };
                node.children.insert(*c, Box::new(new_node));
                node = node.children.get_mut(c).unwrap();
            }
        }

        node.is_end = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        self.contains_prefix(word).map_or(false, |node| node.is_end)
    }

    pub fn start_with(&self, pre_fix: &str) -> bool {
        self.contains_prefix(pre_fix).is_some()
    }

    fn contains_prefix(&self, pre_fix: &str) -> Option<&Node> {
        let mut node = &self.root;

        for c in pre_fix.as_bytes() {
            if node.children.contains(c) {
                node = node.children.get(c).unwrap();
            } else {
                return None;
            }
        }

        Some(node)
    }
}
