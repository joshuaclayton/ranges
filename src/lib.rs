#![deny(missing_docs)]

//! Ranges provides a data structure to manage ranges that may be expanded.
//!
//! Imagine a scenario where you maintain a set of ranges that can be added to:
//!
//! ```
//! use ranges::*;
//!
//! let mut ranges = Ranges::default();
//!
//! ranges.add_range(5..10);
//! ranges.add_range(9..12);
//! ranges.add_range(-1..3);
//!
//! assert_eq!(ranges.to_vec(), vec![-1..3, 5..12]);
//! ```

use std::fmt::Debug;
use std::ops::Range;

/// Structure that manages the expanding list of ranges.
#[derive(Default, Debug)]
pub struct Ranges<T>(Vec<Range<T>>);

fn overlaps<T: PartialOrd>(left: &Range<T>, right: &Range<T>) -> bool {
    left.contains(&right.start)
        || left.contains(&right.end)
        || right.contains(&left.start)
        || right.contains(&left.end)
}

impl<T: Copy + Ord + Debug> Ranges<T> {
    /// Generates a Vec of ranges after flattening
    ///
    /// ```
    /// use ranges::*;
    ///
    /// let mut ranges = Ranges::default();
    ///
    /// ranges.add_range(5..10);
    /// ranges.add_range(9..12);
    ///
    /// assert_eq!(ranges.to_vec(), vec![5..12]);
    /// ```
    pub fn to_vec(&self) -> Vec<Range<T>> {
        let mut results = self.0.clone();
        results.sort_by_key(|r| r.start);

        results
    }

    /// Adds a range to the managed list, expanding existing ranges if there is overlap
    pub fn add_range(&mut self, range: Range<T>) {
        if self.0.is_empty() || self.0.iter().all(|v| !overlaps(v, &range)) {
            self.0.push(range);
            return;
        }

        let mut expanded = vec![];
        for mut range_mut in self.0.iter_mut().filter(|v| overlaps(v, &range)) {
            if flatten_range(&mut range_mut, &range) {
                expanded.push(range_mut.clone());
            }
        }

        for expand in expanded {
            self.0.retain(|v| v != &expand);
            self.add_range(expand);
        }
    }
}

fn flatten_range<T: Copy + PartialOrd + Debug>(left: &mut Range<T>, right: &Range<T>) -> bool {
    let mut expanded = false;
    match (left.contains(&right.start), left.contains(&right.end)) {
        (true, true) => (),
        (true, false) => {
            left.end = right.end;
            expanded = true;
        }
        (false, true) => {
            left.start = right.start;
            expanded = true;
        }
        (false, false) => match (right.contains(&left.start), right.contains(&left.end)) {
            (true, true) => {
                left.start = right.start;
                left.end = right.end;
                expanded = true;
            }
            (true, false) => {
                left.start = right.start;
                expanded = true;
            }
            _ => (),
        },
    }
    expanded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adding_small_ranges() {
        let mut ranges = Ranges::default();

        ranges.add_range(-5..5);
        ranges.add_range(-3..2);
        ranges.add_range(-10..10);
        ranges.add_range(2..15);

        assert_eq!(ranges.to_vec(), vec![(-10..15)]);
    }

    #[test]
    fn test_adding_ranges_that_do_not_overlap() {
        let mut ranges = Ranges::default();

        ranges.add_range(8..12);
        ranges.add_range(-5..5);

        assert_eq!(ranges.to_vec(), vec![-5..5, 8..12]);

        ranges.add_range(7..12);

        assert_eq!(ranges.to_vec(), vec![-5..5, 7..12]);

        ranges.add_range(-7..-4);

        assert_eq!(ranges.to_vec(), vec![-7..5, 7..12]);
    }

    #[test]
    fn test_adding_bigger_ranges_that_do_not_overlap() {
        let mut ranges = Ranges::default();

        ranges.add_range(3623251..4598144);
        ranges.add_range(2614409..4598144);
        ranges.add_range(3623251..3959890);
        ranges.add_range(3130981..4598144);
        ranges.add_range(-5..5000);

        assert_eq!(ranges.to_vec(), vec![-5..5000, 2614409..4598144]);
    }

    #[test]
    fn test_adding_ranges_out_of_order() {
        let mut ranges = Ranges::default();

        ranges.add_range(1243425..3176262);
        ranges.add_range(-831125..1416346);
        ranges.add_range(3580256..4366587);
        ranges.add_range(3096778..3922331);

        assert_eq!(ranges.to_vec(), vec![-831125..4366587]);
    }
}
