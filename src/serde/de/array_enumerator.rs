use magnus::{RArray, Ruby, Value};

use super::super::Error;

/// Index-based Ruby array iterator that avoids Ruby Enumerator fiber overhead.
pub(super) struct ArrayEnumerator<'ruby> {
    ruby: &'ruby Ruby,
    array: RArray,
    index: usize,
}

impl<'ruby> ArrayEnumerator<'ruby> {
    /// Create an iterator over a Ruby array.
    pub(super) fn new(ruby: &'ruby Ruby, array: RArray) -> Self {
        Self {
            ruby,
            array,
            index: 0,
        }
    }

    /// Return the number of entries that have not been yielded.
    pub(super) fn remaining(&self) -> usize {
        self.array.len().saturating_sub(self.index)
    }

    /// Return the current array entry without advancing the iterator.
    fn current(&self) -> Result<Option<Value>, Error> {
        if self.index >= self.array.len() {
            return Ok(None);
        }

        let index = isize::try_from(self.index).map_err(|_| {
            Error::from(magnus::Error::new(
                self.ruby.exception_range_error(),
                "array index out of range",
            ))
        })?;
        self.array.entry(index).map(Some).map_err(Into::into)
    }
}

impl Iterator for ArrayEnumerator<'_> {
    type Item = Result<Value, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current() {
            Ok(Some(value)) => {
                self.index = match self.index.checked_add(1) {
                    Some(index) => index,
                    None => return Some(Err(Error::message("array index overflow"))),
                };
                Some(Ok(value))
            }
            Ok(None) => None,
            Err(error) => Some(Err(error)),
        }
    }
}
