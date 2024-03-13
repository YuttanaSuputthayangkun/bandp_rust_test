pub const MAX_INPUT_LEN: usize = 1000000;

pub struct Input(Vec<Action>);

#[derive(Debug, PartialEq, Eq)]
pub enum InputConversionError {
    EmptyString,
    StringOverMaxLength,
    InvalidCharacter,
}

impl TryFrom<String> for Input {
    type Error = InputConversionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
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

pub fn check_boss_behavior(actions: impl IntoIterator<Item = Action>) -> Output {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    use Action::*;

    #[test]
    fn pos_1() {
        assert_eq!(check_boss_behavior([S, R, S, S, R, R, R]), Output::GoodBoy);
    }

    #[test]
    fn pos_2() {
        assert_eq!(check_boss_behavior([S, S, R, R]), Output::GoodBoy);
    }

    #[test]
    fn neg_1() {
        assert_eq!(check_boss_behavior([R, S, S, R, R]), Output::BadBoy);
    }

    #[test]
    fn neg_2() {
        assert_eq!(
            check_boss_behavior([S, S, S, R, R, R, R, S]),
            Output::BadBoy
        );
    }

    #[test]
    fn neg_3() {
        assert_eq!(check_boss_behavior([S, R, R, S, S, R]), Output::BadBoy);
    }
}
