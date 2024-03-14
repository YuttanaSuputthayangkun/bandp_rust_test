const BROADCAST_URL: &str = "https://mock-node-wgqbnxruha-as.a.run.app/broadcast";

#[derive(Clone, PartialEq, Eq)]
pub struct Symbol(String);

pub const SYMBOL_LENGTH: usize = 3;

#[derive(Debug, PartialEq, Eq)]
pub enum IntoSymbolError {
    InvalidLength,
    InvalidCharacter,
}

impl<'a> TryFrom<&'a str> for Symbol {
    type Error = IntoSymbolError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            invalid_len if invalid_len.len() != SYMBOL_LENGTH => {
                Err(IntoSymbolError::InvalidLength)
            }
            invalid_char if !invalid_char.chars().all(|c| c.is_alphabetic()) => {
                Err(IntoSymbolError::InvalidCharacter)
            }
            valid => Ok(Symbol(valid.to_ascii_uppercase().to_string())),
        }
    }
}

impl std::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub fn broadcast() {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    mod input_validation {
        use super::*;

        #[test]
        fn symbol_invalid_length() {
            assert_eq!(Symbol::try_from("A"), Err(IntoSymbolError::InvalidLength));
            assert_eq!(
                Symbol::try_from("AAAA"),
                Err(IntoSymbolError::InvalidLength)
            );
        }

        #[test]
        fn symbol_invalid_character() {
            assert_eq!(
                Symbol::try_from("555"),
                Err(IntoSymbolError::InvalidCharacter)
            );
        }
    }
}
