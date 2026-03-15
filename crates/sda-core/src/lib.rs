//! Core runtime for the SDA language.
//!
//! This crate will hold the parser, AST, evaluator, error model, and host-facing
//! data bridges. It is the implementation cornerstone of the wider Axiom family.

/// Placeholder type for the future SDA runtime.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct SdaRuntime;

impl SdaRuntime {
    /// Returns the current crate identity for early workspace validation.
    #[must_use]
    pub fn name() -> &'static str {
        "sda-core"
    }
}
