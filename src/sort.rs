use boolector_sys as ffi;

use crate::{Solver, SortRef};

/// Sort description.
///
/// For references to sort instances, see [SortRef](struct.SortRef.html).
pub enum Sort<'a> {
    /// Array with bit vector indexes and bit vector values.
    Array(&'a SortRef<'a>, &'a SortRef<'a>),

    /// Bit vector of given length.
    BitVec(u32),

    /// Function with given inputs and a single output.
    Fun(&'a [&'a SortRef<'a>], &'a SortRef<'a>),
}

impl<'a> Sort<'a> {
    /// Create a sort reference.
    pub fn into_ref(self, solver: &'a Solver) -> SortRef<'a> {
        match self {
            Sort::Array(index_sort, value_sort) => unsafe {
                SortRef::from_ffi(
                    solver,
                    ffi::boolector_array_sort(
                        solver.btor_ptr(),
                        index_sort.sort_ptr(),
                        value_sort.sort_ptr(),
                    ),
                )
            },
            Sort::BitVec(bits) => unsafe {
                SortRef::from_ffi(
                    solver,
                    ffi::boolector_bitvec_sort(solver.btor_ptr(), bits),
                )
            },
            Sort::Fun(domain, codomain) => unsafe {
                let mut domain_ptrs: Vec<_> = domain
                    .iter()
                    .map(|sort| sort.sort_ptr())
                    .collect();

                SortRef::from_ffi(
                    solver,
                    ffi::boolector_fun_sort(
                        solver.btor_ptr(),
                        domain_ptrs.as_mut_ptr(),
                        domain_ptrs.len() as u32,
                        codomain.sort_ptr(),
                    ),
                )
            },
        }
    }
}
