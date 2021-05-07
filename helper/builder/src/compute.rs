use super::*;


pub mod dcs {
    use super::*;

    pub fn all_angles(mut pp: VectorInvPhotoproduction, energy_lab: f64) -> Vec<(f64, f64)> {
        let mut computed: Vec<(f64, f64)> = vec![];

        pp.set_energy(Energy::InLabFrame(energy_lab));

        let mut cos_theta: f64 = 1.;
        while cos_theta >= -1. {
            pp.set_angle(Angle::InRadian(cos_theta.acos()));

            let d_cs = differential(&pp, DifferentialUnit::DifCosTheta);

            let angle = (cos_theta * 100.0).round() / 100.0;

            computed.push((angle, d_cs));

            cos_theta -= 0.01;
        }

        return computed;
    }
}