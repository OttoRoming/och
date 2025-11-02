use super::location::{Location, Region};
use std::{error, fmt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    KeywordName,
    KeywordVersion,
    KeywordDescription,
    KeywordUrl,
    KeywordSource,
    KeywordTar,
    KeywordBuild,
    KeywordRuntime,
    String(String),
    EOD,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub region: Region,
}

#[derive(Clone, Debug)]
pub enum Error {
    UnexpectedEOD,
    UnexpectedCharacter { location: Location, character: char },
    UnknownKeyword { region: Region, name: String },
    EODTokenTooShort { locaiton: Location },
}

impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedCharacter {
                location,
                character,
            } => {
                write!(
                    f,
                    "{} Found unexpected character \"{}\"",
                    location, character
                )
            }
            Self::UnknownKeyword { region, name } => {
                write!(f, "{} Unknown keyword \"{}\"", region, name)
            }
            Self::UnexpectedEOD => {
                write!(f, "Unexpected end of description (did you forget \"---\")")
            }
            Self::EODTokenTooShort { locaiton: location } => {
                writeln!(f, "{} EOD Token is too short :(", location)
            }
        }
    }
}

pub struct Lexer {
    source: String,
    i: usize,
    line: usize,
    char: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            i: 0,
            line: 1,
            char: 1,
            source,
        }
    }

    fn current(&self) -> Result<char, Error> {
        self.source.chars().nth(self.i).ok_or(Error::UnexpectedEOD)
    }

    fn advance(&mut self) -> Result<(), Error> {
        self.i += 1;
        self.char += 1;

        if self.current()? == '\n' {
            self.i += 1;
            self.line += 1;
            self.char = 1;
        }

        Ok(())
    }

    fn current_location(&self) -> Location {
        Location::new(self.line, self.char)
    }

    fn lex_string(&mut self) -> Result<TokenKind, Error> {
        assert_eq!(self.current()?, '"');
        self.advance()?;

        let mut value = String::new();
        while self.current()? != '"' {
            value.push(self.current()?);
            self.advance()?;
        }
        self.advance()?;

        Ok(TokenKind::String(value))
    }

    fn lex_keyword(&mut self) -> Result<TokenKind, Error> {
        let start = self.current_location();
        let mut name = String::new();
        while self.current()?.is_alphabetic() {
            name.push(self.current()?);
            self.advance()?;
        }

        let keyword = match name.as_str() {
            "name" => TokenKind::KeywordName,
            "version" => TokenKind::KeywordVersion,
            "description" => TokenKind::KeywordDescription,
            "url" => TokenKind::KeywordUrl,

            "source" => TokenKind::KeywordSource,
            "tar" => TokenKind::KeywordTar,

            "build" => TokenKind::KeywordBuild,
            "runtime" => TokenKind::KeywordRuntime,
            _ => {
                return Err(Error::UnknownKeyword {
                    region: Region::new(start, self.current_location()),
                    name,
                });
            }
        };

        Ok(keyword)
    }

    fn lex_eod(&mut self) -> Result<TokenKind, Error> {
        for _ in 0..3 {
            if self.current()? != '-' {
                return Err(Error::EODTokenTooShort {
                    locaiton: self.current_location(),
                });
            }
        }

        Ok(TokenKind::EOD)
    }

    fn lex_token(&mut self) -> Result<Token, Error> {
        let start = self.current_location();

        let kind = if self.current()?.is_alphabetic() {
            self.lex_keyword()
        } else {
            match self.current()? {
                '"' => self.lex_string(),
                '-' => self.lex_eod(),
                _ => Err(Error::UnexpectedCharacter {
                    location: self.current_location(),
                    character: self.current()?,
                }),
            }
        }?;

        let end = self.current_location();
        let region = Region::new(start, end);

        Ok(Token { kind, region })
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, Error> {
        let mut result = vec![];

        while self.i < self.source.len() {
            match self.current()? {
                '#' | ' ' | '\t' => {
                    self.advance()?;
                    continue;
                }
                _ => {}
            }

            let token = self.lex_token()?;
            if token.kind == TokenKind::EOD {
                result.push(token);
                break;
            }
            result.push(token);
        }

        Ok(result)
    }
}
