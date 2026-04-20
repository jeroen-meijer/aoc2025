use std::cmp::Ordering;

use anyhow::{Context, anyhow};
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use super::prelude::*;

pub fn get_assignment() -> Assignment {
    Assignment::new(AssignmentOptions {
        day: 3,
        description: "Lobby",
        run: _run,
        example_input_day_1: Some(
            "
987654321111111
811111111111119
234234234234278
818181911112111",
        ),
        answer_example_day_1: Some(357.into()),
        example_input_day_2: Some(
            "
987654321111111
811111111111119
234234234234278
818181911112111",
        ),
        answer_example_day_2: Some(3121910778619u64.into()),
        answer_day_1: Some(17443.into()),
        answer_day_2: Some(172167155440541u64.into()),
    })
}

/// Searches the given bank for the highest number that can be constructed from the digits in the
/// bank without changing their order.
///
/// Recursively:
/// 0. If digits is 0, return empty vec.
/// 1. Search for the highest number in the bank between ranges [0..n-d+1), so a bank of 10 digits
///    with d = 2 means range ends at 10-2+1, so [0..9).
/// 2. Store that number N.
/// 3. Create a new bank consisting of all numbers to the right of N.
/// 4. Create a list with two elements: [N], and a recursive rerun with digits - 1 and the remaining
///    bank.
/// 5. Flat-map, join the string, convert to a number, and return.
fn find_largest_num(digits: usize, bank: &String) -> Result<u64> {
    // Example digits: 2
    // Offset = digits - 1 = 1
    // Example bank: 813593202
    // Start search:        ^
    // Highest nums:     ^
    // Remaining   : _____3202
    // Next search:       ^
    // Done: 93

    fn find_largest_num_internal(digits: usize, remaining_bank: Vec<u8>) -> Result<Vec<u8>> {
        if digits == 0 {
            return Ok(vec![]);
        }

        let search_end_index = remaining_bank.len() - digits + 1;
        let search_range = 0..search_end_index;

        let (largest_index, largest_value) = remaining_bank[search_range.clone()]
            .iter()
            .enumerate()
            .max_by(|a, b| match a.1.cmp(b.1) {
                Ordering::Equal => Ordering::Greater,
                ordering => ordering,
            })
            .ok_or(anyhow!(format!(
                "Search range is empty. Bank: {remaining_bank:?}. Range: {search_range:?}"
            )))?;

        let remaining_bank = remaining_bank[(largest_index + 1)..].to_vec();

        let result = vec![
            vec![*largest_value],
            find_largest_num_internal(digits - 1, remaining_bank)?,
        ]
        .into_iter()
        .flatten()
        .collect_vec();

        Ok(result)
    }

    let parsed_bank = bank
        .chars()
        .into_iter()
        .map(|s| {
            s.to_digit(10)
                .map(|x| x as u8)
                .ok_or(anyhow!(format!("Could not convert string to digit: {s}")))
        })
        .collect::<Result<Vec<_>>>()?;
    let largest_sequence = find_largest_num_internal(digits, parsed_bank)?;

    let string_sequence = largest_sequence.into_iter().map(|d| d.to_string()).join("");

    let largest_number = string_sequence
        .parse()
        .with_context(|| format!("failed to parse largest sequence '{string_sequence}' as u32"));

    largest_number
}

fn _run(context: AssignmentRuntimeContext) -> Result<Option<Answer>> {
    let digits = if context.part_number == 1 { 2 } else { 12 };

    let largest_numbers_per_bank = context
        .data
        .into_par_iter()
        .map(|bank| find_largest_num(digits, bank))
        .collect::<Result<Vec<_>>>()?;

    let sum = largest_numbers_per_bank.into_iter().sum::<u64>();

    answer!(sum)
}
