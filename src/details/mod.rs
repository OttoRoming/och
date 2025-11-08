mod lexer;
mod location;
mod parser;

pub fn parse(source: String) -> Result<Details, parser::Error> {
    parser::Parser::new(source)?.parse()
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Source {
    Tar { url: String, hash: Option<String> },
    Get { url: String, hash: Option<String> },
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
