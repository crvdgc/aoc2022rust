use std::collections::HashMap;

#[derive(Debug)]
enum Node<'a> {
    File(u32),
    Dir {
        parent: Option<usize>,
        children: HashMap<&'a str, usize>,
    },
}

#[derive(Debug)]
struct Tree<'a> {
    nodes: Vec<Node<'a>>,
}

impl<'a> Tree<'a> {
    fn new() -> Tree<'a> {
        let mut tree: Tree<'a> = Tree { nodes: Vec::new() };
        tree.new_node(Node::Dir {
            parent: None,
            children: HashMap::new(),
        });
        tree
    }
    fn size(&self, i: usize) -> u32 {
        match &self.nodes[i] {
            Node::File(size) => *size,
            Node::Dir { children, .. } => children.values().map(|&x| self.size(x)).sum(),
        }
    }

    fn new_node(&mut self, node: Node<'a>) -> usize {
        self.nodes.push(node);
        return self.nodes.len() - 1;
    }

    fn insert_file(&mut self, i: usize, name: &'a str, f_size: u32) {
        let child_i = self.new_node(Node::File(f_size));
        if let Some(Node::Dir { children, .. }) = self.nodes.get_mut(i) {
            children.insert(name, child_i);
        } else {
            panic!("Cannot add children to a file")
        }
    }

    fn insert_dir(&mut self, i: usize, name: &'a str) {
        let child_i = self.new_node(Node::Dir {
            parent: Some(i),
            children: HashMap::new(),
        });
        if let Some(Node::Dir { children, .. }) = self.nodes.get_mut(i) {
            children.insert(name, child_i);
        } else {
            panic!("Cannot add children to a file")
        }
    }

    fn index_child(&self, i: usize, name: &'a str) -> usize {
        if let Node::Dir { children, .. } = &self.nodes[i] {
            // dbg!(name);
            // dbg!(children);
            *children.get(name).unwrap()
        } else {
            panic!("Cannot index a file")
        }
    }

    fn index_parent(&self, i: usize) -> usize {
        if let Node::Dir { parent, .. } = &self.nodes[i] {
            match parent {
                None => panic!("Cannot cd .. at root"),
                Some(parent) => *parent,
            }
        } else {
            panic!("Cannot index a file")
        }
    }
}

fn process_command<'a>(tree: &mut Tree<'a>, pwd: usize, command: &'a str) -> usize {
    if let Some(dir) = command.strip_prefix("cd ") {
        let dir = &dir[..dir.len() - 1];
        // dbg!(pwd);
        // dbg!(dir);
        // dbg!(&tree);
        if dir == ".." {
            tree.index_parent(pwd)
        } else {
            tree.index_child(pwd, dir)
        }
    } else if let Some(ls_result) = command.strip_prefix("ls\n") {
        for line in ls_result.lines() {
            if let Some(dir) = line.strip_prefix("dir ") {
                tree.insert_dir(pwd, dir);
            } else {
                // file
                let mut iter = line.split(" ");
                let size: u32 = iter.next().unwrap().parse().unwrap();
                let file = iter.next().unwrap();
                tree.insert_file(pwd, file, size);
            }
        }
        pwd
    } else {
        panic!("Unknown command")
    }
}

fn parse<'a>(input: &'a str) -> Tree<'a> {
    let mut commands = input.split("$ ");
    let _ = commands.next(); // discard empty first command
    let _ = commands.next(); // discard "cd /"
    let mut tree = Tree::new();
    let mut pwd: usize = 0;
    for command in commands {
        pwd = process_command(&mut tree, pwd, command)
    }
    tree
}

fn filter_dir_smaller_than<'a>(tree: &Tree<'a>, n: u32, i: usize) -> u32 {
    if let Node::Dir { children, .. } = &tree.nodes[i] {
        let sub_sum = children
            .values()
            .map(|child| filter_dir_smaller_than(tree, n, *child))
            .sum();
        let cur_size = tree.size(i);
        if cur_size <= n {
            sub_sum + cur_size
        } else {
            sub_sum
        }
    } else {
        0
    }
}

pub fn part0(input: String) -> () {
    let tree = parse(&input);
    // println!("{}", tree.size(0))
    let ans = filter_dir_smaller_than(&tree, 100000, 0);
    println!("{}", ans)
}

fn smallest_dir_larger_than<'a>(tree: &Tree<'a>, n: u32, i: usize) -> Option<u32> {
    if let Node::Dir { children, .. } = &tree.nodes[i] {
        let sub_res: Vec<u32> = children
            .values()
            .filter_map(|child| smallest_dir_larger_than(tree, n, *child))
            .collect();
        if let Some(res) = sub_res.iter().min() {
            Some(*res)
        } else {
            let cur_size = tree.size(i);
            if cur_size >= n {
                Some(cur_size)
            } else {
                None
            }
        }
    } else {
        None
    }
}
pub fn part1(input: String) -> () {
    let tree = parse(&input);
    let used = tree.size(0);
    let total: u32 = 70000000;
    let current_free: u32 = total - used;
    let target: u32 = 30000000;
    let need_to_free = target - current_free;
    let ans = smallest_dir_larger_than(&tree, need_to_free, 0);
    println!("{}", ans.unwrap())
}
pub fn example_input() -> String {
    r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#
        .to_string()
}
