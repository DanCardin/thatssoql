use crate::pager::Pager;
use crate::row::Row;
use crate::statement::Statement;
use crate::table::Table;

pub struct Cursor<'a> {
    table: &'a mut Table,
    pager: &'a mut Pager,
    row: usize,
}

impl<'a> Cursor<'a> {
    pub fn table_start(table: &'a mut Table, pager: &'a mut Pager) -> Self {
        Self {
            table: table,
            pager: pager,
            row: 1,
        }
    }

    // fn table_end(table: &'a mut Table) -> Self {
    //     let row = self.pager.rows.len();
    //     Self {
    //         table: table,
    //         row: row,
    //     }
    // }

    pub fn execute(self: &mut Self, statement: Statement) {
        match statement {
            Statement::Insert(id, name, email) => {
                let row = Row {
                    id: id,
                    name: name.as_bytes().to_vec(),
                    email: email.as_bytes().to_vec(),
                };
                self.pager.insert(row);
            }
            Statement::Select => {
                while let Some(row) = self.get_value() {
                    println!("{:?}", row);
                    self.advance();
                }
            }
            _ => panic!("wat"),
        }
    }

    pub fn get_value(self: &mut Self) -> Option<&Row> {
        self.pager.rows.get(&self.row)
    }

    fn advance(self: &mut Self) {
        self.row += 1;
    }
}
