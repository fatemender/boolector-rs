//! High-level type-safe bindings for the [Boolector] SMT solver.
//!
//! [Boolector]: https://boolector.github.io/
//!
//! # Quick start
//!
//! Let's show that there exist two 8-bit unsigned integers such that their sum
//! is less than either of them (this is true when an unsigned overflow occurs).
//!
//! ```
//! // Create a solver instance and enable model generation for asserted expressions.
//! let mut solver = boolector::Builder::new()
//!     .generate_model(boolector::GenerateModel::Asserted)
//!     .finish();
//!
//! // Create an 8-bit sort instance.
//! let b8 = solver.sort(boolector::Sort::BitVec(8));
//!
//! // Create two variables.
//! let x = b8.var(Some("x"));
//! let y = b8.var(Some("y"));
//! ```

pub use self::builder::Builder;
pub use self::generate_model::GenerateModel;
pub use self::node::Node;
pub use self::node_ref::NodeRef;
pub use self::solver::Solver;
pub use self::sort::Sort;
pub use self::sort_ref::SortRef;
pub use self::unsigned::Unsigned;

mod builder;
mod generate_model;
mod node;
mod node_ref;
mod solver;
mod sort;
mod sort_ref;
mod unsigned;
