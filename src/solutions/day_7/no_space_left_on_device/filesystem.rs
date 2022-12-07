use std::collections::HashMap;
use std::fmt;

type Name = String;
type Path = Vec<Name>;

pub struct TreeNode {
    name: Name,
    children: HashMap<Name, TreeNode>,
    size: u64,
    is_dir: bool,
}

impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (size={})", self.name, self.size)
    }
}

impl TreeNode {
    pub fn new_dir(name: Name) -> Self {
        Self {
            name,
            children: HashMap::new(),
            size: 0,
            is_dir: true,
        }
    }

    pub fn new_file(name: Name, size: u64) -> Self {
        Self {
            name,
            children: HashMap::new(),
            size: size,
            is_dir: false,
        }
    }

    pub fn add_child(&mut self, node: TreeNode) {
        self.children.insert(node.name.to_string(), node);
    }

    pub fn get_child(&mut self, name: Name) -> Option<&mut TreeNode> {
        match self.children.get_mut(&name) {
            Some(node) => Some(node),
            None => None,
        }
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn get_node(&mut self, path: &Path) -> Option<&mut TreeNode> {
        self.get_node_iter(path, 0)
    }

    fn get_node_iter(&mut self, path: &Path, i: usize) -> Option<&mut TreeNode> {
        if i == path.len() {
            return Some(self);
        }
        match self.children.get_mut(&path[i]) {
            Some(node) => {
                return node.get_node_iter(path, i + 1);
            }
            _ => return None,
        };
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        self.print_iter(0);
    }

    #[allow(dead_code)]
    fn print_iter(&self, i: usize) {
        for _ in 0..i {
            print!("    ");
        }
        println!("{} (size={})", self.name, self.size);
        for (_, node) in self.children.iter() {
            node.print_iter(i + 1);
        }
    }

    pub fn eval_size(&mut self) {
        if !self.is_dir {
            return;
        }
        let mut size = 0;
        for (_, node) in self.children.iter_mut() {
            node.eval_size();
            size += node.size;
        }
        self.size = size;
    }

    pub fn get_nodes_of_size_at_most(&self, size: u64) -> Vec<&TreeNode> {
        let mut result = vec![];
        if !self.is_dir {
            return result;
        }
        for (_, node) in self.children.iter() {
            for v in node.get_nodes_of_size_at_most(size).iter() {
                result.push(v);
            }
        }
        if self.size <= size {
            result.push(self);
        }

        result
    }

    pub fn get_min_dir_of_size_higher_than(&self, min_size: u64) -> Option<&TreeNode> {
        if !self.is_dir || self.size < min_size {
            return None;
        }
        let mut current: Option<&TreeNode> = None;
        for (_, node) in self.children.iter() {
            match node.get_min_dir_of_size_higher_than(min_size) {
                Some(node_min_dir) => match current {
                    Some(min_dir) => {
                        if node_min_dir.size < min_dir.size {
                            current = Some(node_min_dir);
                        }
                    }
                    None => current = Some(node_min_dir),
                },
                None => {}
            }
        }
        if current.is_some() {
            return current;
        }
        if self.size >= min_size {
            return Some(self);
        }
        None
    }
}

pub struct Tree {
    pub root: TreeNode,
    path: Path,

    total_space: u64,
}

impl Tree {
    pub fn new(total_space: u64) -> Self {
        let root = TreeNode::new_dir("/".to_string());
        Self {
            root: root,
            path: vec![],
            total_space,
        }
    }

    pub fn cd(&mut self, dir: &String) {
        if dir.eq("/") {
            self.path = vec![];
            return;
        }
        if dir.eq("..") {
            self.path.pop();
            return;
        }
        let current = match self.current_node() {
            Some(node) => node,
            None => panic!("current node not found, path: {:?}", self.path),
        };
        match current.get_child(dir.to_string()) {
            Some(_) => self.path.push(dir.to_string()),
            None => panic!("dir '{dir}' not found, current path: {:?}", self.path),
        }
    }

    pub fn add_dir(&mut self, name: &String) {
        match self.current_node() {
            Some(node) => {
                node.add_child(TreeNode::new_dir(name.to_string()));
            }
            None => panic!("current node not found, path: {:?}", self.path),
        }
    }

    pub fn add_file(&mut self, name: &String, size: u64) {
        match self.current_node() {
            Some(node) => {
                node.add_child(TreeNode::new_file(name.to_string(), size));
            }
            None => return,
        }
    }

    fn current_node(&mut self) -> Option<&mut TreeNode> {
        self.root.get_node(&self.path)
    }

    pub fn sum_size_of_dirs_less_than(&self, size: u64) -> u64 {
        let mut sum = 0;
        self.root
            .get_nodes_of_size_at_most(size)
            .iter()
            .for_each(|node| {
                sum += node.get_size();
            });
        sum
    }

    pub fn get_size_of_dir_to_delete(&self, needed_space: u64) -> u64 {
        let current_free_space = self.total_space - self.root.get_size();
        let min_space_to_free = needed_space - current_free_space;

        match self.root.get_min_dir_of_size_higher_than(min_space_to_free) {
            Some(node) => node.get_size(),
            None => self.root.get_size(),
        }
    }
}
