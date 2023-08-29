use std::{collections::HashMap, io::Error, path::PathBuf, hash::Hash};
fn main() {
    let mut argv = std::env::args().skip(1);
    let key = argv
        .next()
        .expect("No key passed!  Did you forget to pass args: key value");
    let value = argv
        .next()
        .expect("No value passed! Did you forget to pass args: key value");
    println!("key: {} = {}", key, value);
    let path = "/tmp/kvstore.db";

    // let contents = format!("{}\t{}\n", key, value);
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
    let database = Database::new(path.into()).expect(format!("Database::new(\"{path}\") crashed").as_str());
}

struct Database {
    path: PathBuf,
    map: HashMap<String, String>,
}

impl Database {
    fn new(db_filepath: PathBuf) -> Result<Database, Error> {
        // read the DB file
        // let contents = match std::fs::read_to_string(&db_filepath) {
        //     Ok(c) => c,
        //     Err(e) => {
        //         return Err(e);
        //     }
        // };
        let mut m: HashMap<String, String> = HashMap::new();
        let contents = std::fs::read_to_string(&db_filepath)?;
        // parse the string
        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupt database");
            m.insert(key.to_owned(), value.to_owned());
        }
        // populate our map
        Ok(Database {
            path: db_filepath,
            map: m,
        })
    }

    // fn insert() -> Result<(), Error> {

    // }
}
