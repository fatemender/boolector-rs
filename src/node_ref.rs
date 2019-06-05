use std::{mem, ops, ptr};

use boolector_sys as ffi;

use crate::{Node, Solver};

/// Expression node reference.
pub struct NodeRef<'a> {
    solver: &'a Solver,
    node_ptr: *mut ffi::BoolectorNode,
}

impl<'a> NodeRef<'a> {
    /// Return parent solver instance.
    pub fn solver(&self) -> &Solver {
        self.solver
    }

    /// Construct a node reference from raw `BoolectorNode` pointer and its
    /// parent solver.
    pub unsafe fn from_ffi(solver: &'a Solver, node_ptr: *mut ffi::BoolectorNode) -> Self {
        NodeRef {
            solver,
            node_ptr,
        }
    }

    /// Consume the node reference and return the underlying raw `BoolectorNode`
    /// pointer.
    pub fn into_ffi(mut self) -> *mut ffi::BoolectorNode {
        mem::replace(&mut self.node_ptr, ptr::null_mut())
    }

    /// Return the underlying raw `BoolectorNode` pointer.
    pub fn node_ptr(&self) -> *mut ffi::BoolectorNode {
        self.node_ptr
    }
}

impl<'a> Clone for NodeRef<'a> {
    fn clone(&self) -> Self {
        NodeRef {
            solver: self.solver,
            node_ptr: unsafe {
                ffi::boolector_copy(self.solver.btor_ptr(), self.node_ptr)
            },
        }
    }
}

impl<'a> Drop for NodeRef<'a> {
    fn drop(&mut self) {
        if !self.node_ptr.is_null() {
            unsafe {
                ffi::boolector_release(self.solver.btor_ptr(), self.node_ptr);
            }
        }
    }
}

impl<'a> ops::Add for &'a NodeRef<'a> {
    type Output = NodeRef<'a>;

    fn add(self, other: Self) -> Self::Output {
        Node::Add(self, other).into_ref(self.solver())
    }
}

impl<'a> ops::BitAnd for &'a NodeRef<'a> {
    type Output = NodeRef<'a>;

    fn bitand(self, other: Self) -> Self::Output {
        Node::And(self, other).into_ref(self.solver())
    }
}

impl<'a> ops::BitOr for &'a NodeRef<'a> {
    type Output = NodeRef<'a>;

    fn bitor(self, other: Self) -> Self::Output {
        Node::Or(self, other).into_ref(self.solver())
    }
}

impl<'a> ops::BitXor for &'a NodeRef<'a> {
    type Output = NodeRef<'a>;

    fn bitxor(self, other: Self) -> Self::Output {
        Node::Xor(self, other).into_ref(self.solver())
    }
}

impl<'a> ops::Mul for &'a NodeRef<'a> {
    type Output = NodeRef<'a>;

    fn mul(self, other: Self) -> Self::Output {
        Node::Mul(self, other).into_ref(self.solver())
    }
}

impl<'a> ops::Not for &'a NodeRef<'a> {
    type Output = NodeRef<'a>;

    fn not(self) -> Self::Output {
        Node::Not(self).into_ref(self.solver())
    }
}

impl<'a> ops::Shl for &'a NodeRef<'a> {
    type Output = NodeRef<'a>;

    fn shl(self, other: Self) -> Self::Output {
        Node::Sll(self, other).into_ref(self.solver())
    }
}

impl<'a> ops::Sub for &'a NodeRef<'a> {
    type Output = NodeRef<'a>;

    fn sub(self, other: Self) -> Self::Output {
        Node::Sub(self, other).into_ref(self.solver())
    }
}
