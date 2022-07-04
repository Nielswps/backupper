use backupper;
use config::Config;

fn main() {
    let settings = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .unwrap();

    let src_dir: String = match settings.get("src_dir") {
        Ok(v) => v,
        Err(_) => panic!("Unable to load src_dir from config file, make sure to have config.toml and src_dir as a valid key")
    };

    let dst_dir: String = match settings.get("dst_dir") {
        Ok(v) => v,
        Err(_) => panic!("Unable to load dst_dir from config file, make sure to have config.toml and dst_dir as a valid key")
    };

    let blacklist_tmp = match settings.get_array("blacklist") {
        Ok(v) => v,
        Err(_) => panic!("Unable to load blacklist from config file, make sure to have config.toml and blacklist as a valid key")
    };
    let blacklist: Vec<String> = blacklist_tmp.iter().map(|x| x.to_string()).collect();

    match backupper::copy(src_dir, dst_dir, blacklist) {
        Ok(()) => println!("--- Copy of files completed! ---"),
        Err(error) => panic!("The following error occurred while backing up: {:?}", error),
    };
}
