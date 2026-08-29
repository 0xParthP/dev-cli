#![allow(dead_code)]

use predicates::prelude::*;

/// Common stdout assertion used across CLI integration tests.
pub fn contains_usage() -> predicates::str::ContainsPredicate {
    predicate::str::contains("Usage")
}
