
use crate::variables::{*};
use crate::systemparam::{*};


pub struct Observer {

}

impl Observer {

    pub fn kinetic_energy(&self, vars: &Variables) -> f64 {

        let mut k: f64 = 0.0;

        for i in 0..vars.atoms.len() {
            k += vars.atoms[i].px * vars.atoms[i].px;
            k += vars.atoms[i].py * vars.atoms[i].py;
            k += vars.atoms[i].pz * vars.atoms[i].pz;
        }

        k /= vars.atoms.len() as f64;

        k * 0.5
    }

    pub fn potential_energy(&self, vars: &Variables, pairs: &Vec<Pair>) -> f64 {

        let C0: f64 = params::C0;
        let CL2: f64 = params::CL2;

        let mut v = 0.0;
        let pn = vars.atoms.len();
        let pp = pairs.len();
        for k in 0..pp {
            let i = pairs[k].i as usize;
            let j = pairs[k].j as usize;
            let mut dx: f64 = vars.atoms[j].qx - vars.atoms[i].qx;
            let mut dy: f64 = vars.atoms[j].qy - vars.atoms[i].qy;
            let mut dz: f64 = vars.atoms[j].qz - vars.atoms[i].qz;
            (dx, dy, dz) = adjust_periodic(dx, dy, dz);
            let r2 = dx * dx + dy * dy + dz * dz;
            if r2 > params::CL2 { continue }
            let r6 = r2 * r2 * r2;
            let r12 = r6 * r6;
            v += params::epsilon * ( 4.0 * (1.0/r12 - 1.0/r6) + params::C0 );
        }
        v /= pn as f64;

        v
    }

}


