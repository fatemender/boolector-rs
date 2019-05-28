use std::{marker, mem, ptr};

use boolector_sys as ffi;

use crate::{Node, NodeRef, Sort, SortRef};

/// Solver instance.
pub struct Solver<'a> {
    btor_ptr: *mut ffi::Btor,
    _pd: marker::PhantomData<&'a ()>,
}

impl<'a> Solver<'a> {
    /// Create a node instance.
    pub fn node(&'a self, node: Node<'a>) -> NodeRef<'a> {
        node.into_ref(self)
    }

    /// Create a sort instance.
    pub fn sort(&'a self, sort: Sort) -> SortRef<'a> {
        sort.into_ref(self)
    }

    /// Assert an expression.
    pub fn assert(&self, expr: NodeRef<'a>) {
        assert_eq!(self.btor_ptr, expr.solver().btor_ptr);

        unsafe {
            ffi::boolector_assert(self.btor_ptr, expr.node_ptr());
        }
    }

    /// Solve the formula and consume the solver instance.
    pub fn solve(self) {
    }

    /// Construct a solver instance from raw `Btor` pointer.
    pub unsafe fn from_ffi(btor_ptr: *mut ffi::Btor) -> Self {
        Solver {
            btor_ptr,
            _pd: marker::PhantomData,
        }
    }

    /// Consume the solver and return the underlying raw `Btor` pointer.
    pub fn into_ffi(mut self) -> *mut ffi::Btor {
        mem::replace(&mut self.btor_ptr, ptr::null_mut())
    }

    /// Return raw `Btor` pointer.
    pub fn btor_ptr(&self) -> *mut ffi::Btor {
        self.btor_ptr
    }
}

impl<'a> Drop for Solver<'a> {
    fn drop(&mut self) {
        if !self.btor_ptr.is_null() {
            println!("refs: {}", unsafe { ffi::boolector_get_refs(self.btor_ptr) });
            unsafe {
                ffi::boolector_delete(self.btor_ptr);
            }
        }
    }
}
