mod combinator;
mod expression;
mod generator;
mod permutator;

pub(crate) use combinator::Combinator;
pub(crate) use expression::Expression;
pub(crate) use generator::Generator;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Default, Clone, Debug)]
pub(crate) struct Credentials {
    pub username: String,
    pub password: String,
}

impl Credentials {
    #[inline(always)]
    pub fn single(&self) -> &str {
        if self.username.is_empty() {
            &self.password
        } else {
            &self.username
        }
    }
}
