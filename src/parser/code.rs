use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/code.pest"]
pub struct Code;
