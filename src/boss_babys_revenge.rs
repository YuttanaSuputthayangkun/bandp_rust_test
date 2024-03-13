use std::ops::Deref;

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

        // match each character, if any char is not either S or R, it will result in error
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

// implement deref for easy access to inner elements
// as we want to treat it as vec anyway
impl Deref for Input {
    type Target = Vec<Action>;
    fn deref(&self) -> &Self::Target {
        &self.0
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
    // fast checks to detect easy negative cases
    {
        // retaliate first is always a bad boy. can unwrap here because the input has been validated to be not empty.
        if input.first().unwrap() == &Action::R {
            return Output::BadBoy;
        }

        // shoot last means no retaliation, so he's a bad boy. can unwrap here because the input has been validated to be not empty.
        if input.last().unwrap() == &Action::S {
            return Output::BadBoy;
        }
    }

    // the strategy is to divide input into chunks of shoot-retaliation sequence
    // repeatedly calls 'take_sequence' until no elements left
    // for example SRSSRRR will be divided into "SR" "SSRRR"
    {
        let actions = &input.0;
        // use peekable to preserve elements, otherwise 1 element will be lost between each sequence
        // I don't want to use classical for loop as it's not flexible and not so Rusty
        let mut iter = actions.iter().peekable();
        loop {
            let sequence_type = take_sequence(&mut iter);
            match sequence_type {
                SequenceType::Invalid => break Output::BadBoy, // sequence invalid, break off loop with negative result
                SequenceType::Empty => break Output::GoodBoy, // no element left, break off loop with positive result
                SequenceType::Valid => continue, // sequence valid, but perhaps there are elements left, so continue on
            }
        }
    }
}

#[derive(Debug)]
enum SequenceType {
    Valid,
    Invalid,
    Empty,
}

// take_sequence will consume iterator by skipping as many R, then as many S
// then checks is the sequence is valid
fn take_sequence<'a, I>(mut iter: &mut std::iter::Peekable<I>) -> SequenceType
where
    I: Iterator<Item = &'a Action>,
{
    let mut s_count: usize = 0;
    let mut r_count: usize = 0;

    loop {
        let peek = iter.peek();
        if let Some(&action) = peek {
            match action {
                // this means we found shoot when there are retaliates before
                // means the new sequence is ahead, current one has ended
                Action::S if r_count > 0 => break,
                Action::S => {
                    s_count += 1;
                    iter.next();
                }
                Action::R => {
                    r_count += 1;
                    iter.next();
                }
            };
        } else {
            break;
        }
    }

    match (s_count, r_count) {
        (0, 0) => SequenceType::Empty,
        // if number of shoot is less or equal than retaliate, consider it valid
        (s, r) if 0 < s && s <= r => SequenceType::Valid,
        _ => SequenceType::Invalid,
    }
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
