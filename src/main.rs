fn main() {
    let mut argv = std::env::args().skip(1);
    let key = argv
        .next()
        .expect("No key passed!  Did you forget to pass args: key value");
    let value = argv
        .next()
        .expect("No value passed! Did you forget to pass args: key value");
    println!("key: {} = {}", key, value);
    let contents = format!("{}\t{}\n", key, value);
    let path = "/tmp/kvstore.db";
    let result = std::fs::write(path, contents);
    match result {
        Ok(()) => {
            println!("Successfully wrote to db file: {path}\n");
            std::process::exit(0);
        }
        Err(e) => {
            panic!("Error writing to db file: {path}\n{e}");
        }
    }
}

}
