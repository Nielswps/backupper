extern crate core;

use std::fs;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use tempfile;

use backupper::copy;

fn create_file(dir: &Path, file_contents: &&str, file_name: &str) {
    let file = File::create(dir.join(file_name.to_owned() + ".txt"));

    let mut source_file = match file {
        Ok(f) => f,
        Err(_) => panic!("File failed to be created"),
    };

    writeln!(source_file, "{}", &file_contents).expect("");
}

#[test]
fn copy_new_file_from_source_to_target() {
    let temp_source = tempfile::tempdir().unwrap();
    let temp_target = tempfile::tempdir().unwrap();

    let file_contents = "This is a test file";
    let file_name = "temp-file";

    let source_path = temp_source.path();
    let target_path = temp_target.path();

    create_file(source_path, &file_contents, file_name);

    match copy(source_path, target_path, vec![]) {
        Ok(_) => {}
        Err(e) => panic!("{}", e)
    }

    assert!(target_path.join(file_name.to_owned() + ".txt").exists());
    assert_eq!(read_to_string(target_path.join(file_name.to_owned() + ".txt")).unwrap(), file_contents.to_owned() + "\n");

}

#[test]
fn overwrite_file_at_same_name_and_newer_timestamp() {
    let temp_source = tempfile::tempdir().unwrap();
    let source_path = temp_source.path();

    let temp_target = tempfile::tempdir().unwrap();
    let target_path = temp_target.path();

    let original_file_contents = "This is a test file";
    let modified_file_contents = "This is a modified test file";

    let file_name = "temp-file.txt";

    create_file(target_path, &original_file_contents, file_name);
    sleep(Duration::new(1, 0));
    create_file(source_path, &modified_file_contents, file_name);

    match copy(source_path, target_path, vec![]) {
        Ok(_) => {}
        Err(e) => panic!("{}", e)
    }

    assert!(target_path.join(file_name.to_owned() + ".txt").exists());
    assert_eq!(read_to_string(source_path.join(file_name.to_owned() + ".txt")).unwrap(), modified_file_contents.to_owned() + "\n");
    assert_eq!(read_to_string(target_path.join(file_name.to_owned() + ".txt")).unwrap(), modified_file_contents.to_owned() + "\n");
}

#[test]
fn skip_file_at_same_name_and_same_timestamp() {
    let temp_source = tempfile::tempdir().unwrap();
    let source_path = temp_source.path();

    let temp_target = tempfile::tempdir().unwrap();
    let target_path = temp_target.path();

    let original_file_contents = "This is a test file";
    let modified_file_contents = "This is a modified test file";

    let file_name = "temp-file";

    create_file(target_path, &original_file_contents, file_name);
    create_file(source_path, &modified_file_contents, file_name);

    match copy(source_path, target_path, vec![]) {
        Ok(_) => {}
        Err(e) => panic!("{}", e)
    }

    assert!(target_path.join(file_name.to_owned() + ".txt").exists());
    assert_eq!(read_to_string(source_path.join(file_name.to_owned() + ".txt")).unwrap(), modified_file_contents.to_owned() + "\n");
    assert_eq!(read_to_string(target_path.join(file_name.to_owned() + ".txt")).unwrap(), original_file_contents.to_owned() + "\n");
}

#[test]
fn skip_file_at_same_name_and_older_timestamp() {
    let temp_source = tempfile::tempdir().unwrap();
    let source_path = temp_source.path();

    let temp_target = tempfile::tempdir().unwrap();
    let target_path = temp_target.path();

    let original_file_contents = "This is a test file";
    let modified_file_contents = "This is a modified test file";

    let file_name = "temp-file";

    create_file(source_path, &modified_file_contents, file_name);

    sleep(Duration::new(1, 0));

    create_file(target_path, &original_file_contents, file_name);

    match copy(source_path, target_path, vec![]) {
        Ok(_) => {}
        Err(e) => panic!("{}", e)
    }

    assert!(target_path.join(file_name.to_owned() + ".txt").exists());
    assert_eq!(read_to_string(source_path.join(file_name.to_owned() + ".txt")).unwrap(), modified_file_contents.to_owned() + "\n");
    assert_eq!(read_to_string(target_path.join(file_name.to_owned() + ".txt")).unwrap(), original_file_contents.to_owned() + "\n");
}

#[test]
fn copy_dir_from_source_to_target() {
    let temp_source = tempfile::tempdir().unwrap();
    let source_path = temp_source.path();

    let temp_target = tempfile::tempdir().unwrap();
    let target_path = temp_target.path();

    fs::create_dir_all(source_path.join("subDir")).expect("");

    let file_contents = "This is a test file";
    let file_name = "temp-file";

    let subdir = source_path.join("subDir");

    create_file(subdir.as_path(), &file_contents, file_name);

    match copy(source_path, target_path, vec![]) {
        Ok(_) => {}
        Err(e) => panic!("{}", e)
    }

    assert!(target_path.join("subDir").join(file_name.to_owned() + ".txt").exists());
    assert_eq!(read_to_string(target_path.join("subDir").join(file_name.to_owned() + ".txt")).unwrap(), file_contents.to_owned() + "\n");
}

