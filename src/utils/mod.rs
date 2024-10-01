use std::io::Write;

pub fn get_cwd() -> std::path::PathBuf {
    std::env::current_dir().expect("Shoud be able to read cwd")
}

pub fn write_file(path: &std::path::Path, content: &str) {
    let mut file = std::fs::File::create(path).expect("Should be able to open file");
    file.write_all(content.as_bytes())
        .unwrap_or_else(|_| panic!("Should be able to write file: {path:?}"));
}

