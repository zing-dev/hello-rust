pub mod dir_entry {
    use std::fs::read_dir;
    use std::{io, result};

    #[test]
    fn path() {
        let mut dir = read_dir(".").unwrap();
        let path = dir.next().unwrap().unwrap().path();
        println!("{:?}", path)
    }

    #[test]
    fn metadata() {
        if let Ok(entries) = read_dir(".") {
            for entry in entries {
                if let Ok(entry) = entry {
                    // Here, `entry` is a `DirEntry`.
                    if let Ok(metadata) = entry.metadata() {
                        // Now let's show our entry's permissions!
                        println!(
                            "{:?}: {:?},{:?}",
                            entry.path(),
                            metadata.permissions(),
                            metadata.len()
                        );
                    } else {
                        println!("Couldn't get metadata for {:?}", entry.path());
                    }
                }
            }
        }
    }

    #[test]
    fn file_name() -> io::Result<()> {
        read_dir(".")?
            .map(|res| res.map(|entry| println!("{:?}", { entry.file_name() })))
            .collect::<result::Result<(), std::io::Error>>();
        Ok(())
    }
}
