use std::collections::HashMap;

use serde::{Serialize, de::DeserializeOwned};

pub struct LZ78entry<T: DeserializeOwned + Serialize> {
    index: usize,
    next_char: Option<T>,
}
