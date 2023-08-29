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
    let filename = "/tmp/kvstore.db";
    let path = PathBuf::new().with_file_name(filename);

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
    let database = Database::new(path).expect(format!("Database::new(\"{:#?}\") crashed", filename).as_str());
}

struct Database {
    path: PathBuf,
    map: HashMap<String, String>,
}

impl Database {
    fn new(db_filepath: PathBuf) -> Result<Database, Error> {
        // read the DB file
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
