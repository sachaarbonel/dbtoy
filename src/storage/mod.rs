use crate::{ColumnDef, DataValue};

pub mod disk;
pub mod memory;

pub trait Storage {
    type NewArgs;
    fn new(args: Self::NewArgs) -> Self;
    fn insert_table(
        &mut self,
        table_name: String,
        columns: Vec<ColumnDef>,
        row: Vec<Vec<DataValue>>,
    );

    fn get_table(&mut self, table_name: &str)
        -> Option<&mut (Vec<ColumnDef>, Vec<Vec<DataValue>>)>;
    fn push_value(&mut self, table_name: &str, row: Vec<DataValue>);

    fn contains_key(&self, table_name: &str) -> bool;
}
