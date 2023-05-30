use common::lines_from_file;
use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

type Error = &'static str;

pub fn run(filename: &str) -> Result<(), Error> {
    let root = parse(lines_from_file(filename)?)?;
    print_tree(&root, 0);
    let (total, solution_p1) = solve_p1(&root);
    println!("Solution P1: {} (total: {total})", solution_p1);
    let (_, solution_p2) = solve_p2(
        &root,
        get_missing_space(&root).ok_or("Space is not missing")?,
    );
    println!(
        "Solution P2: {}",
        solution_p2.ok_or("No dir could be removed")?
    );
    Ok(())
}

const STORAGE: usize = 70000000;
const UPDATE_SIZE: usize = 30000000;
const MAX_USAGE: usize = STORAGE - UPDATE_SIZE;

fn get_missing_space(root: &Directory) -> Option<usize> {
    let used_storage = root.get_size();

    if used_storage > MAX_USAGE {
        Some(used_storage - MAX_USAGE)
    } else {
        None
    }
}

fn solve_p1(dir: &Directory) -> (usize, usize) {
    let (size, mut result) = dir
        .children
        .borrow()
        .values()
        .map(|item| match item {
            Node::Directory(dir) => solve_p1(dir),
            Node::File(file) => (file.size, 0),
        })
        .reduce(|(a, c), (b, d)| (a + b, c + d))
        .unwrap_or((0, 0));

    if size <= 100000 {
        result += size;
    }

    (size, result)
}

// Not proud of this one..
fn solve_p2(dir: &Directory, removal_target: usize) -> (usize, Option<usize>) {
    let (size, mut result) = dir
        .children
        .borrow()
        .values()
        .map(|item| match item {
            Node::Directory(dir) => solve_p2(dir, removal_target),
            Node::File(file) => (file.size, None),
        })
        .reduce(|a, b| (a.0 + b.0, [a.1, b.1].iter().filter_map(|i| i.clone()).min()))
        .unwrap_or((0, None));

    if result == None && size >= removal_target {
        result = Some(size)
    }

    (size, result)
}

fn parse(input: impl Iterator<Item = impl Borrow<str>>) -> Result<Directory, Error> {
    let root = Rc::new(Directory::new(None));
    let mut current: Rc<Directory> = root.clone();
    let mut ls_output = false;

    for line in input {
        let mut line = line.borrow().split_whitespace();

        let first = line.next().ok_or("Empty line.")?;
        if first == "$" {
            ls_output = false;
        }

        if ls_output {
            match first {
                "dir" => {
                    let name = line
                        .next()
                        .ok_or("Incorrect ls output: expecting directory's name.")?
                        .to_string();
                    let dir = Node::Directory(Rc::new(Directory::new(Some(&current))));
                    current.children.borrow_mut().insert(name, dir);
                }
                size => {
                    let size = size
                        .parse::<usize>()
                        .or(Err("Incorrect ls output: unknown type of node."))?;
                    let name = line
                        .next()
                        .ok_or("Incorrect ls output: expecting file's name.")?
                        .to_string();
                    let file = Node::File(File { size });
                    current.children.borrow_mut().insert(name, file);
                }
            }
        } else {
            match line.next().ok_or("No command...")? {
                "cd" => match line.next().ok_or("cd command expects a name")? {
                    ".." => {
                        current = current.parent.upgrade().ok_or("No parent !")?.clone();
                    }
                    "/" => {
                        current = root.clone();
                    }
                    dir => match current.clone().children.borrow().get(dir) {
                        Some(Node::Directory(dir)) => {
                            current = dir.clone();
                        }
                        _ => return Err("cd operand is not a directory."),
                    },
                },
                "ls" => {
                    ls_output = true;
                }
                _ => {
                    return Err("Unknown command.");
                }
            }
        }
    }

    Ok(Rc::try_unwrap(root).unwrap())
}

#[derive(Debug)]
enum Node {
    File(File),
    Directory(Rc<Directory>),
}

impl Node {
    fn get_size(&self) -> usize {
        match self {
            Node::File(file) => file.size,
            Node::Directory(dir) => dir.get_size(),
        }
    }
}

#[derive(Debug)]
struct File {
    size: usize,
}

#[derive(Debug)]
struct Directory {
    children: RefCell<HashMap<String, Node>>,
    parent: Weak<Directory>,
}

impl Directory {
    fn new(parent: Option<&Rc<Self>>) -> Self {
        Self {
            parent: parent.map_or_else(Weak::new, Rc::downgrade),
            children: RefCell::new(HashMap::new()),
        }
    }
    fn get_size(&self) -> usize {
        self.children.borrow().values().map(Node::get_size).sum()
    }
}

fn print_tree(dir: &Directory, depth: usize) {
    for (name, node) in dir.children.borrow().iter() {
        match node {
            Node::File(file) => {
                println!("{:indent$}📄 {} ({})", "", name, &file.size, indent = depth);
            }
            Node::Directory(dir) => {
                println!("{:indent$}📂 {}", "", name, indent = depth);
                print_tree(dir, depth + 2);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
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
7214296 k";

    #[test]
    fn solution_p1() {
        let root = parse(INPUT.lines()).unwrap();
        let (_, solution) = solve_p1(&root);
        assert_eq!(solution, 95437);
    }

    #[test]
    fn solution_p2() {
        let root = parse(INPUT.lines()).unwrap();
        let solution = solve_p2(&root, get_missing_space(&root).unwrap())
            .1
            .unwrap();
        assert_eq!(solution, 24933642);
    }
}
