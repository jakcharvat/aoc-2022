use std::{collections::BTreeMap, io::stdin};

type INode = usize;

type DirChildren = BTreeMap<String, INode>;

#[derive(variantly::Variantly)]
enum FileKind {
    Regular,
    Dir(DirChildren),
}

struct File {
    parent: Option<INode>,
    size: usize,
    kind: FileKind,
}

struct Fs {
    files: Vec<File>,
    curr_file: INode,
}

const FS_CAPACITY: usize = 70000000;
const FS_UPDATE_NEEDED: usize = 30000000;
const ROOT_INODE: INode = 0;

impl Fs {
    pub fn new() -> Fs {
        Fs {
            files: vec![File {
                parent: None,
                size: 0,
                kind: FileKind::Dir(DirChildren::new()),
            }],
            curr_file: ROOT_INODE,
        }
    }

    fn get_cwd_children(&self) -> &DirChildren {
        self.files[self.curr_file].kind.dir_ref().unwrap()
    }

    fn get_cwd_children_mut(&mut self) -> &mut DirChildren {
        self.files[self.curr_file].kind.dir_mut().unwrap()
    }

    pub fn cd(&mut self, to_dir: &str) {
        let inode = match to_dir {
            "/" => ROOT_INODE,
            ".." => self.files[self.curr_file].parent.unwrap(),
            name => *self.get_cwd_children().get(name).unwrap(),
        };

        self.curr_file = inode;
    }

    fn mkdir(&mut self, dir_name: String) {
        let id = self.files.len();
        self.files.push(File {
            parent: Some(self.curr_file),
            size: 0,
            kind: FileKind::Dir(DirChildren::new()),
        });

        assert!(self.get_cwd_children_mut().insert(dir_name, id).is_none());
    }

    fn touch(&mut self, file_name: String, size: usize) {
        let id = self.files.len();
        self.files.push(File {
            parent: Some(self.curr_file),
            size,
            kind: FileKind::Regular,
        });

        assert!(self.get_cwd_children_mut().insert(file_name, id).is_none());
    }

    pub fn ls_line(&mut self, line: &str) {
        let split = line.split(" ").collect::<Vec<_>>();
        let name = split[1].to_owned();

        if split[0] == "dir" {
            self.mkdir(name);
        } else {
            let size = split[0].parse().unwrap();
            self.touch(name, size);
        }
    }

    pub fn recalculate_sizes(&mut self) {
        self.recalculate_sizes_rec(ROOT_INODE);
    }

    fn recalculate_sizes_rec(&mut self, inode: usize) -> usize {
        match &self.files[inode].kind {
            FileKind::Regular => self.files[inode].size,
            FileKind::Dir(children) => {
                let child_ids = children.iter().map(|(_, child)| *child).collect::<Vec<_>>();
                let size: usize = child_ids
                    .into_iter()
                    .map(|id| self.recalculate_sizes_rec(id))
                    .sum();

                self.files[inode].size = size;
                size
            }
        }
    }
}

fn main() {
    let mut lines = stdin().lines().map(|l| l.unwrap()).peekable();
    let mut fs = Fs::new();

    while let Some(line) = lines.next() {
        let command = line
            .strip_prefix("$ ")
            .unwrap()
            .split(" ")
            .collect::<Vec<_>>();

        match command[0] {
            "cd" => fs.cd(command[1]),

            "ls" => {
                while lines.peek().map(|p| !p.starts_with("$")) == Some(true) {
                    let line = lines.next().unwrap();
                    fs.ls_line(&line);
                }
            }

            d => panic!("Unknown command '{}'", d),
        }
    }

    fs.recalculate_sizes();

    println!(
        "Part 1: {}",
        fs.files
            .iter()
            .filter(|f| f.kind.is_dir() && f.size <= 100000)
            .map(|f| f.size)
            .sum::<usize>()
    );

    let free = FS_CAPACITY - fs.files[ROOT_INODE].size;
    let need = FS_UPDATE_NEEDED - free;

    println!(
        "Part 2: {}",
        fs.files
            .iter()
            .filter(|f| f.kind.is_dir() && f.size >= need)
            .min_by_key(|d| d.size)
            .unwrap()
            .size
    )
}
