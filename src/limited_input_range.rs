use std::ops::Deref;

#[derive(Debug, PartialEq, Eq)]
pub enum RangeError {
    UnderRange,
    OverRange,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LimitedInputRange<const MIN: usize, const MAX: usize>(usize);

impl<const MIN: usize, const MAX: usize> TryFrom<usize> for LimitedInputRange<MIN, MAX> {
    type Error = RangeError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        use RangeError::*;
        match value {
            under if under < MIN => Err(UnderRange),
            over if over > MAX => Err(OverRange),
            _ => Ok(LimitedInputRange(value)),
        }
    }
}

// for ease of use
impl<const MIN: usize, const MAX: usize> Deref for LimitedInputRange<MIN, MAX> {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
