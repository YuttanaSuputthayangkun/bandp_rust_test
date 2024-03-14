use std::{ops::Deref, sync::Arc, thread};

use itertools::{Itertools, Unique};

use crate::limited_input_range::{self, *};

const MAX_CHICKEN_NUM_LENGTH: usize = 1000000;
const MAX_ROOF_LENGTH: usize = 1000000;
const MAX_CHICKEN_POSITION: usize = 1000000000;

// I want to validate input by type
pub type ChickenNum = LimitedInputRange<1, MAX_CHICKEN_NUM_LENGTH>;
pub type RoofLength = LimitedInputRange<1, MAX_ROOF_LENGTH>;

#[derive(Debug, PartialEq, Eq)]
pub enum ChickenPositionError {
    ChickenPositionNotUnique,
    PositionOutOfRange,
    LengthError(limited_input_range::RangeError),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ChickenPositions(Vec<usize>);

impl TryFrom<Vec<usize>> for ChickenPositions {
    type Error = ChickenPositionError;

    fn try_from(value: Vec<usize>) -> Result<Self, Self::Error> {
        // just to re-use range checking code
        let range = LimitedInputRange::<1, MAX_CHICKEN_POSITION>::try_from(value.len());
        range.map_err(ChickenPositionError::LengthError)?; // map range error into our subtype

        // O(N)
        let all_chicken_positions_in_range =
            value.iter().all(|p| (1..=MAX_CHICKEN_POSITION).contains(p));
        if !all_chicken_positions_in_range {
            return Err(ChickenPositionError::PositionOutOfRange);
        }

        // O(N) time complexity with O(N) memory
        // I put this last because it's more expensive than other
        // the specs doesn't say anything about this, but I assume the position, logically, should be unique
        let all_chicken_posions_unique = value.iter().unique().count() == value.len();
        if !all_chicken_posions_unique {
            return Err(ChickenPositionError::ChickenPositionNotUnique);
        }

        // will not validate the position order, assuming from the specs that it's guaranteed to be sorted by ascending

        let new_output = ChickenPositions(value);
        Ok(new_output)
    }
}

impl Deref for ChickenPositions {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum InputError {
    ChickenNumNotMatch,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Input {
    chicken_num: ChickenNum,
    roof_length: RoofLength,
    chicken_positions: ChickenPositions,
}

impl Input {
    pub fn new(
        chicken_num: ChickenNum,
        roof_length: RoofLength,
        chicken_positions: ChickenPositions,
    ) -> Result<Self, InputError> {
        use InputError::*;

        // O(1)
        if *chicken_num != chicken_positions.len() {
            return Err(ChickenNumNotMatch);
        }

        let new_input = Input {
            chicken_num,
            roof_length,
            chicken_positions,
        };
        Ok(new_input)
    }
}

pub fn max_chicken_protected(input: Input) -> usize {
    // O(N)
    // the strategy is to brute-force roof test on every chicken positions, but parallelize across threads

    // wrap with arc to share across the threads
    // since I don't need mutation, I don't need lock
    let shared_positions = Arc::new(input.chicken_positions);

    let mut join_handles = vec![];
    for (i, p) in shared_positions.iter().enumerate() {
        let move_positions = shared_positions.clone(); // clone before move to closure
        let handle = thread::spawn(move || {
            // I assume skip shouldn't take O(N) because the underlying type is vector
            // It should be able to jump ahead in O(1)
            let mut positions = move_positions.iter().skip(i).take(*input.roof_length);

            let first = positions.next().unwrap();
            let mut count = 1; // include first

            // O(N) where N is roof_length
            // since we already called take(roof_length), it should act as a guard
            // take_while will not take more than roof_length
            count += positions
                .take_while(|&p| p - first < *input.roof_length) // continue to take while the roof covers
                .count();

            count
        });
        join_handles.push(handle);
    }

    // I don't know if there's a better way to join multiple threads, so just linear loop for now
    let results = join_handles.into_iter().map(|h| h.join().unwrap());

    results.max().unwrap() // unwrap because previous input validation ensures that the elements are not empty
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::limited_input_range::RangeError;
    use InputError::*;

    mod input_validation {
        use std::iter;

        use super::*;
        use InputError::*;

        #[test]
        fn test_chicken_num_not_match() {
            assert_eq!(
                Input::new(
                    1.try_into().unwrap(),
                    1.try_into().unwrap(),
                    vec![1, 2].try_into().unwrap()
                ),
                Err(ChickenNumNotMatch)
            );
        }

        #[test]
        fn test_chicken_num_out_of_range() {
            assert_eq!(ChickenNum::try_from(0), Err(RangeError::UnderRange));
            assert_eq!(
                ChickenNum::try_from(MAX_CHICKEN_NUM_LENGTH + 1),
                Err(RangeError::OverRange)
            )
        }

        #[test]
        fn test_roof_length_out_of_range() {
            assert_eq!(RoofLength::try_from(0), Err(RangeError::UnderRange));
            assert_eq!(
                RoofLength::try_from(MAX_ROOF_LENGTH + 1),
                Err(RangeError::OverRange)
            );
        }

        #[test]
        fn test_chicken_position_out_of_range() {
            assert_eq!(
                ChickenPositions::try_from(vec![0, MAX_CHICKEN_POSITION + 1]),
                Err(ChickenPositionError::PositionOutOfRange)
            );
        }

        #[test]
        fn test_chicken_position_not_unique() {
            assert_eq!(
                ChickenPositions::try_from(vec![1, 2, 2]),
                Err(ChickenPositionError::ChickenPositionNotUnique)
            );
        }

        #[test]
        fn test_chicken_position_length_range() {
            // I won't test this case because the range is too high
            // assert_eq!(
            //     ChickenPositions::try_from((1..MAX_CHICKEN_POSITION + 1).collect::<Vec<_>>()),
            //     Err(ChickenPositionError::LengthError(RangeError::OverRange))
            // );
            assert_eq!(
                ChickenPositions::try_from(vec![]),
                Err(ChickenPositionError::LengthError(RangeError::UnderRange))
            );
        }
    }

    #[test]
    fn test_1() {
        let input = Input::new(
            5.try_into().unwrap(),
            5.try_into().unwrap(),
            vec![2, 5, 10, 12, 15].try_into().unwrap(),
        )
        .unwrap();
        assert_eq!(max_chicken_protected(input), 2);
    }

    #[test]
    fn test_2() {
        let input = Input::new(
            6.try_into().unwrap(),
            10.try_into().unwrap(),
            vec![1, 11, 30, 34, 35, 37].try_into().unwrap(),
        )
        .unwrap();
        assert_eq!(max_chicken_protected(input), 4);
    }

    #[test]
    fn test_3() {
        let input = Input::new(
            6.try_into().unwrap(),
            10.try_into().unwrap(),
            vec![1, 2, 3, 4, 5, 6].try_into().unwrap(),
        )
        .unwrap();
        assert_eq!(max_chicken_protected(input), 6);
    }

    #[test]
    fn test_4() {
        let input = Input::new(
            6.try_into().unwrap(),
            4.try_into().unwrap(),
            vec![1, 2, 4, 5, 6, 8].try_into().unwrap(),
        )
        .unwrap();
        assert_eq!(max_chicken_protected(input), 3);
    }
}
