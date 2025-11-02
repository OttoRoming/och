use super::{
    Details, Source,
    lexer::{self, Lexer, Token, TokenKind},
};
use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    Lexer(lexer::Error),
    UnexpectedToken(Token),
    MissingName,
    MissingVersion,
    MissingDescripiton,
}
impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Lexer(error) => write!(f, "Lexer error {}", error),
            Error::UnexpectedToken(token) => {
                write!(f, "{} Unexpected token {:?}", token.region, token.kind)
            }
            Error::MissingName => write!(f, "Details is missing name"),
            Error::MissingVersion => write!(f, "Details is missing version"),
            Error::MissingDescripiton => write!(f, "Details is missing description"),
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    i: usize,
}

impl Parser {
    pub fn new(source: String) -> Result<Self, Error> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex().map_err(|e| Error::Lexer(e))?;

        Ok(Self { tokens, i: 0 })
    }

    fn current(&self) -> &Token {
        &self.tokens[self.i]
    }

    fn advance(&mut self) {
        self.i += 1;
    }

    fn parse_name(&mut self) -> Result<String, Error> {
        assert_eq!(self.current().kind, TokenKind::KeywordName);
        self.advance();

        let name = match &self.current().kind {
            TokenKind::String(s) => s.to_string(),
            _ => return Err(Error::UnexpectedToken(self.current().clone())),
        };
        self.advance();

        Ok(name)
    }

    fn parse_version(&mut self) -> Result<String, Error> {
        assert_eq!(self.current().kind, TokenKind::KeywordVersion);
        self.advance();

        let version = match &self.current().kind {
            TokenKind::String(version) => version.to_string(),
            _ => return Err(Error::UnexpectedToken(self.current().clone())),
        };
        self.advance();

        Ok(version)
    }

    fn parse_description(&mut self) -> Result<String, Error> {
        assert_eq!(self.current().kind, TokenKind::KeywordDescription);
        self.advance();

        let description = match &self.current().kind {
            TokenKind::String(description) => description.to_string(),
            _ => return Err(Error::UnexpectedToken(self.current().clone())),
        };
        self.advance();

        Ok(description)
    }

    fn parse_url(&mut self) -> Result<String, Error> {
        assert_eq!(self.current().kind, TokenKind::KeywordUrl);
        self.advance();

        let url = match &self.current().kind {
            TokenKind::String(url) => url.to_string(),
            _ => return Err(Error::UnexpectedToken(self.current().clone())),
        };
        self.advance();

        Ok(url)
    }

    fn parse_string_list(&mut self) -> Vec<String> {
        let mut list = vec![];

        loop {
            match &self.current().kind {
                TokenKind::String(s) => list.push(s.to_string()),
                _ => break,
            }
            self.advance();
        }

        list
    }

    fn parse_build_dependencies(&mut self) -> Vec<String> {
        assert_eq!(self.current().kind, TokenKind::KeywordBuild);
        self.advance();

        self.parse_string_list()
    }

    fn parse_runtime_dependencies(&mut self) -> Vec<String> {
        assert_eq!(self.current().kind, TokenKind::KeywordRuntime);
        self.advance();

        self.parse_string_list()
    }

    fn parse_source_tar(&mut self) -> Result<Source, Error> {
        assert_eq!(self.current().kind, TokenKind::KeywordTar);
        self.advance();

        let url = match &self.current().kind {
            TokenKind::String(s) => s.to_string(),
            _ => return Err(Error::UnexpectedToken(self.current().clone())),
        };
        self.advance();

        let hash = match &self.current().kind {
            TokenKind::String(s) => Some(s.to_string()),
            _ => None,
        };
        if hash.is_some() {
            self.advance();
        }

        Ok(Source::Tar { url, hash })
    }

    fn parse_source(&mut self) -> Result<Source, Error> {
        assert_eq!(self.current().kind, TokenKind::KeywordSource);
        self.advance();

        match self.current().kind {
            TokenKind::KeywordTar => self.parse_source_tar(),
            _ => Err(Error::UnexpectedToken(self.current().clone())),
        }
    }

    pub fn parse(&mut self) -> Result<Details, Error> {
        let mut name = None;
        let mut version = None;
        let mut description = None;
        let mut url = None;
        let mut sources = vec![];

        let mut build_dependencies = vec![];
        let mut runtime_dependencies = vec![];

        loop {
            match self.current().kind {
                TokenKind::KeywordName => name = Some(self.parse_name()?),
                TokenKind::KeywordVersion => version = Some(self.parse_version()?),
                TokenKind::KeywordDescription => description = Some(self.parse_description()?),
                TokenKind::KeywordUrl => url = Some(self.parse_url()?),
                TokenKind::KeywordSource => sources.push(self.parse_source()?),
                TokenKind::KeywordBuild => {
                    build_dependencies.extend(self.parse_build_dependencies())
                }
                TokenKind::KeywordRuntime => {
                    runtime_dependencies.extend(self.parse_runtime_dependencies())
                }
                TokenKind::EOD => break,
                _ => {
                    return Err(Error::UnexpectedToken(self.current().clone()));
                }
            };
        }

        Ok(Details {
            name: name.ok_or(Error::MissingName)?,
            version: version.ok_or(Error::MissingVersion)?,
            description: description.ok_or(Error::MissingDescripiton)?,
            url,
            sources,
            runtime_dependencies,
            build_dependencies,
        })
    }
}
