use std::{path::Path, fs::File, io::Read};

pub fn read_file_contents(file_path: &str) -> String {
    let path = Path::new(file_path);
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("unable to open {}: {}", path.display(), why),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(why) => panic!("unable to read {}: {}", path.display(), why),
    };
    contents
}
