mod lexer;
mod location;
mod parser;

use crate::source::Source;

pub fn parse(source: String) -> Result<Details, parser::Error> {
    parser::Parser::new(source)?.parse()
}

#[derive(Debug)]
pub struct Details {
    pub name: String,
    pub version: String,
    pub description: String,
    pub url: Option<String>,

    pub runtime_dependencies: Vec<String>,
    pub build_dependencies: Vec<String>,

    pub sources: Vec<Source>,
}
