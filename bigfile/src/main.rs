use std::cmp::Ordering;
use std::fs::{self};
use std::path::Path;

fn main() {
    let mut fs_entries: Vec<FsEntryInfo> = Vec::new();
    let dir_size = visit_entry(Path::new("."), &mut fs_entries);
    fs_entries.sort();
    let max_path_len = find_max_path_len(&fs_entries);
    let max_size_len = dir_size.to_string().len();

    println!("Entries");
    for i in fs_entries.iter().rev() {
        println!("{} {:path_len$} {:size_len$} ({:size_len$}) ({:5.2}%)",
                 if i.is_dir { "Dir: " } else { "File:" },
                 path_to_str(&i.path.to_path_buf()),
                 i.significant_size,
                 i.full_size,
                 (i.significant_size as f64 / dir_size as f64) * 100 as f64,
                 path_len = max_path_len,
                 size_len = max_size_len
        );
    }
}

fn visit_entry(dir: &Path, fs_entries: &mut Vec<FsEntryInfo>) -> u64 {
    let mut full_size: u64 = 0;
    let mut significant_size: u64 = 0;
    let mut largest_file_size: u64 = 0;
    let mut all_files_size: u64 = 0;
    if dir.is_dir() {
        if let Ok(dir_iter) = fs::read_dir(dir) {
            for entry in dir_iter {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_dir() {
                            let subdir_size = visit_entry(&path, fs_entries);
                            full_size = full_size + subdir_size;
                        } else {
                            if let Ok(metadata) = path.metadata() {
                                let file_size = metadata.len();
                                full_size = full_size + file_size;
                                all_files_size = all_files_size + file_size;
                                if file_size > largest_file_size {
                                    significant_size = significant_size + largest_file_size;
                                    largest_file_size = file_size;
                                } else {
                                    significant_size = significant_size + file_size;
                                }
                                fs_entries.push(FsEntryInfo {
                                    is_dir: false,
                                    path: path.to_path_buf(),
                                    full_size: file_size,
                                    significant_size: file_size,
                                });
                            }
                        }
                    }
                    Err(error) => {
                        eprintln!("Error while reading entries in dir {}: {}",
                                  path_to_str(&dir.to_path_buf()), error);
                    }
                }
            }

            //if largest file of the dir is less than 80% of the whole dir
            if (largest_file_size as f64) < (all_files_size as f64 * 0.8) {
                significant_size = significant_size + largest_file_size;
            }
        }

        fs_entries.push(FsEntryInfo {
            is_dir: true,
            path: dir.to_path_buf(),
            full_size,
            significant_size });
    }
    full_size
}

fn find_max_path_len(entries: &Vec<FsEntryInfo>) -> usize {
    entries.iter()
        .map(|entry| path_to_str(&entry.path).len())
        .max()
        .unwrap_or(100)
}

fn path_to_str(path: &std::path::PathBuf) -> &str {
    path.to_str().expect("Can't convert path to string")
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct FsEntryInfo {
    is_dir: bool,
    path: std::path::PathBuf,
    full_size: u64,
    significant_size: u64,
}

impl PartialOrd for FsEntryInfo {
    fn partial_cmp(&self, other: &FsEntryInfo) -> Option<Ordering> {
        let mut result = self.significant_size.cmp(&other.significant_size);

        if let Ordering::Equal = result {
            result = self.full_size.cmp(&other.full_size);
        }

        if let Ordering::Equal = result {
            result = self.path.cmp(&other.path);
        }

        if let Ordering::Equal = result {
            Option::None
        } else {
            Option::Some(result)
        }
    }
}