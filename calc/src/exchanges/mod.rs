//! # Examples
//!     - Photoproduction: Vector
//!     - Channel: T
//!     - Particle: Kaon
//! 
//! `
//! pub struct TKaon {
//!     pub coupling_constants: Real,
//!     pub ff_cutoff: Real,
//! 
//!     // Additional fields related
//!     // to t-ch Kaon exchange.
//! }
//! 
//! impl ExchangeBase<VectorParticleInv> for TKaon {
//!     fn calc_invariant_amplitude(
//!         &self,
//!         rx: &RxData,
//!         states: VectorParticleInv,
//!     ) -> StateMatrix {
//!         // TODO: Particle info declarations 
//!         // TODO: Constants calculations 
//!         // TODO: Your derivations here 
//!     }
//! }
//! `


use super::*;
use crate::common::{
    const_pdg as pdg,
    form_factors::{
        // baryonic as ff_b,
        // mesonic  as ff_m,
        // common   as ff_c,
    }
};

use hep_framework::{
    constants::fundamental::hep_units::UNIT_ELEMENTARY_CHARGE,
    math::lorentz::*,
    quantum::relativistic::*,
};

pub mod t;
pub mod s;
pub mod u;