use yewdux::prelude::*;

#[derive(Clone, PartialEq, Store)]
pub struct ProfileStore {
    pub(crate) first_name: String,
    pub(crate) last_name: String,
}

impl Default for ProfileStore {
    fn default() -> Self {
        Self {
            first_name: "foo".to_string(),
            last_name: "bar".to_string(),
        }
    }
}