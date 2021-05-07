use super::*;


// #[inline]
// pub fn mesonic(cutoff: Real, p4: &Momentum4, mass: Real) -> Real {
//     (cutoff.powi(2) - mass.powi(2)) / (cutoff.powi(2) - p4.norm_sq())
// }

// #[inline]
// pub fn baryonic(cutoff: Real, p4: &Momentum4, mass: Real) -> Real {
//     cutoff.powi(4) / ( cutoff.powi(4) + (p4.norm_sq() - mass.powi(2)).powi(2) )
// }

// #[inline]
// pub fn common(
//     m_cutoff: Real, m_p4: &Momentum4, m_mass: Real,
//     b_cutoff: Real, b_p4: &Momentum4, b_mass: Real,
// ) -> Real {    
//     1. - (
//         (1. - mesonic(m_cutoff, m_p4, m_mass))
//         * (1. - baryonic(b_cutoff, b_p4, b_mass))
//     )
// }