use super::{Buffer, Selected, Storage};

#[derive(Debug, Clone, Default)]
pub struct BufferStorage<T> {
    pub storage: Storage<T>,
    pub buffer: Buffer,
    pub selected: Selected,
}

#[derive(Clone, Copy, Debug)]
pub enum GetSelectedErr {
    Index(usize),
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
    pub fn new(storage: Storage<T>, buffer: Buffer, selected: Selected) -> Self {
        Self {
            storage,
            buffer,
            selected,
        }
    }

    pub fn reset(&mut self) -> &Self {
        self.buffer.reset();
        self.selected.clear();
        self
    }

    pub fn filter_in_place(&mut self, f: impl FnMut(&T) -> bool) -> &mut Self {
        self.buffer.filter_in_place(&self.storage, f);
        self
    }

    pub fn get_selected(&self) -> Result<&T, GetSelectedErr> {
        self.storage
            .get(self.selected.index().ok_or(GetSelectedErr::Empty)?)
            .ok_or_else(|| GetSelectedErr::Index(self.selected.index().unwrap()))
    }

    pub fn get_selected_mut(&mut self) -> Result<&mut T, GetSelectedErr> {
        self.storage
            .get_mut(self.selected.index().ok_or(GetSelectedErr::Empty)?)
            .ok_or_else(|| GetSelectedErr::Index(self.selected.index().unwrap()))
    }

    pub fn get_index_and_selected_and(&self) -> Result<(usize, &T), GetSelectedErr> {
        let index = self.selected.index().ok_or(GetSelectedErr::Empty)?;
        Ok((
            index,
            self.storage
                .get(self.selected.index().ok_or(GetSelectedErr::Empty)?)
                .ok_or_else(|| GetSelectedErr::Index(self.selected.index().unwrap()))?,
        ))
    }

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
