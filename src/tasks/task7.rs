use crate::task_handler::get_task;

pub fn tasks() {
    let input = get_task(7);
    println!("{}", task1(&input));
    println!("{}", task2(&input));
}

pub fn task1(input: &str) -> usize {
    let mut tree = Tree::new();
    input.lines().skip(1).for_each(|x| tree.take_line(x));
    tree.sum_nodes();
    tree.dirs
        .iter()
        .filter(|&x| x.sum < 100_000)
        .fold(0, |sum, dir| sum + dir.sum)
}

pub fn task2(input: &str) -> usize {
    let mut tree = Tree::new();
    input.lines().skip(1).for_each(|x| tree.take_line(x));
    tree.sum_nodes();
    let goal = 30_000_000;
    let total = 70_000_000;
    let diff = goal - (total - tree.dirs[0].sum);
    tree.dirs.iter().filter(|x| x.sum > diff).min().unwrap().sum
}

#[derive(Eq, PartialEq)]
struct Dir {
    name: String,
    // files: Vec<(usize, String)>,
    parent: Option<usize>,
    children: Vec<usize>,
    sum: usize,
}

impl Dir {
    pub fn new(input: &str, parent: Option<usize>) -> Self {
        Self {
            name: input.split_once(' ').unwrap().1.to_string(),
            // files: Vec::new(),
            parent,
            children: Vec::new(),
            sum: 0,
        }
    }
}

impl Ord for Dir {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.sum.cmp(&other.sum)
    }
}

impl PartialOrd for Dir {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.sum.partial_cmp(&other.sum)
    }
}

struct Tree {
    pub dirs: Vec<Dir>,
    curr_node: usize,
}

impl Tree {
    fn new() -> Self {
        Self {
            dirs: vec![Dir::new("dir /", None)],
            curr_node: 0,
        }
    }

    pub fn take_line(&mut self, str: &str) {
        if str.starts_with('$') {
            match str.rsplit_once(' ') {
                Some(("$ cd", "..")) => self.curr_node = self.dirs[self.curr_node].parent.unwrap(),
                Some(("$ cd", dir)) => {
                    self.curr_node = *self.dirs[self.curr_node]
                        .children
                        .iter()
                        .find(|&&x| self.dirs[x].name == dir)
                        .unwrap();
                }
                _ => (),
            }
        } else if str.starts_with("dir") {
            let len = self.dirs.len();
            self.dirs[self.curr_node].children.push(len);
            self.dirs.push(Dir::new(str, Some(self.curr_node)));
        } else {
            let x = str.split_once(' ').unwrap();
            self.dirs[self.curr_node].sum += x.0.parse::<usize>().unwrap();
        }
    }

    fn get_parent(&mut self) -> Option<&mut Dir> {
        self.dirs
            .get(self.curr_node)
            .and_then(|p| p.parent)
            .and_then(|i| self.dirs.get_mut(i))
    }

    pub fn sum_nodes(&mut self) {
        for i in (0..self.dirs.len()).rev() {
            self.curr_node = i;
            let sum = self.dirs[i].sum;
            if let Some(dir) = self.get_parent() {
                dir.sum += sum;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{task1, task2};

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
    fn test_task1() {
        assert_eq!(task1(INPUT), 95437);
    }

    #[test]
    fn test_task2() {
        assert_eq!(task2(INPUT), 24_933_642);
    }
}
