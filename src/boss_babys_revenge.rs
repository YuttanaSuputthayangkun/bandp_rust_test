pub const MAX_INPUT_LEN: usize = 1000000;

#[derive(Debug, PartialEq, Eq)]
pub struct Input(Vec<Action>);

#[derive(Debug, PartialEq, Eq)]
pub enum InputConversionError {
    EmptyString,
    StringOverMaxLength,
    InvalidCharacter,
}

impl TryFrom<&str> for Input {
    type Error = InputConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use InputConversionError::*;
        if value.is_empty() {
            return Err(EmptyString);
        }

        if value.len() > MAX_INPUT_LEN {
            return Err(StringOverMaxLength);
        }

        let mut actions = vec![];
        for c in value.chars() {
            let action = match c {
                c if c == 'S' => Action::S,
                c if c == 'R' => Action::R,
                _ => Err(InvalidCharacter)?,
            };
            actions.push(action);
        }
        Ok(Input(actions))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    S, // shoot
    R, // retaliate
}

#[derive(Debug, PartialEq, Eq)]
pub enum Output {
    GoodBoy,
    BadBoy,
}

pub fn check_boss_behavior(input: Input) -> Output {
    todo!()
}

#[cfg(test)]
mod test {
    use std::iter;

    use super::*;
    use Action::*;
    use InputConversionError::*;
    use Output::*;

    #[test]
    fn validate_input_over_max() {
        let string = "S".repeat(MAX_INPUT_LEN + 1);
        assert_eq!(Input::try_from(string.as_str()), Err(StringOverMaxLength))
    }

    #[test]
    fn validate_input_empty() {
        assert_eq!(Input::try_from(""), Err(EmptyString))
    }

    #[test]
    fn validate_input_invalid_character() {
        assert_eq!(Input::try_from("A"), Err(InvalidCharacter))
    }

    #[test]
    fn pos_1() {
        assert_eq!(check_boss_behavior("SRSSRRR".try_into().unwrap()), GoodBoy);
    }

    #[test]
    fn pos_2() {
        assert_eq!(check_boss_behavior("SSRR".try_into().unwrap()), GoodBoy);
    }

    #[test]
    fn neg_1() {
        assert_eq!(check_boss_behavior("RSSRR".try_into().unwrap()), BadBoy);
    }

    #[test]
    fn neg_2() {
        assert_eq!(check_boss_behavior("SSSRRRRS".try_into().unwrap()), BadBoy);
    }

    #[test]
    fn neg_3() {
        assert_eq!(check_boss_behavior("SRRSSR".try_into().unwrap()), BadBoy);
    }
}
