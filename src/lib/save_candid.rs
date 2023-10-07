pub fn save_candid(candid: String, name: String) {
    use std::env;
    use std::fs::write;
    use std::path::PathBuf;

    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let dir = dir.parent().unwrap().parent().unwrap().parent().unwrap().join("candid");
    write(dir.join(format!("{}.did", name)), candid).expect("Write failed.");
}
