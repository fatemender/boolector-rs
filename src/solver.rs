use std::{mem, ptr};

use boolector_sys as ffi;

use crate::{Model, Node, NodeRef, SolveResult, Sort, SortRef};

/// Solver instance.
pub struct Solver {
    btor_ptr: *mut ffi::Btor,
}

impl Solver {
    /// Create a node instance.
    pub fn node<'a>(&'a self, node: Node<'a>) -> NodeRef<'a> {
        node.into_ref(self)
    }

    /// Create a sort instance.
    pub fn sort<'a>(&'a self, sort: Sort<'a>) -> SortRef<'a> {
        sort.into_ref(self)
    }

    /// Assert an expression.
    pub fn assert<'a>(&'a self, expr: &'a NodeRef<'a>) {
        assert_eq!(self.btor_ptr, expr.solver().btor_ptr);

        unsafe {
            ffi::boolector_assert(self.btor_ptr, expr.node_ptr());
        }
    }

    /// Solve the formula.
    pub fn solve(&self) -> SolveResult {
        let result = unsafe {
            ffi::boolector_sat(self.btor_ptr)
        };

        if result == ffi::BtorSolverResult_BTOR_RESULT_SAT as i32 {
            let model_was_generated = unsafe {
                ffi::boolector_get_opt(
                    self.btor_ptr,
                    ffi::BtorOption_BTOR_OPT_MODEL_GEN,
                )
            };

            if model_was_generated != 0 {
                SolveResult::Sat(Some(Model { solver: self }))
            } else {
                SolveResult::Sat(None)
            }
        } else if result == ffi::BtorSolverResult_BTOR_RESULT_UNSAT as i32 {
            SolveResult::Unsat
        } else {
            panic!("unexpected return value from boolector_sat()");
        }
    }

    /// Construct a solver instance from raw `Btor` pointer.
    pub unsafe fn from_ffi(btor_ptr: *mut ffi::Btor) -> Self {
        Solver {
            btor_ptr,
        }
    }

    /// Consume the solver instance and return the underlying raw `Btor`
    /// pointer.
    pub fn into_ffi(mut self) -> *mut ffi::Btor {
        mem::replace(&mut self.btor_ptr, ptr::null_mut())
    }

    /// Return raw `Btor` pointer.
    pub fn btor_ptr(&self) -> *mut ffi::Btor {
        self.btor_ptr
    }
}

impl Drop for Solver {
    fn drop(&mut self) {
        if !self.btor_ptr.is_null() {
            println!("refs: {}", unsafe { ffi::boolector_get_refs(self.btor_ptr) });

            unsafe {
                ffi::boolector_delete(self.btor_ptr);
            }
        }
    }
}
