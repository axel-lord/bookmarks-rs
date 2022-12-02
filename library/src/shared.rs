use crate::container;
use std::sync::{Arc, RwLock};

pub type BufferStorage<T> = Arc<RwLock<container::BufferStorage<T>>>;
