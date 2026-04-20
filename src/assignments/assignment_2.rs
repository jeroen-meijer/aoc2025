use std::ops::RangeInclusive;

use anyhow::anyhow;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use super::prelude::*;

pub fn get_assignment() -> Assignment {
    Assignment::new(AssignmentOptions {
        day: 2,
        description: "Gift Shop",
        run: _run,
        example_input_day_1: Some(
            "
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124",
        ),
        answer_example_day_1: Some(1227775554u64.into()),
        example_input_day_2: Some(
            "
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124",
        ),
        answer_example_day_2: Some(4174379265u64.into()),
        answer_day_1: Some(23039913998u64.into()),
        answer_day_2: Some(35950619148u64.into()),
    })
}

fn is_symmetric_num(num: &u64, check_all_chunks: bool) -> bool {
    let num_str = num.to_string();
    let str_len = num_str.len();

    let has_equal_chunks = |chunk_size: usize| -> bool {
        let strings = num_str
            .chars()
            .chunks(chunk_size)
            .into_iter()
            .map(Iterator::collect::<String>)
            .collect_vec();

        let all_equal = &strings.iter().all_equal();

        *all_equal
    };

    if !check_all_chunks {
        if str_len % 2 != 0 {
            return false;
        }
        return has_equal_chunks(str_len / 2);
    }

    let possible_chunk_sizes = (1..str_len).filter(|s| str_len % s == 0);

    for s in possible_chunk_sizes {
        if has_equal_chunks(s) {
            return true;
        }
    }

    false
}

fn _run(context: AssignmentRuntimeContext) -> Result<Option<Answer>> {
    let ranges = context
        .data
        .join("")
        .trim()
        .split(",")
        .map(|range_str| {
            let (start_str, end_str) = range_str.split("-").collect_tuple().ok_or(anyhow!(""))?;
            let (start, end) = (start_str.parse()?, end_str.parse()?);
            Ok(start..=end)
        })
        .collect::<Result<Vec<RangeInclusive<u64>>>>()?;

    let check_all_chunks = context.part_number == 2;
    let found_symmetric_nums = ranges
        .into_par_iter()
        .flat_map(|range| range.collect_vec())
        .filter(|num| is_symmetric_num(num, check_all_chunks));

    let result = found_symmetric_nums.sum::<u64>();

    answer!(result)
}
