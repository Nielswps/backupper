use std::{fs};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub fn copy<U: AsRef<Path>, V: AsRef<Path>>(
    from: U,
    to: V,
    blacklist: Vec<String>,
) -> Result<(), std::io::Error> {
    let mut stack = vec![PathBuf::from(from.as_ref())];
    let input_root_size = PathBuf::from(from.as_ref()).components().count();
    let output_root = PathBuf::from(to.as_ref());

    while let Some(working_path) = stack.pop() {
        if blacklist.iter().any(|dir| working_path.ends_with(dir)) {
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
            println!(" mkdir: {:?}", &dest);
            fs::create_dir_all(&dest)?;
        }

        let dir_content = match fs::read_dir(&working_path) {
            Ok(content) => content,
            Err(_) => {
                println!("Unable to read dir {:?}", &working_path);
                continue;
            }
        };
        for entry in dir_content {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    println!("Retrieving directory entry resulted in error: {:?}", &e);
                    continue;
                }
            };

            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);

                        let source_time = match get_last_modified(&path) {
                            Ok(time) => time,
                            Err(e) => {
                                println!("Retrieving metadata of file: {:?}, resulted in: {:?}", &path, &e);
                                continue;
                            }
                        };

                        if dest_path.exists() {
                            let dest_time = match get_last_modified(&dest_path) {
                                Ok(time) => time,
                                Err(e) => {
                                    println!("Retrieving metadata of file: {:?}, resulted in: {:?}", &path, &e);
                                    continue;
                                }
                            };

                            if source_time.cmp(&dest_time).is_le()
                            {
                                continue;
                            } else {
                                println!("  overwrote: {:?} with {:?}", &dest_path, &path);
                            }
                        } else {
                            println!("  created: {:?} from {:?}", &dest_path, &path);
                        }

                        match fs::copy(&path, &dest_path) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Encountered error: {:?}, trying to copy: {:?}", &e, &path);
                                continue;
                            }
                        };
                    }
                    None => {
                        println!("Unable to process file: {:?}", &path);
                    }
                }
            }
        }
    }

    Ok(())
}

fn get_last_modified(path: &PathBuf) -> Result<SystemTime, std::io::Error> {
    let metadata = match fs::metadata(&path) {
        Ok(data) => data,
        Err(e) => return Err(e)
    };

    let time = match metadata.modified() {
        Ok(mod_time) => mod_time,
        Err(e) => return Err(e)
    };
    Ok(time)
}
