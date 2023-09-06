use std::{
    collections::HashMap,
    io::Error,
    path::{Path, PathBuf},
};
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

    let mut database =
        Database::new(path).expect(format!("Database::new({:#?}) crashed", filename).as_str());
    println!("Initial Database is: {:?}", database);
    // database.insert(key.to_uppercase(), value.clone());
    database.insert(key.to_lowercase(), value.clone());
    // database.save(filename);
    println!("Intermediate Database is: {:?}", database);
    // database.flush().unwrap();
    // If flush() is called above, The following will not compile because flush(self) moves the database into the function scope
    database.insert(key.to_uppercase(), value);
    println!("Final Database is: {:?}", database);
}

#[derive(Debug)]
struct Database {
    path: PathBuf,
    map: HashMap<String, String>,
    flushed: bool,
}

impl Database {
    fn new(db_filepath: PathBuf) -> Result<Database, Error> {
        let mut map: HashMap<String, String> = HashMap::new();
        // read the DB file
        if !db_filepath.exists() {
            let _maybe_parent_dir = db_filepath.parent();
            let parent_dir = _maybe_parent_dir.unwrap_or_else(|| Path::new("."));
            if !parent_dir.exists() {
                std::fs::create_dir_all(parent_dir)?;
            } //else if parent_dir.exists() && !parent_dir.is_dir() {
              // return std::io::Error
              // Err(format!("Path exists: {:?}", parent_dir.to_str()));
              // }
        } else {
            let contents = std::fs::read_to_string(&db_filepath)?;

            // parse the string
            for line in contents.lines() {
                let (key, value) = line.split_once('\t').expect("Corrupt database");
                map.insert(key.to_owned(), value.to_owned());
            }
        }

        // populate our map
        Ok(Database {
            path: db_filepath,
            map,
            flushed: false,
        })
    }

    // fn insert() -> Result<(), Error> {
    fn insert(&mut self, key: String, value: String) -> () {
        self.map.insert(key, value);
    }

    fn save(&self, filename: &str) -> Result<(), Error> {
        todo!("Implement Database.save(): Save and write Database to file.");
        // Original main() implementation:
        // if !path.exists() {
        //     let contents = format!("{}\t{}\n", key, value);
        //     let result = std::fs::write(path, contents);
        //     match result {
        //         Ok(()) => {
        //             println!("Successfully wrote to db file: {filename}\n");
        //             // TODO: return Result(Ok());
        //             // std::process::exit(0);
        //         }
        //         Err(e) => {
        //             return std::io::Error
        //             // panic!("Error writing to db file: {filename}\n{e}");
        //         }
        //     }
        // }
    }

    fn flush(mut self) -> std::io::Result<()> {
        self.flushed = true;
        do_flush(&self)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.flushed {
            let _ = do_flush(&self);
        }
        // let _ = self.flush();
    }
}

fn do_flush(database: &Database) -> std::io::Result<()> {
    println!("do_flush() called");
    let mut contents = String::new();
    for (key, value) in &database.map {
        // let kvpair = format!("{}\t{}\n", key, value);
        // contents.push_str(&kvpair);
        // contents += &kvpair;
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');
    }
    std::fs::write(&database.path, contents)
}
