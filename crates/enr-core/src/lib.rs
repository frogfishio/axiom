//! Core runtime for enrichment and join semantics.

/// Placeholder type for the future enrichment runtime.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct EnrRuntime;

impl EnrRuntime {
    /// Returns the current crate identity for early workspace validation.
    #[must_use]
    pub fn name() -> &'static str {
        "enr-core"
    }
}
