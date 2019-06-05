use boolector_sys as ffi;

use crate::{NodeRef, Solver};

/// Node description.
///
/// For references to node instances, see [NodeRef](struct.NodeRef.html).
///
/// # Shifts and rotates
///
/// For shift and rotate operations Boolector requires that the first argument
/// has length `n` that is power of two and the second argument has length equal
/// to `log2(n)`.
pub enum Node<'a> {
    /// Integer addition for bit vectors.
    Add(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Bitwise AND for bit vectors.
    And(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Concatenation for bit vectors.
    Concat(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// If-then-else conditional for bit vectors or arrays, the condition must
    /// be a boolean.
    Cond(&'a NodeRef<'a>, &'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Integer decrement for bit vectors.
    Dec(&'a NodeRef<'a>),

    /// Equality for bit vectors or arrays.
    Eq(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Equivalence for booleans.
    Iff(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Integer increment for bit vectors.
    Inc(&'a NodeRef<'a>),

    /// Integer truncating multiplication for bit vectors.
    Mul(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Bitwise NAND for bit vectors.
    NAnd(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Inequality for bit vectors or arrays.
    Ne(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Two's complement (signed integer) negation for bit vectors.
    Neg(&'a NodeRef<'a>),

    /// Bitwise NOR for bit vectors.
    NOr(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// One's complement negation for bit vectors.
    Not(&'a NodeRef<'a>),

    /// Bitwise OR for bit vectors.
    Or(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// AND-reduction for bit vectors.
    RedAnd(&'a NodeRef<'a>),

    /// OR-reduction for bit vectors.
    RedOr(&'a NodeRef<'a>),

    /// XOR-reduction for bit vectors.
    RedXor(&'a NodeRef<'a>),

    /// Rotate left for bit vectors, see note above about shifts and rotates.
    Rol(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Rotate right for bit vectors, see note above about shifts and rotates.
    Ror(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Signed integer addition overflow flag for bit vectors.
    SAddO(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Signed integer division for bit vectors.
    SDiv(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Signed integer division overflow flag for bit vectors.
    SDivO(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Signed integer greater-than comparison for bit vectors.
    SGt(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Signed integer greater-than-or-equal comparison for bit vectors.
    SGte(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Shift left for bit vectors, see note above about shifts and rotates.
    Sll(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Signed integer less-than comparison for bit vectors.
    SLt(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Signed integer less-than-or-equal comparison for bit vectors.
    SLte(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Signed integer modulo (sign matches the divisor sign) for bit vectors.
    SMod(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Signed integer multiplication overflow flag for bit vectors.
    SMulO(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Arithmetic shift right for bit vectors, see note above about shifts and
    /// rotates.
    Sra(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Signed integer remainder for bit vectors.
    SRem(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Logical shift right for bit vectors, see note above about shifts and
    /// rotates.
    Srl(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Signed integer subtraction overflow flag for bit vectors.
    SSubO(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Integer subtraction for bit vectors.
    Sub(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Unsigned integer addition overflow flag for bit vectors.
    UAddO(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Unsigned integer division for bit vectors, returns -1 for division by
    /// zero.
    UDiv(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Unsigned integer greater-than comparison for bit vectors.
    UGt(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Unsigned integer greater-than-or-equal comparison for bit vectors.
    UGte(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Unsigned integer less-than comparison for bit vectors.
    ULt(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Unsigned integer less-than-or-equal comparison for bit vectors.
    ULte(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Unsigned integer multiplication overflow flag for bit vectors.
    UMulO(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Unsigned integer remainder for bit vectors, returns 0 for division by
    /// zero.
    URem(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Unsigned integer subtraction overflow flag for bit vectors.
    USubO(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Bitwise XNOR for bit vectors.
    XNOr(&'a NodeRef<'a>, &'a NodeRef<'a>),

    /// Bitwise XOR for bit vectors.
    Xor(&'a NodeRef<'a>, &'a NodeRef<'a>),
}

impl<'a> Node<'a> {
    /// Create a node reference.
    pub fn into_ref(self, solver: &'a Solver) -> NodeRef<'a> {
        match self {
            Node::Add(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_add),
            Node::And(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_and),
            Node::Concat(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_concat),
            Node::Cond(a, b, c) => Self::into_ref_ffi_3(solver, a, b, c, ffi::boolector_cond),
            Node::Dec(a) => Self::into_ref_ffi_1(solver, a, ffi::boolector_dec),
            Node::Eq(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_eq),
            Node::Iff(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_iff),
            Node::Inc(a) => Self::into_ref_ffi_1(solver, a, ffi::boolector_inc),
            Node::Mul(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_mul),
            Node::NAnd(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_nand),
            Node::Ne(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_ne),
            Node::Neg(a) => Self::into_ref_ffi_1(solver, a, ffi::boolector_neg),
            Node::NOr(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_nor),
            Node::Not(a) => Self::into_ref_ffi_1(solver, a, ffi::boolector_not),
            Node::Or(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_or),
            Node::RedAnd(a) => Self::into_ref_ffi_1(solver, a, ffi::boolector_redand),
            Node::RedOr(a) => Self::into_ref_ffi_1(solver, a, ffi::boolector_redor),
            Node::RedXor(a) => Self::into_ref_ffi_1(solver, a, ffi::boolector_redxor),
            Node::Rol(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_rol),
            Node::Ror(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_ror),
            Node::SAddO(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_saddo),
            Node::SDiv(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_sdiv),
            Node::SDivO(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_sdivo),
            Node::SGt(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_sgt),
            Node::SGte(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_sgte),
            Node::Sll(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_sll),
            Node::SLt(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_slt),
            Node::SLte(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_slte),
            Node::SMod(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_smod),
            Node::SMulO(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_smulo),
            Node::Sra(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_sra),
            Node::SRem(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_srem),
            Node::Srl(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_srl),
            Node::SSubO(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_ssubo),
            Node::Sub(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_sub),
            Node::UAddO(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_uaddo),
            Node::UDiv(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_udiv),
            Node::UGt(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_ugt),
            Node::UGte(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_ugte),
            Node::ULt(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_ult),
            Node::ULte(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_ulte),
            Node::UMulO(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_umulo),
            Node::URem(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_urem),
            Node::USubO(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_usubo),
            Node::XNOr(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_xnor),
            Node::Xor(a, b) => Self::into_ref_ffi_2(solver, a, b, ffi::boolector_xor),
        }
    }
}

type NodeFn1 = unsafe extern fn(*mut ffi::Btor, *mut ffi::BoolectorNode) -> *mut ffi::BoolectorNode;
type NodeFn2 = unsafe extern fn(*mut ffi::Btor, *mut ffi::BoolectorNode, *mut ffi::BoolectorNode) -> *mut ffi::BoolectorNode;
type NodeFn3 = unsafe extern fn(*mut ffi::Btor, *mut ffi::BoolectorNode, *mut ffi::BoolectorNode, *mut ffi::BoolectorNode) -> *mut ffi::BoolectorNode;

impl<'a> Node<'a> {
    #[inline]
    fn into_ref_ffi_1(solver: &'a Solver, a: &'a NodeRef<'a>, f: NodeFn1) -> NodeRef<'a> {
        assert_eq!(solver.btor_ptr(), a.solver().btor_ptr());

        unsafe {
            NodeRef::from_ffi(solver, f(solver.btor_ptr(), a.node_ptr()))
        }
    }

    #[inline]
    fn into_ref_ffi_2(solver: &'a Solver, a: &'a NodeRef<'a>, b: &'a NodeRef<'a>, f: NodeFn2) -> NodeRef<'a> {
        assert_eq!(solver.btor_ptr(), a.solver().btor_ptr());
        assert_eq!(solver.btor_ptr(), b.solver().btor_ptr());

        unsafe {
            NodeRef::from_ffi(solver, f(solver.btor_ptr(), a.node_ptr(), b.node_ptr()))
        }
    }

    #[inline]
    fn into_ref_ffi_3(solver: &'a Solver, a: &'a NodeRef<'a>, b: &'a NodeRef<'a>, c: &'a NodeRef<'a>, f: NodeFn3) -> NodeRef<'a> {
        assert_eq!(solver.btor_ptr(), a.solver().btor_ptr());
        assert_eq!(solver.btor_ptr(), b.solver().btor_ptr());
        assert_eq!(solver.btor_ptr(), c.solver().btor_ptr());

        unsafe {
            NodeRef::from_ffi(solver, f(solver.btor_ptr(), a.node_ptr(), b.node_ptr(), c.node_ptr()))
        }
    }
}
