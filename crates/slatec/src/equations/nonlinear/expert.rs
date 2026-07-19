//! Expert nonlinear-system solving.
//!
//! # Status: Implemented
//!
//! Enable `nonlinear-expert`.

#[cfg(feature = "nonlinear-expert")]
pub use crate::nonlinear::{
    ExpertNonlinearOptions, ExpertNonlinearResult, JacobianIndexError, JacobianMut,
    JacobianStructure, NonlinearError, VariableScaling, solve_system_expert,
    solve_system_expert_f32, solve_system_with_jacobian, solve_system_with_jacobian_f32,
};
