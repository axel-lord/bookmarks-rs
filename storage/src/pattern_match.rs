use crate::{token, Field};
use std::{convert::TryInto, ops::Range};

/// Split a string slice into a vector of string slices with the delimiter [token::DELIM].
///
/// # Panics
/// If [substring_location] fails, should in this case never happen.
pub fn split_by_delim_to_ranges(list: &str) -> Vec<Range<usize>> {
    list.split(token::DELIM)
        .filter_map(|item| {
            let trimmed = item.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(substring_location(list, trimmed).unwrap())
            }
        })
        .collect()
}

/// Split a string slice into an iterator of [Field] based on the delimiter [token::DELIM].
///
/// # Panics
/// If [substring_location] fails, should in this case never happen.
pub fn split_list_field(list_field: &'_ str) -> impl '_ + Iterator<Item = Field> {
    list_field.split(token::DELIM).filter_map(|item| {
        let trimmed = item.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(substring_location(list_field, trimmed).unwrap().into())
        }
    })
}

/// Get the location of a string slice in another string slice.
///
/// The position is by pointer offset and as such while O(n) only works if the
/// slice is a subslice. Some meassures have been taken to detect invalid use
/// and return None in such a case, however no guarantees are given.
pub fn substring_location(string: &str, substring: &str) -> Option<Range<usize>> {
    let string_ptr = string.as_ptr();
    let substring_ptr = substring.as_ptr();

    if substring_ptr < string_ptr
        || unsafe {
            substring_ptr.offset(substring.len().try_into().ok()?)
                > string_ptr.offset(string.len().try_into().ok()?)
        }
    {
        return None;
    }

    let start = unsafe { substring_ptr.offset_from(string_ptr) } as usize;
    let end = unsafe {
        substring_ptr
            .offset(substring.len().try_into().ok()?)
            .offset_from(string_ptr)
    } as usize;

    Some(start..end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn substring_location_basic() {
        let string = "Hello there! Nice to meet you!";
        assert_eq!(substring_location(string, &string[0..5]), Some(0..5));
        assert_eq!(substring_location(string, &string[2..5]), Some(2..5));
        assert_eq!(substring_location(string, &string[2..7]), Some(2..7));
    }

    #[test]
    pub fn field_split_basic() {
        let field = "a <,> b <,> c";

        assert_eq!(
            split_by_delim_to_ranges(field)
                .iter()
                .map(|r| &field[r.clone()])
                .collect::<Vec<_>>(),
            vec!["a", "b", "c"]
        );
        assert_eq!(
            split_list_field(field)
                .map(|f| &field[f.0])
                .collect::<Vec<_>>(),
            vec!["a", "b", "c"]
        );
        let field = "a <,> b <,> c <,>";

        assert_eq!(
            split_by_delim_to_ranges(field)
                .iter()
                .map(|r| &field[r.clone()])
                .collect::<Vec<_>>(),
            vec!["a", "b", "c"]
        );
        assert_eq!(
            split_list_field(field)
                .map(|f| &field[f.0])
                .collect::<Vec<_>>(),
            vec!["a", "b", "c"]
        );

        let field = "";

        assert_eq!(
            split_by_delim_to_ranges(field)
                .iter()
                .map(|r| &field[r.clone()])
                .collect::<Vec<_>>(),
            Vec::<&str>::new()
        );
        assert_eq!(
            split_list_field(field)
                .map(|f| &field[f.0])
                .collect::<Vec<_>>(),
            Vec::<&str>::new()
        );
    }
}
