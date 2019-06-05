use std::ops;

use crate::{Node, NodeRef};

/// Unsigned marker for `NodeRef`.
#[derive(Clone)]
#[repr(transparent)]
pub struct Unsigned<'a>(pub NodeRef<'a>);

impl<'a> Unsigned<'a> {
    pub fn lt(&'a self, other: &'a Self) -> NodeRef<'a> {
        Node::ULt(&*self, &*other).into_ref(self.0.solver())
    }
}

impl<'a> ops::Add for &'a Unsigned<'a> {
    type Output = Unsigned<'a>;

    fn add(self, other: Self) -> Self::Output {
        Unsigned(Node::Add(&*self, &*other).into_ref(self.0.solver()))
    }
}

impl<'a> ops::BitAnd for &'a Unsigned<'a> {
    type Output = Unsigned<'a>;

    fn bitand(self, other: Self) -> Self::Output {
        Unsigned(Node::And(&*self, &*other).into_ref(self.0.solver()))
    }
}

impl<'a> ops::BitOr for &'a Unsigned<'a> {
    type Output = Unsigned<'a>;

    fn bitor(self, other: Self) -> Self::Output {
        Unsigned(Node::Or(&*self, &*other).into_ref(self.0.solver()))
    }
}

impl<'a> ops::BitXor for &'a Unsigned<'a> {
    type Output = Unsigned<'a>;

    fn bitxor(self, other: Self) -> Self::Output {
        Unsigned(Node::Xor(&*self, &*other).into_ref(self.0.solver()))
    }
}

impl<'a> ops::Deref for Unsigned<'a> {
    type Target = NodeRef<'a>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> ops::Div for &'a Unsigned<'a> {
    type Output = Unsigned<'a>;

    fn div(self, other: Self) -> Self::Output {
        Unsigned(Node::UDiv(&*self, &*other).into_ref(self.0.solver()))
    }
}

impl<'a> ops::Mul for &'a Unsigned<'a> {
    type Output = Unsigned<'a>;

    fn mul(self, other: Self) -> Self::Output {
        Unsigned(Node::Mul(&*self, &*other).into_ref(self.0.solver()))
    }
}

impl<'a> ops::Not for &'a Unsigned<'a> {
    type Output = Unsigned<'a>;

    fn not(self) -> Self::Output {
        Unsigned(Node::Not(&*self).into_ref(self.0.solver()))
    }
}

impl<'a> ops::Rem for &'a Unsigned<'a> {
    type Output = Unsigned<'a>;

    fn rem(self, other: Self) -> Self::Output {
        Unsigned(Node::URem(&*self, &*other).into_ref(self.0.solver()))
    }
}

impl<'a> ops::Shl for &'a Unsigned<'a> {
    type Output = Unsigned<'a>;

    fn shl(self, other: Self) -> Self::Output {
        Unsigned(Node::Sll(&*self, &*other).into_ref(self.0.solver()))
    }
}

impl<'a> ops::Shr for &'a Unsigned<'a> {
    type Output = Unsigned<'a>;

    fn shr(self, other: Self) -> Self::Output {
        Unsigned(Node::Srl(&*self, &*other).into_ref(self.0.solver()))
    }
}

impl<'a> ops::Sub for &'a Unsigned<'a> {
    type Output = Unsigned<'a>;

    fn sub(self, other: Self) -> Self::Output {
        Unsigned(Node::Sub(&*self, &*other).into_ref(self.0.solver()))
    }
}
