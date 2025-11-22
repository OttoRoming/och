mod lexer;
mod location;
mod parser;
#[cfg(test)]
mod test;

use crate::source::Source;

pub use parser::Error;

pub fn parse(source: String) -> Result<Details, Error> {
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
