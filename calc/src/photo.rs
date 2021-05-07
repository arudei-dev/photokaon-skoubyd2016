//! # Examples: Kaonstar Photoproduction
//! 
//! `
//! pub fn init_kaonstar(charge: Charge) -> VectorInvPhotoproduction {
//!     let (mass_inc_b, mass_out_c) = match charge {
//!         Charge::Positive => (
//!             pdg::baryon::nucl::proton::MASS_MEV,
//!             crate::common::const_pdg::MEV__KAONSTAR_PLUS_MASS,
//!         ),
//!         Charge::Neutral => (
//!             pdg::baryon::nucl::neutron::MASS_MEV,
//!             crate::common::const_pdg::MEV__KAONSTAR_ZERO_MASS,
//!         ),
//!         _ => panic!(
//!             "Only neutral and (+)-charged reaction is supported."
//!         ),
//!     };
//! 
//!     return VectorInvPhotoproduction::init_with(
//!         RxData {
//!             charge,
//!             p_inc_a: Particle::zero(),
//!             p_inc_b: Particle::init_with_mass(mass_inc_b),
//!             p_out_c: Particle::init_with_mass(mass_out_c),
//!             p_out_d: Particle::init_with_mass(
//!                 crate::common::const_pdg::MEV__LAMBDA_MASS,
//!             ),
//!         },
//!         AmplsConfig {
//!             spin_avg: 4.,
//!             h_photon: vec![Helicity::Left, Helicity::Right],
//!             h_out_c:  vec![Helicity::Left, Helicity::Right],
//!             s_inc_b:  vec![Spin::Up, Spin::Down],
//!             s_out_d:  vec![Spin::Up, Spin::Down],
//!         },
//!     );
//! }
//! `


use super::*;

