#![allow(dead_code)]

use assert_cmd::assert::Assert;
use predicates::prelude::*;

/// Common stdout assertion used across CLI integration tests.
pub fn contains_usage() -> predicates::str::ContainsPredicate {
    predicate::str::contains("Usage")
}

pub trait CliAssert {
    fn success_contains(self, text: &str) -> Self;
}

impl CliAssert for Assert {
    fn success_contains(self, text: &str) -> Self {
        self.success().stdout(predicates::str::contains(text))
    }
}
