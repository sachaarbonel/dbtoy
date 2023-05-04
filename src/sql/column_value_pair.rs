use nom::{
    bytes::complete::{tag, take_while1},
    combinator::opt,
    sequence::tuple,
    IResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColumnValuePair {
    pub column_name: String,
    pub table_name: String,
}

impl ColumnValuePair {
    pub fn new(column_name: &str, table_name: &str) -> Self {
        Self {
            column_name: column_name.to_string(),
            table_name: table_name.to_string(),
        }
    }
}

fn identifier(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
}

impl ColumnValuePair {
    pub fn parse(input: &str) -> IResult<&str, ColumnValuePair> {
        let (input, (table_part, column)) =
            tuple((opt(tuple((identifier, tag(".")))), identifier))(input)?;

        let table_name = match table_part {
            Some((table, _)) => table.to_string(),
            None => String::new(),
        };

        Ok((
            input,
            ColumnValuePair {
                column_name: column.to_string(),
                table_name,
            },
        ))
    }
}
