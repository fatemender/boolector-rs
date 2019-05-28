use std::ops;

use crate::{Node, NodeRef};

/// Unsigned marker for `NodeRef`.
#[derive(Clone)]
#[repr(transparent)]
pub struct Unsigned<'a>(pub NodeRef<'a>);

impl<'a> Unsigned<'a> {
    pub fn lt(self, other: Self) -> NodeRef<'a> {
        let solver = self.0.solver();
        Node::ULt(self.0, other.0).into_ref(solver)
    }
}

impl<'a> ops::Add for Unsigned<'a> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let solver = self.0.solver();
        Unsigned(Node::Add(self.0, other.0).into_ref(solver))
    }
}

impl<'a> ops::BitAnd for Unsigned<'a> {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        let solver = self.0.solver();
        Unsigned(Node::And(self.0, other.0).into_ref(solver))
    }
}

impl<'a> ops::BitOr for Unsigned<'a> {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        let solver = self.0.solver();
        Unsigned(Node::Or(self.0, other.0).into_ref(solver))
    }
}

impl<'a> ops::BitXor for Unsigned<'a> {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        let solver = self.0.solver();
        Unsigned(Node::Xor(self.0, other.0).into_ref(solver))
    }
}

impl<'a> ops::Deref for Unsigned<'a> {
    type Target = NodeRef<'a>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> ops::Div for Unsigned<'a> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let solver = self.0.solver();
        Unsigned(Node::UDiv(self.0, other.0).into_ref(solver))
    }
}

impl<'a> ops::Mul for Unsigned<'a> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let solver = self.0.solver();
        Unsigned(Node::Mul(self.0, other.0).into_ref(solver))
    }
}

impl<'a> ops::Not for Unsigned<'a> {
    type Output = Self;

    fn not(self) -> Self {
        let solver = self.0.solver();
        Unsigned(Node::Not(self.0).into_ref(solver))
    }
}

impl<'a> ops::Rem for Unsigned<'a> {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        let solver = self.0.solver();
        Unsigned(Node::URem(self.0, other.0).into_ref(solver))
    }
}

impl<'a> ops::Shl for Unsigned<'a> {
    type Output = Self;

    fn shl(self, other: Self) -> Self {
        let solver = self.0.solver();
        Unsigned(Node::Sll(self.0, other.0).into_ref(solver))
    }
}

impl<'a> ops::Shr for Unsigned<'a> {
    type Output = Self;

    fn shr(self, other: Self) -> Self {
        let solver = self.0.solver();
        Unsigned(Node::Srl(self.0, other.0).into_ref(solver))
    }
}

impl<'a> ops::Sub for Unsigned<'a> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let solver = self.0.solver();
        Unsigned(Node::Sub(self.0, other.0).into_ref(solver))
    }
}
