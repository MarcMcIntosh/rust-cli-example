use std::{collections::HashSet, fs::ReadDir, os::unix::ffi::OsStrExt, path::PathBuf};

#[derive(Debug, Clone, Default)]
struct Entity {
    root: PathBuf
}

impl Entity {
    // Could have options?
    fn read_dir_entry(&self, dir: ReadDir) -> Vec<Entity> {
        dir.into_iter().filter_map(|d| {
            match d {
                Err(_) => None,
                Ok(dir_entry) => Some(Entity::from(dir_entry.path())),
            }
        }).collect()
    }
    pub fn children(&self) -> Vec<Entity> {
        if self.root.is_dir() == false {
            return Vec::<Entity>::new();
        }
        match self.root.read_dir() {
            Err(_) => Vec::<Entity>::new(),
            Ok(read_dir) => self.read_dir_entry(read_dir)
        }
    }
}

impl Iterator for Entity {
    type Item = Vec<Entity>;
    fn next(&mut self) -> Option<Self::Item> {
        let children= self.children();
        if children.is_empty() {
            return None
        }
        return Some(children)
    }
}

// TODO: Into<PathBuf>

impl From<PathBuf> for Entity {
    fn from(value: PathBuf) -> Self {
        Entity { root: value }
    }
}


impl Entity {
    pub fn new() -> Self { Entity::default() }
}


fn concat<T>(this: T, that: impl IntoIterator<Item = T::Item>) -> T 
where T: IntoIterator + FromIterator<T::Item> {
    T::from_iter(this.into_iter().chain(that.into_iter()))
}



// TODO: figure out how to do this on windows :/
// https://rust-lang.github.io/rustup/cross-compilation.html
fn is_hidden(path: PathBuf) -> bool {
    path
      .file_prefix()
      .is_some_and(|file_name| {
        file_name.as_bytes().first() == Some(&b'.')
      })
}


// TODO: would pointers be more efficient for seen and results?
fn list_path_recursive(
    path: PathBuf,
    depth: u32,
    all: bool,
    seen: HashSet<PathBuf>,
) -> Vec<PathBuf>{

    if all == false && is_hidden(path.clone()) {
        return Vec::<PathBuf>::new();
    } else if seen.contains(&path) {
        return Vec::<PathBuf>::new();
    } else if depth == 0 || path.is_file() {
        return Vec::from([path]);
    }

    let root = Entity::from(path.clone());
    let next_seen = concat(seen, [root.root.clone()]);

    let children = root.children()
    .into_iter()
    .flat_map(|entity| {
       list_path_recursive(entity.root, depth -1, all, next_seen.clone())
    }).collect::<Vec<PathBuf>>();

    return  children;
}


// TBD: support stdout / stderr pipe?
pub fn ls(path: PathBuf, recurse: bool, all: bool) -> Vec<PathBuf> {
    let depth = if recurse {u32::MAX} else {1};
    let seen = HashSet::<PathBuf>::new();
    list_path_recursive(path, depth, all, seen)
}








