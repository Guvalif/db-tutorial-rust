use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::path::Path;
use std::ptr::null;

use row::Row;
use table::TABLE_MAX_PAGES;

struct Pager {
    file: File,
    file_length: u64,
    pages: [*const Row; TABLE_MAX_PAGES],
}

impl Pager {
    pub fn open(file_name: &str) -> Pager {
        let path = Path::new(file_name);

        let mut file = File::open(&path)
            .unwrap_or_else(|_| panic!("Error: Unable to open file '{}'!", path.display()));

        let file_length = file.seek(SeekFrom::End(0)).unwrap();

        Pager {
            file,
            file_length,
            pages: [null(); TABLE_MAX_PAGES],
        }
    }
}
