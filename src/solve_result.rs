use crate::Model;

/// Result of a `solve` operation.
pub enum SolveResult<'a> {
    /// The formula is satisfiable.  Depending on `Builder` configuration, a
    /// model may be provided.
    Sat(Option<Model<'a>>),

    /// The formula is unsatisfiable.
    Unsat,
}

impl<'a> SolveResult<'a> {
    /// Return whether the result is `Sat`.
    pub fn is_sat(&self) -> bool {
        match self {
            SolveResult::Sat(..) => true,
            SolveResult::Unsat => false,
        }
    }
}
