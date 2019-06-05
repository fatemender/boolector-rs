use std::ffi::CStr;
use std::os::raw::c_char;

use boolector_sys as ffi;

use crate::Solver;

/// Possible value for a bit vector expression in a generated model.
pub struct BitVecAssignment<'a> {
    pub(crate) solver: &'a Solver,
    pub(crate) value_ptr: *const c_char,
}

impl<'a> BitVecAssignment<'a> {
    /// Return a string representation of the assignment.
    ///
    /// Each character in the string is either '0' or '1' for bits with fixed
    /// values, or 'x' for bits with arbitrary values.
    pub fn to_str(&self) -> &str {
        unsafe { CStr::from_ptr(self.value_ptr) }
            .to_str()
            .expect("unexpected character in bit vector assignment")
    }
}

impl<'a> Drop for BitVecAssignment<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::boolector_free_bv_assignment(self.solver.btor_ptr(), self.value_ptr);
        }
    }
}
