//! Core runtime for output shaping.

/// Placeholder type for the future shaping runtime.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct ShapeRuntime;

impl ShapeRuntime {
    /// Returns the current crate identity for early workspace validation.
    #[must_use]
    pub fn name() -> &'static str {
        "shape-core"
    }
}
