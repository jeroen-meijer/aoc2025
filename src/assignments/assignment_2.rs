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
        answer_example_day_1: Some(1227775554.into()),
        example_input_day_2: None,
        answer_example_day_2: None,
        answer_day_1: None,
        answer_day_2: None,
    })
}

fn _run(context: AssignmentRuntimeContext) -> Result<Option<Answer>> {
    // TODO: Implement solution for day 2

    Ok(None)
}
