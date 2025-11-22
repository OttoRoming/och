use super::*;

#[test]
fn no_tokens() {
    assert_eq!(parse("".to_string()).unwrap_err(), Error::NoTokens);
}
