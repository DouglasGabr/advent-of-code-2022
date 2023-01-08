use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
};

#[derive(Debug, Default)]
struct Directory<'a> {
    total_size: u32,
    directories: HashMap<&'a str, Rc<RefCell<Directory<'a>>>>,
    files: HashMap<&'a str, u32>,
}

#[derive(Debug)]
struct Pwd<'a> {
    path: Vec<Rc<RefCell<Directory<'a>>>>,
}

impl<'a> Pwd<'a> {
    fn new(root: Rc<RefCell<Directory<'a>>>) -> Self {
        Pwd {
            path: vec![root.clone()],
        }
    }

    fn navigate_to_root(&mut self) {
        let root = self.path[0].clone();
        self.path = vec![root];
    }

    fn navigate_up(&mut self) {
        self.path.pop();
    }

    fn navigate_into_dir(&mut self, sub_directory_name: &'a str) {
        let sub_directory = self.ensure_directory(sub_directory_name);
        self.path.push(sub_directory);
    }

    fn ensure_directory(&mut self, sub_directory_name: &'a str) -> Rc<RefCell<Directory<'a>>> {
        let current_directory = self.path.last().expect("pwd should not be empty, ever");
        let directories = &mut current_directory.borrow_mut().directories;
        directories.entry(sub_directory_name).or_default().clone()
    }

    fn ensure_file(&mut self, file_name: &'a str, file_size: u32) {
        let size_to_add_to_dirs = {
            let current_directory = self.path.last().expect("pwd should not be empty, ever");
            let files = &mut current_directory.borrow_mut().files;
            match files.entry(file_name) {
                Entry::Vacant(entry) => *entry.insert(file_size),
                _ => 0,
            }
        };
        for dir in self.path.iter() {
            dir.borrow_mut().total_size += size_to_add_to_dirs;
        }
    }

    fn process_input(&mut self, input: &'a str) {
        for line in input.lines() {
            if line.starts_with('$') {
                if line.contains("$ ls") {
                    continue;
                } else if line.contains("$ cd") {
                    let target_directory = &line[5..];
                    match target_directory {
                        "/" => {
                            self.navigate_to_root();
                        }
                        ".." => {
                            self.navigate_up();
                        }
                        sub_directory_name => {
                            self.navigate_into_dir(sub_directory_name);
                        }
                    }
                }
            } else {
                let parts = line.split_once(' ').expect("whitespace must exist");
                match parts {
                    ("dir", dir_name) => {
                        self.ensure_directory(dir_name);
                    }
                    (file_size, file_name) => {
                        let parsed_size: u32 = file_size.parse().expect("file size is valid u32");
                        self.ensure_file(file_name.trim(), parsed_size);
                    }
                }
            }
        }
    }
}

fn part1(input: &str) -> u32 {
    let root = Rc::new(RefCell::new(Directory::default()));

    let mut pwd = Pwd::new(root.clone());
    pwd.process_input(input);

    fn process_dir(dir: &Directory, sum_of_dirs_with_size_lte_100k: &mut u32) {
        if dir.total_size <= 100_000 {
            *sum_of_dirs_with_size_lte_100k += dir.total_size;
        }
        for (_, sub_dir) in dir.directories.iter() {
            process_dir(&sub_dir.borrow(), sum_of_dirs_with_size_lte_100k);
        }
    }

    let mut sum_of_dirs_with_size_lte_100k = 0;
    process_dir(&root.borrow(), &mut sum_of_dirs_with_size_lte_100k);

    sum_of_dirs_with_size_lte_100k
}

const TOTAL_DISK_SIZE: u32 = 70_000_000;
const NEEDED_UNUSED_SPACE: u32 = 30_000_000;

fn part2(input: &str) -> u32 {
    let root = Rc::new(RefCell::new(Directory::default()));

    let mut pwd = Pwd::new(root.clone());
    pwd.process_input(input);

    let free_space = TOTAL_DISK_SIZE - root.borrow().total_size;
    let missing_space = NEEDED_UNUSED_SPACE - free_space;

    fn process_dir(dir: &Directory, missing_space: u32, valid_sizes: &mut Vec<u32>) {
        if dir.total_size >= missing_space {
            valid_sizes.push(dir.total_size);
        }
        for (_, sub_dir) in dir.directories.iter() {
            process_dir(&sub_dir.borrow(), missing_space, valid_sizes);
        }
    }

    let mut valid_sizes = Vec::new();
    process_dir(&root.borrow(), missing_space, &mut valid_sizes);

    *valid_sizes
        .iter()
        .min()
        .expect("some directory should be valid")
}

fn main() {
    let test_input = include_str!("../input/day7.test");
    let prod_input = include_str!("../input/day7.prod");

    let test_part1_result = part1(test_input);
    println!("part 1 test: {}", test_part1_result);

    let prod_part1_result = part1(prod_input);
    println!("part 1 prod: {}", prod_part1_result);

    let test_part2_result = part2(test_input);
    println!("part 2 test: {:?}", test_part2_result);

    let prod_part2_result = part2(prod_input);
    println!("part 2 prod: {:?}", prod_part2_result);
}
