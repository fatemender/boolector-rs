use boolector_sys as ffi;

use crate::{BitVecAssignment, NodeRef, Solver};

/// Model generated for a satisfiable formula.
pub struct Model<'a> {
    pub(crate) solver: &'a Solver,
}

impl<'a> Model<'a> {
    /// Return parent solver instance.
    pub fn solver(&self) -> &Solver {
        self.solver
    }

    /// Return assignment for a bit vector expression.
    pub fn bit_vec(&'a self, expr: &'a NodeRef<'a>) -> BitVecAssignment<'a> {
        assert_eq!(self.solver().btor_ptr(), expr.solver().btor_ptr());

        BitVecAssignment {
            solver: self.solver(),
            value_ptr: unsafe {
                ffi::boolector_bv_assignment(
                    self.solver.btor_ptr(),
                    expr.node_ptr(),
                )
            },
        }
    }
}
