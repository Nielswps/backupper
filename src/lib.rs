use std::cmp::Ordering;
use std::fs;
use std::path::{Path, PathBuf};

pub fn copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V, blacklist: Vec<String>) -> Result<(), std::io::Error> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));

    let input_root_size = PathBuf::from(from.as_ref()).components().count();
    let output_root = PathBuf::from(to.as_ref());

    while let Some(working_path) = stack.pop() {
        if  blacklist.iter().any(|dir| working_path.ends_with(dir)) {
            continue;
        }

        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root_size).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            println!(" mkdir: {:?}", dest);
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        let source_time = fs::metadata(&path)?.modified()?;

                        if dest_path.exists() {
                            if source_time.cmp(&fs::metadata(&dest_path)?.modified()?).eq(&Ordering::Greater) {
                                println!("  overwrote: {:?} with {:?}", &dest_path, &path);
                                fs::copy(&path, &dest_path)?;
                            }
                        } else {
                            println!("  created: {:?} from {:?}", &dest_path, &path);
                            fs::copy(&path, &dest_path)?;
                        }
                    }
                    None => {
                        println!("failed: {:?}", path);
                    }
                }
            }
        }
    }

    Ok(())
}