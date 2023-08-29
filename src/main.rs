use std::{collections::HashMap, io::Error, path::PathBuf};
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

    let database = Database::new(path.into());
    // let result = std::fs::write(path, contents);
    // match result {
    //     Ok(()) => {
    //         println!("Successfully wrote to db file: {path}\n");
    //         std::process::exit(0);
    //     }
    //     Err(e) => {
    //         panic!("Error writing to db file: {path}\n{e}");
    //     }
    // }
}

struct Database {
    path: PathBuf,
    map: HashMap<String, String>,
}

impl Database {
    fn new(db_filepath: PathBuf) -> Result<Database, Error> {
        // read the DB file
        let contents = match std::fs::read_to_string(&db_filepath) {
            Ok(c) => c,
            Err(e) => {
                return Err(e);
            }
        };
        // parse the string
        // populate our map
        Database {
            path: db_filepath,
            map: HashMap::new(),
        }
    }

    // fn insert() -> Result<(), Error> {

    // }
}
