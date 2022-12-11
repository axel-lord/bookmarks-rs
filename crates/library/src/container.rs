//! Container and access types are useful for unified behaviour accross different commands.

mod buffer;
mod buffer_storage;
mod selected;
mod storage;

pub use buffer::Buffer;
pub use buffer_storage::BufferStorage;
pub use buffer_storage::GetSelectedErr;
pub use selected::Selected;
pub use storage::Storage;
