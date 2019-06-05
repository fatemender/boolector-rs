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
//! let solver = boolector::Builder::new()
//!     .generate_model(boolector::GenerateModel::Asserted)
//!     .finish();
//!
//! // Create an 8-bit sort instance.
//! let b8 = solver.sort(boolector::Sort::BitVec(8));
//!
//! // Create two variables and mark them as unsigned.
//! let x = boolector::Unsigned(b8.var(Some("x")));
//! let y = boolector::Unsigned(b8.var(Some("y")));
//!
//! // Build `((x + y) < x) & ((x + y) < y)`.
//! let sum = &x + &y;
//! let sum_lt_x = sum.lt(&x);
//! let sum_lt_y = sum.lt(&y);
//! let formula = &sum_lt_x & &sum_lt_y;
//!
//! // Assert the formula.
//! solver.assert(&formula);
//!
//! // Solve it: should be satisfiable.
//! assert!(solver.solve().is_sat());
//! ```

pub use self::bit_vec_assignment::*;
pub use self::builder::*;
pub use self::generate_model::*;
pub use self::model::*;
pub use self::node::*;
pub use self::node_ref::*;
pub use self::solve_result::*;
pub use self::solver::*;
pub use self::sort::*;
pub use self::sort_ref::*;
pub use self::unsigned::*;

mod bit_vec_assignment;
mod builder;
mod generate_model;
mod model;
mod node;
mod node_ref;
mod solve_result;
mod solver;
mod sort;
mod sort_ref;
mod unsigned;
