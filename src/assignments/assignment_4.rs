use std::{collections::HashSet, fmt::Debug, ops::Range, time::Instant};

use anyhow::bail;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use super::prelude::*;

pub fn get_assignment() -> Assignment {
    Assignment::new(AssignmentOptions {
        day: 4,
        description: "Printing Department",
        run: _run,
        example_input_day_1: Some(
            "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        ),
        answer_example_day_1: Some(13.into()),
        example_input_day_2: Some(
            "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        ),
        answer_example_day_2: Some(43.into()),
        answer_day_1: Some(1543.into()),
        answer_day_2: Some(9038.into()),
    })
}

struct Grid {
    data: Vec<Vec<bool>>,
}

impl Grid {
    #[must_use]
    fn from_lines(str_data: &Vec<String>) -> Result<Grid> {
        let Some(first_line) = str_data.first() else {
            bail!("Cannot make grid from empty lines")
        };

        if first_line.is_empty() {
            bail!("Cannot make grid from empty lines")
        }

        let width = first_line.len();
        if !str_data.iter().all(|line| line.len() == width) {
            bail!("All lines must have the same length ({width})")
        }

        let parsed_data = str_data
            .into_par_iter()
            .enumerate()
            .map(|(li, line)| {
                line.chars()
                    .enumerate()
                    .map(|(ci, c)| match c {
                        '.' => Ok(false),
                        '@' => Ok(true),
                        _ => bail!("Invalid char found at y={li} x={ci}: {c}"),
                    })
                    .collect()
            })
            .collect::<Result<_>>()?;

        Ok(Grid { data: parsed_data })
    }

    fn disable_indices(&mut self, indices: HashSet<usize>) {
        for index in indices {
            if let Some(value) = self.value_at_index_mut(index) {
                *value = false
            }
        }
    }

    #[must_use]
    fn width(&self) -> usize {
        self.data.first().unwrap().len()
    }

    #[must_use]
    fn height(&self) -> usize {
        self.data.len()
    }

    #[must_use]
    fn size(&self) -> usize {
        self.width() * self.height()
    }

    #[must_use]
    fn coord_at(&self, index: usize) -> GridCoord {
        let width = self.width();
        GridCoord {
            line: index / width,
            offset: index % width,
        }
    }

    #[must_use]
    fn value_at_index(&self, index: usize) -> Option<&bool> {
        self.value_at_coord(self.coord_at(index))
    }

    #[must_use]
    fn value_at_index_mut(&mut self, index: usize) -> Option<&mut bool> {
        self.value_at_coord_mut(self.coord_at(index))
    }

    #[must_use]
    fn value_at_coord(&self, coord: GridCoord) -> Option<&bool> {
        self.data
            .get(coord.line)
            .and_then(|line| line.get(coord.offset))
    }

    #[must_use]
    fn value_at_coord_mut(&mut self, coord: GridCoord) -> Option<&mut bool> {
        self.data
            .get_mut(coord.line)
            .and_then(|line| line.get_mut(coord.offset))
    }

    #[must_use]
    fn neighboring_values_at(&self, index: usize) -> Vec<(usize, &bool)> {
        let width = self.width();
        let self_coord = self.coord_at(index);

        let index_is_on_line = |index: &usize, line_offset: i32| {
            let current_line = self_coord.line;
            let new_line = current_line as i32 + line_offset;
            if new_line < 0 {
                return false;
            }
            self.coord_at(*index).line == (new_line as usize)
        };

        let index_is_on_curr_line = |index: &usize| index_is_on_line(index, 0);
        let index_is_on_prev_line = |index: &usize| index_is_on_line(index, -1);
        let index_is_on_next_line = |index: &usize| index_is_on_line(index, 1);

        // All indices, some may be None if out of bounds for 'usize'
        vec![
            // NW ↖
            index
                .checked_sub(width)
                .and_then(|v| v.checked_sub(1))
                .filter(index_is_on_prev_line),
            // N  ↑
            index.checked_sub(width),
            // NE ↗
            index
                .checked_sub(width)
                .and_then(|v| v.checked_add(1))
                .filter(index_is_on_prev_line),
            // E  →
            index.checked_add(1).filter(index_is_on_curr_line),
            // W  ←
            index.checked_sub(1).filter(index_is_on_curr_line),
            // SE ↘
            index
                .checked_add(width)
                .and_then(|v| v.checked_add(1))
                .filter(index_is_on_next_line),
            // S  ↓
            index.checked_add(width),
            // SW ↙
            index
                .checked_add(width)
                .and_then(|v| v.checked_sub(1))
                .filter(index_is_on_next_line),
        ]
        .into_par_iter()
        // Keep only valid 'usize's
        .filter_map(|index| index)
        // Check value at index and tuple with index, throw out if None
        .filter_map(|index| self.value_at_index(index).map(|value| (index, value)))
        .collect()
    }

    #[must_use]
    fn candidate_neighbor_count_at(&self, index: usize) -> usize {
        self.neighboring_values_at(index)
            .into_par_iter()
            .filter(|(_, value)| value == &&true)
            .count()
    }

    #[must_use]
    fn accessible_neighbor_indices_for(
        &self,
        accessibility_range: Range<usize>,
        indices: HashSet<usize>,
    ) -> HashSet<usize> {
        indices
            .into_par_iter()
            .filter_map(|index| {
                if self.value_at_index(index) != Some(&true) {
                    return None;
                }

                let candidate_count = self.candidate_neighbor_count_at(index);
                if accessibility_range.contains(&candidate_count) {
                    Some(index)
                } else {
                    None
                }
            })
            .collect()
    }
}

struct GridCoord {
    line: usize,
    offset: usize,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Grid")
            .field("width", &self.width())
            .field("height", &self.height())
            .finish()
    }
}

fn _run(context: AssignmentRuntimeContext) -> Result<Option<Answer>> {
    let accessibility_range = 0..4;

    let mut grid = Grid::from_lines(context.data)?;
    let size = grid.size();
    let mut indices = (0..size).collect::<HashSet<_>>();

    if context.part_number == 1 {
        let accessible_indices = grid.accessible_neighbor_indices_for(accessibility_range, indices);
        answer!(accessible_indices.len())
    }

    while let found_indices =
        grid.accessible_neighbor_indices_for(accessibility_range.clone(), indices.clone())
        && found_indices.len() != 0
    {
        indices.retain(|v| !found_indices.contains(v));
        grid.disable_indices(found_indices);
    }

    let removed_indices_count = size - indices.len();
    answer!(removed_indices_count)
}
