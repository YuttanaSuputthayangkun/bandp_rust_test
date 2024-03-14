use itertools::{Itertools, Unique};

const MAX_CHICKEN_NUM_LENGTH: usize = 1000000;
const MAX_ROOF_LENGTH: usize = 1000000;
const MAX_CHICKEN_POSITION: u32 = 1000000000;

#[derive(Debug, PartialEq, Eq)]
pub enum InputError {
    ChickenNumNotMatch,
    ChickenNumOutOfRange,
    RoofLengthOutOfRange,
    ChickenPositionOutOfRange,
    ChickenPositionLengthOutOfRange,
    ChickenPositionNotUnique,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    chicken_num: usize,
    roof_length: usize,
    chicken_positions: Vec<u32>,
}

impl Input {
    pub fn new(
        chicken_num: usize,
        roof_length: usize,
        chicken_positions: Vec<u32>,
    ) -> Result<Self, InputError> {
        use InputError::*;

        // O(1)
        if chicken_num != chicken_positions.len() {
            return Err(ChickenNumNotMatch);
        }

        // O(1)
        // I assume this because range bounds are known at compile time and should have nothing to do with the elements in between
        let is_chicken_num_in_range = (1..=MAX_CHICKEN_NUM_LENGTH).contains(&chicken_num);
        if !is_chicken_num_in_range {
            return Err(ChickenNumOutOfRange);
        }

        // O(1)
        // I assume this because range bounds are known at compile time and should have nothing to do with the elements in between
        let is_roof_length_in_range = (1..=MAX_ROOF_LENGTH).contains(&roof_length);
        if !is_roof_length_in_range {
            return Err(RoofLengthOutOfRange);
        }

        // O(N)
        let all_chicken_positions_in_range = chicken_positions
            .iter()
            .all(|p| (1..=MAX_CHICKEN_POSITION).contains(p));
        if !all_chicken_positions_in_range {
            return Err(ChickenPositionOutOfRange);
        }

        // O(N) time complexity with O(N) memory
        // I put this last because it's more expensive than other
        // the specs doesn't say anything about this, but I assume the position, logically, should be unique
        let all_chicken_posions_unique =
            chicken_positions.iter().unique().count() == chicken_positions.len();
        if !all_chicken_posions_unique {
            return Err(ChickenPositionNotUnique);
        }

        let new_input = Input {
            chicken_num,
            roof_length,
            chicken_positions,
        };
        Ok(new_input)
    }
}

pub fn max_chicken_protected() -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    use InputError::*;

    #[test]
    fn test_chicken_num_not_match() {
        assert_eq!(Input::new(1, 1, vec![]), Err(ChickenNumNotMatch));
    }

    #[test]
    fn test_chicken_num_out_of_range() {
        let chicken_num = MAX_CHICKEN_NUM_LENGTH + 1;
        let chicken_positions = (1..=chicken_num as u32).collect::<Vec<u32>>();
        assert_eq!(
            Input::new(chicken_num, 0, chicken_positions),
            Err(ChickenNumOutOfRange)
        );
    }

    #[test]
    fn test_roof_length_out_of_range() {
        assert_eq!(Input::new(1, 0, vec![1]), Err(RoofLengthOutOfRange));
    }

    #[test]
    fn test_chicken_position_out_of_range() {
        assert_eq!(
            Input::new(2, 2, vec![0, MAX_CHICKEN_POSITION + 1]),
            Err(ChickenPositionOutOfRange)
        );
    }

    #[test]
    fn test_chicken_position_not_unique() {
        assert_eq!(
            Input::new(3, 2, vec![1, 2, 2]),
            Err(ChickenPositionNotUnique)
        );
    }

    #[test]
    fn test_chicken_position_length_out_of_range() {
        assert_eq!(
            Input::new(2, 2, vec![0, (MAX_CHICKEN_POSITION + 1)]),
            Err(ChickenPositionOutOfRange)
        );
    }
}
