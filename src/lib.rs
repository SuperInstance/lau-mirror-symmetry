#![deny(unsafe_code)]
//! # lau-mirror-symmetry
//!
//! Mirror symmetry — the duality between symplectic and complex geometry.
//!
//! This crate implements the mathematical structures underlying mirror symmetry:
//! - **Calabi-Yau manifolds** with Ricci-flat Kähler metrics and SU(n) holonomy
//! - **Hodge diamonds** for Calabi-Yau 3-folds, including h^{1,1} and h^{2,1} mirror pairs
//! - **Gromov-Witten invariants** counting pseudo-holomorphic curves
//! - **Quantum cohomology** with deformed cup products
//! - **Mirror maps** between A-model and B-model geometries
//! - **Homological mirror symmetry** (Fukaya category ↔ derived category)
//! - **Monodromy** and Picard-Fuchs equations
//! - **Agent dualities**: two agent architectures computing the same invariants

pub mod calabi_yau;
pub mod hodge;
pub mod gromov_witten;
pub mod quantum_cohomology;
pub mod mirror_map;
pub mod homological;
pub mod picard_fuchs;
pub mod agent_duality;

pub use calabi_yau::CalabiYauManifold;
pub use hodge::{HodgeDiamond, HodgeNumber};
pub use gromov_witten::{GromovWittenInvariant, CurveClass};
pub use quantum_cohomology::QuantumCohomologyRing;
pub use mirror_map::MirrorMap;
pub use homological::{FukayaCategory, DerivedCategory, HMSDuality};
pub use picard_fuchs::PicardFuchsEquation;
pub use agent_duality::AgentDuality;
