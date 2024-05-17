#![allow(unused_imports)]
pub mod user;

pub mod prelude {
    use super::*;

    pub use user::*;
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct QuerySegments {
    pub(crate) cluster: String,
    pub(crate) custom_url: String,
}

impl QuerySegments {
    pub fn new(cluster: String, custom_url: String) -> Self {
        Self {
            cluster,
            custom_url,
        }
    }
}
