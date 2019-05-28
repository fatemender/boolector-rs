use boolector_sys as ffi;

use crate::{Solver, SortRef};

/// Sort description.
///
/// For references to sort instances, see [SortRef](struct.SortRef.html).
pub enum Sort {
    /// Bit vector of given length.
    BitVec(u32),
}

impl Sort {
    /// Create a sort reference.
    pub fn into_ref<'a>(self, solver: &'a Solver<'a>) -> SortRef<'a> {
        match self {
            Sort::BitVec(bits) => unsafe {
                SortRef::from_ffi(
                    solver,
                    ffi::boolector_bitvec_sort(solver.btor_ptr(), bits),
                )
            },
        }
    }
}
