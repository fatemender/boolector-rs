use std::ffi::CString;
use std::{mem, ptr};

use boolector_sys as ffi;

use crate::{NodeRef, Solver};

/// Sort reference.
pub struct SortRef<'a> {
    solver: &'a Solver,
    sort_ptr: ffi::BoolectorSort,
}

impl<'a> SortRef<'a> {
    /// Return parent solver instance.
    pub fn solver(&self) -> &Solver {
        self.solver
    }

    /// Return whether this sort is an array sort.
    pub fn is_array(&self) -> bool {
        unsafe {
            ffi::boolector_is_array_sort(self.solver.btor_ptr(), self.sort_ptr)
        }
    }

    /// Return whether this sort is a bit vector sort.
    pub fn is_bitvec(&self) -> bool {
        unsafe {
            ffi::boolector_is_bitvec_sort(self.solver.btor_ptr(), self.sort_ptr)
        }
    }

    /// Create a fresh variable of this sort and optionally associate a symbol
    /// with it.
    pub fn var(&self, symbol: Option<&str>) -> NodeRef<'a> {
        let cstr = symbol.and_then(|s| CString::new(s).ok());
        let cstr_ptr = match &cstr {
            Some(s) => s.as_ptr(),
            None => ptr::null(),
        };

        if self.is_array() {
            unsafe {
                NodeRef::from_ffi(self.solver, ffi::boolector_array(
                    self.solver.btor_ptr(),
                    self.sort_ptr,
                    cstr_ptr,
                ))
            }
        } else if self.is_bitvec() {
            unsafe {
                NodeRef::from_ffi(self.solver, ffi::boolector_var(
                    self.solver.btor_ptr(),
                    self.sort_ptr,
                    cstr_ptr,
                ))
            }
        } else {
            panic!("unexpected sort when creating a fresh variable");
        }
    }

    /// Construct a sort reference from raw `BoolectorSort` value and its parent
    /// solver.
    pub unsafe fn from_ffi(solver: &'a Solver, sort_ptr: ffi::BoolectorSort) -> Self {
        SortRef {
            solver,
            sort_ptr,
        }
    }

    /// Consume the sort reference and return the underlying `BoolectorSort`
    /// value.
    pub fn into_ffi(mut self) -> ffi::BoolectorSort {
        mem::replace(&mut self.sort_ptr, ptr::null_mut())
    }

    /// Return the underlying raw `BoolectorSort` value.
    pub fn sort_ptr(&self) -> ffi::BoolectorSort {
        self.sort_ptr
    }
}

impl<'a> Clone for SortRef<'a> {
    fn clone(&self) -> Self {
        SortRef {
            solver: self.solver,
            sort_ptr: unsafe {
                ffi::boolector_copy_sort(self.solver.btor_ptr(), self.sort_ptr)
            },
        }
    }
}

impl<'a> Drop for SortRef<'a> {
    fn drop(&mut self) {
        if !self.sort_ptr.is_null() {
            unsafe {
                ffi::boolector_release_sort(self.solver.btor_ptr(), self.sort_ptr);
            }
        }
    }
}