#[test]
fn overwrite_part_of_dir_and_skip_rest_based_on_timestamps() {
    let temp_source = tempfile::tempdir().unwrap();
    let source_path = temp_source.path();

    let temp_target = tempfile::tempdir().unwrap();
    let target_path = temp_target.path();

    fs::create_dir_all(source_path.join("subDir")).expect("");
    fs::create_dir_all(target_path.join("subDir")).expect("");

    let original_file_contents = "This is a test file";
    let modified_file_contents = "This is a modified test file";
    let file_1_name = "temp-file-1";
    let file_2_name = "temp-file-2";

    create_file(source_path.join("subDir").as_path(), &original_file_contents, file_1_name);
    create_file(target_path.join("subDir").as_path(), &modified_file_contents, file_1_name);
    create_file(target_path.join("subDir").as_path(), &original_file_contents, file_2_name);

    sleep(Duration::new(1, 0));

    create_file(source_path.join("subDir").as_path(), &modified_file_contents, file_2_name);

    match copy(source_path, target_path, vec![]) {
        Ok(_) => {}
        Err(e) => panic!("{}", e)
    }

    assert!(target_path.join("subDir").join(file_1_name.to_owned() + ".txt").exists());
    assert!(target_path.join("subDir").join(file_2_name.to_owned() + ".txt").exists());
    assert_eq!(read_to_string(target_path.join("subDir").join(file_1_name.to_owned() + ".txt")).unwrap(), modified_file_contents.to_owned() + "\n");
    assert_eq!(read_to_string(target_path.join("subDir").join(file_2_name.to_owned() + ".txt")).unwrap(), modified_file_contents.to_owned() + "\n");
}

#[test]
fn copy_dir_but_ignored_blacklist() {
    let temp_source = tempfile::tempdir().unwrap();
    let source_path = temp_source.path();

    let temp_target = tempfile::tempdir().unwrap();
    let target_path = temp_target.path();

    let sub_dir = source_path.join("subDir");
    let blacklisted_dir = source_path.join("blacklisted_dir");

    fs::create_dir_all(&sub_dir).expect("");
    fs::create_dir_all(&blacklisted_dir).expect("");

    let file_contents = "This is a test file";
    let file_name = "temp-file";

    create_file(sub_dir.as_path(), &file_contents, file_name);
    create_file(blacklisted_dir.as_path(), &file_contents, "ignoredFile");

    match copy(source_path, target_path, vec![String::from("blacklisted_dir")]) {
        Ok(_) => {}
        Err(e) => panic!("{}", e)
    }

    let target_file = target_path.join("subDir").join(file_name.to_owned() + ".txt");
    let ignored_target_file = target_path.join("blacklisted_dir").join("ignoredFile.txt");

    assert!(target_file.exists());
    assert!(!ignored_target_file.exists());
    assert_eq!(read_to_string(target_file).unwrap(), file_contents.to_owned() + "\n");
}

#[test]
fn copy_dir_and_file_nested_in_hierarchy() {
    let temp_source = tempfile::tempdir().unwrap();
    let source_path = temp_source.path();

    let temp_target = tempfile::tempdir().unwrap();
    let target_path = temp_target.path();

    let sub_dir = source_path.join("subDir");
    let sub_sub_dir = source_path.join("subDir").join("subSubDir");

    fs::create_dir_all(&sub_dir).expect("");
    fs::create_dir_all(&sub_sub_dir).expect("");

    let file_contents = "This is a test file";
    let file_name = "temp-file";

    create_file(sub_dir.as_path(), &file_contents, &(file_name.to_owned() + "-1"));
    create_file(sub_sub_dir.as_path(), &file_contents, &(file_name.to_owned() + "-2"));

    match copy(source_path, target_path, vec![String::from("blacklisted_dir"), String::from("blacklisted_dir_2")]) {
        Ok(_) => {}
        Err(e) => panic!("{}", e)
    }

    let target_file_1 = target_path.join("subDir").join(file_name.to_owned() + "-1.txt");
    let target_file_2 = target_path.join("subDir").join("subSubDir").join(file_name.to_owned() + "-2.txt");

    assert!(target_file_1.exists());
    assert!(target_file_2.exists());
    assert_eq!(read_to_string(target_file_1).unwrap(), file_contents.to_owned() + "\n");
    assert_eq!(read_to_string(target_file_2).unwrap(), file_contents.to_owned() + "\n");
}