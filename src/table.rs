use std::fmt;
use std::mem::size_of;

use row::Row;

const PAGE_SIZE: usize = 4096;
const TABLE_MAX_PAGES: usize = 100;
const ROW_SIZE: usize = size_of::<Row>();
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

pub struct Table {
    n_rows: u32,
    pages: Vec<Vec<Row>>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            n_rows: 0,
            pages: Vec::new(),
        }
    }

    pub fn insert(&mut self, row: Row) -> Result<u32, String> {
        if self.n_rows as usize == TABLE_MAX_ROWS {
            Err("Error: Table full.".to_owned())
        } else {
            if self.n_rows as usize % ROWS_PER_PAGE == 0 {
                self.pages.push(vec![row]);
            } else {
                self.pages.last_mut().unwrap().push(row);
            }

            self.n_rows += 1;

            Ok(self.n_rows)
        }
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.pages.iter().flatten() {
            writeln!(f, "{}", row);
        }

        write!(f, "")
    }
}
