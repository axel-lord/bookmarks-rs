use super::{Buffer, Selected, Storage};

/// A conveniance type for combining [Buffer], [Storage] and [Selected], since they are often used
/// in combinayion with each other.
#[derive(Debug, Clone, Default)]
pub struct BufferStorage<T> {
    /// List of some kind of item.
    pub storage: Storage<T>,
    /// List of indices.
    pub buffer: Buffer,
    /// An optional single index.
    pub selected: Selected,
}

/// Error type for when getting an item based on [Selected] fails.
#[derive(Clone, Copy, Debug)]
pub enum GetSelectedErr {
    /// When the index is out of range.
    Index(usize),
    /// When nothing is selected.
    Empty,
}

impl std::fmt::Display for GetSelectedErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Self::Index(i) = self {
            write!(f, "index {i} is invalid")
        } else {
            write!(f, "nothing selected")
        }
    }
}

impl std::error::Error for GetSelectedErr {}

impl<T> BufferStorage<T> {
    /// Create a new [BufferStorage].
    pub fn new(storage: Storage<T>, buffer: Buffer, selected: Selected) -> Self {
        Self {
            storage,
            buffer,
            selected,
        }
    }

    /// Reset the [Selected] and [Buffer] of the [BufferStorage].
    /// Important to note that the [Storage] stays the same.
    pub fn reset(&mut self) -> &Self {
        self.buffer.reset();
        self.selected.clear();
        self
    }

    /// Filter the [Buffer] in place based on the condition applied to the contents of the [Storage].
    pub fn filter_in_place(&mut self, f: impl FnMut(&T) -> bool) -> &mut Self {
        self.buffer.filter_in_place(&self.storage, f);
        self
    }

    /// Get the currently selected item in the [Storage] based on the [Selected].
    pub fn get_selected(&self) -> Result<&T, GetSelectedErr> {
        self.storage
            .get(self.selected.index().ok_or(GetSelectedErr::Empty)?)
            .ok_or_else(|| GetSelectedErr::Index(self.selected.index().unwrap()))
    }

    /// Get the currently selected item as mutable in the [Storage] based on the [Selected].
    pub fn get_selected_mut(&mut self) -> Result<&mut T, GetSelectedErr> {
        self.storage
            .get_mut(self.selected.index().ok_or(GetSelectedErr::Empty)?)
            .ok_or_else(|| GetSelectedErr::Index(self.selected.index().unwrap()))
    }

    /// Get the currently selected item in the [Storage] based on the [Selected], also gets the
    /// index of the item.
    pub fn get_index_and_selected_and(&self) -> Result<(usize, &T), GetSelectedErr> {
        let index = self.selected.index().ok_or(GetSelectedErr::Empty)?;
        Ok((
            index,
            self.storage
                .get(self.selected.index().ok_or(GetSelectedErr::Empty)?)
                .ok_or_else(|| GetSelectedErr::Index(self.selected.index().unwrap()))?,
        ))
    }

    /// Get the currently selected item as mutable in the [Storage] based on the [Selected], also gets the
    /// index of the item.
    pub fn get_index_and_selected_mut(&mut self) -> Result<(usize, &mut T), GetSelectedErr> {
        let index = self.selected.index().ok_or(GetSelectedErr::Empty)?;
        Ok((
            index,
            self.storage
                .get_mut(index)
                .ok_or_else(|| GetSelectedErr::Index(self.selected.index().unwrap()))?,
        ))
    }
}
