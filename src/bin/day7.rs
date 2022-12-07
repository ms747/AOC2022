use std::ptr::null;

#[derive(Debug)]
struct File {
    name: &'static str,
    parent: *const File,
    children: Option<Vec<File>>,
    size: usize,
}

impl File {
    fn new() -> Self {
        Self {
            name: "/",
            parent: null(),
            children: Some(vec![]),
            size: 0,
        }
    }

    fn touch(dir: *mut File, name: &'static str, size: usize) {
        let pwd = unsafe { &mut *dir };

        let file = File {
            name,
            parent: dir as *const _,
            children: None,
            size,
        };

        match &mut pwd.children {
            Some(children) => children.push(file),
            _ => unreachable!(),
        };
    }

    fn mkdir(dir: *mut File, name: &'static str) {
        let pwd = unsafe { &mut *dir };

        let dir = File {
            name,
            parent: pwd as *const _,
            children: Some(vec![]),
            size: 0,
        };

        match &mut pwd.children {
            Some(children) => children.push(dir),
            _ => unreachable!(),
        };
    }
}

#[derive(Debug)]
struct FileSystem {
    root: Box<File>,
    current_dir: *const File,
}

impl FileSystem {
    fn new(root: File) -> Self {
        let root = Box::new(root);

        Self {
            current_dir: &*root,
            root,
        }
    }

    fn touch(&mut self, name: &'static str, size: usize) {
        if self.current_dir.is_null() {
            let pwd = &mut *self.root as *mut File;
            File::touch(pwd, name, size);
        } else {
            let pwd = self.current_dir as *mut File;
            File::touch(pwd, name, size);
        }
    }

    fn mkdir(&mut self, name: &'static str) {
        if self.current_dir.is_null() {
            let pwd = &mut *self.root as *mut File;
            File::mkdir(pwd, name);
        } else {
            let pwd = self.current_dir as *mut File;
            File::mkdir(pwd, name);
        }
    }

    fn cd(&mut self, dir: &str) {
        if dir == ".." {
            let pwd = unsafe { &*self.current_dir };
            if pwd.name != "/" {
                self.current_dir = pwd.parent;
            }
            return;
        } else if dir == "/" {
            self.current_dir = &*self.root;
            return;
        }

        let pwd = self.current_dir as *mut File;
        let pwd = unsafe { &mut *pwd };

        match &mut pwd.children {
            Some(pwd) => {
                for (i, file) in pwd.iter().enumerate() {
                    if file.name == dir && file.size == 0 {
                        self.current_dir = &pwd[i];
                    }
                }
            }
            _ => unreachable!(),
        };
    }

    fn du(root: &File, sizes: &mut Vec<usize>) -> usize {
        let mut sum = 0;

        if let Some(children) = &root.children {
            for file in children {
                if file.size == 0 {
                    sum += FileSystem::du(file, sizes);
                } else {
                    sum += file.size;
                }
            }
        }

        sizes.push(sum);
        sum
    }
}

const INPUT: &str = include_str!("input7.txt");

fn main() {
    let mut fs = FileSystem::new(File::new());

    for line in INPUT.trim().lines() {
        if line.starts_with('$') {
            let cmd = &line[2..4];
            if cmd == "cd" {
                // cd
                let path = &line[5..];
                fs.cd(path);
            } else {
                // ls. Redundant.
                continue;
            }
        } else if line.starts_with('d') {
            // mkdir
            let dir = line.split(' ').nth(1).unwrap();
            fs.mkdir(dir);
        } else {
            // touch
            let (size, filename) = line.split_once(' ').unwrap();
            fs.touch(filename, size.parse().unwrap());
        }
    }

    let mut dirs = vec![];
    let total_used_space = FileSystem::du(&fs.root, &mut dirs);

    let part1: usize = dirs.iter().filter(|size| **size <= 100000).sum();

    let total_space= 70000000;
    let free_space = total_space - total_used_space;
    let part2 = dirs
        .into_iter()
        .filter(|size| free_space + size >= 30000000)
        .min()
        .unwrap();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
