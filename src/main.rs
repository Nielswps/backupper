use backup;

fn main() {
    const SOURCE: &str = "/home/niels/Documents";
    const TARGET: &str = "/media/niels/TOSHIBA EXT";
    const BLACKLIST: &'static [&'static str] = &["gitHub"];

    let copy_result = backup::copy(SOURCE, TARGET, BLACKLIST);
    match copy_result {
        Ok(()) => println!("--- Copy of files completed! ---"),
        Err(error) => panic!("The following error occurred while backing up: {:?}", error),
    };
}
