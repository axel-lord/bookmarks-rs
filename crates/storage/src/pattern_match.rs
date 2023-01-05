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
                Some(
                    unsafe { substring_location(list, trimmed) }
                        .expect("always the case, this expect should never fail, since a trim result is always a substring of the trimmed string"),
                )
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
            Some(
                unsafe { substring_location(list_field, trimmed) }
                    .expect("in no case should this fail, since the bytes of the trim result  is alway a substring of the trimmed str")
                    .into(),
            )
        }
    })
}

/// Get the location of a string slice in another string slice.
///
/// The position is by pointer offset and as such while O(n) only works if the
/// slice is a subslice. Some meassures have been taken to detect invalid use
/// and return None in such a case, however no guarantees are given.
///
/// # Safety
/// Behaviour may be undefined if the substring is not part of the string.
pub unsafe fn substring_location(string: &str, substring: &str) -> Option<Range<usize>> {
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

/// Narrow a range of a string by whitespace.
///
/// # Panics
/// If the given range is out of bounds.
pub fn range_trim(source: &str, location: Range<usize>) -> Range<usize> {
    match unsafe { substring_location(source, source[location.clone()].trim()) } {
        Some(value) => value,
        None => panic!("location {location:?} is out of bounds for source {source:?}"),
    }
}

/// Join and iterator of string slices into a single string delimited by [token::DELIM].
pub fn join_with_delim(mut fields: impl Iterator<Item = impl AsRef<str>>) -> String {
    use once_cell::sync::Lazy;
    static DELIM: Lazy<String> = Lazy::new(|| format!(" {} ", token::DELIM));

    let mut out = String::new();

    for i in fields.by_ref().take(1) {
        out += i.as_ref();
    }

    for i in fields {
        out += &DELIM;
        out += i.as_ref();
    }

    out
}

/// Write contents of a string slice iterator delimited by [token::DELIM].
///
/// # Errors
/// If a write operation failed.
pub fn write_delim_list(
    f: &mut std::fmt::Formatter<'_>,
    mut iter: impl Iterator<Item = impl AsRef<str>>,
) -> std::fmt::Result {
    for i in iter.by_ref().take(1) {
        write!(f, " {} ", i.as_ref())?;
    }
    for i in iter {
        write!(f, "{} {} ", token::DELIM, i.as_ref())?;
    }
    Ok(())
}

/// Write contents of a list field.
///
/// # Errors
/// If a write operation failed.
pub fn write_list_field(
    f: &mut std::fmt::Formatter<'_>,
    mut iter: impl Iterator<Item = impl AsRef<str>>,
) -> std::fmt::Result {
    write!(f, "[")?;
    for i in iter.by_ref().take(1) {
        write!(f, "{}", i.as_ref())?;
    }
    for i in iter {
        write!(f, ", {}", i.as_ref())?;
    }
    write!(f, "]")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn substring_location_basic() {
        let string = "Hello there! Nice to meet you!";
        assert_eq!(
            unsafe { substring_location(string, &string[0..5]) },
            Some(0..5)
        );
        assert_eq!(
            unsafe { substring_location(string, &string[2..5]) },
            Some(2..5)
        );
        assert_eq!(
            unsafe { substring_location(string, &string[2..7]) },
            Some(2..7)
        );
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
                .map(|f| &field[Range::from(f)])
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
                .map(|f| &field[Range::from(f)])
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
                .map(|f| &field[Range::from(f)])
                .collect::<Vec<_>>(),
            Vec::<&str>::new()
        );
    }
}
