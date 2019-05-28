/// Model generation mode.
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u32)]
pub enum GenerateModel {
    /// Generate model for asserted expressions.
    Asserted = 1,

    /// Generate model for all expressions.
    All = 2,
}
