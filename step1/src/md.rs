
use crate::systemparam::{*};
use crate::variables::{*};
use crate::observer::{*};

use std::fs::File;
use std::io::Write;

pub struct MD {

    pub vars: Variables,
    pub obs: Observer,

}


impl MD {

    pub fn makeconf(&mut self) {

        let DENSITY: f64 = 0.05; //ok epsilon = 1e-38
        //let DENSITY: f64 = 0.5; // ok epsilon = 1e-55
        let s: f64 = 1.0 / (DENSITY * 0.25).powf(1.0/3.0);
        let hs: f64 = s * 0.5;
        let is: i32 = (params::L / s) as i32;

        for iz in 0..is {
            for iy in 0..is {
                for ix in 0..is {
                    self.vars.add_atoms(ix as f64 * s, iy as f64 * s, iz as f64 * s);
                    self.vars.add_atoms(ix as f64 * s + hs, iy as f64 * s, iz as f64 * s);
                    self.vars.add_atoms(ix as f64 * s, iy as f64 * s + hs, iz as f64 * s);
                    self.vars.add_atoms(ix as f64 * s, iy as f64 * s, iz as f64 * s + hs);
                }
            }
        }
        println!("Atom is {}", &self.vars.atoms.len());
        //self.vars.set_initial_velocity(1.0);
        self.vars.set_initial_velocity(0.5);
    }

    pub fn update_position(&mut self) {

        let dt2: f64 = params::dt * 0.5;
        for i in 0..self.vars.atoms.len() {
            self.vars.atoms[i].qx += self.vars.atoms[i].px * dt2;
            self.vars.atoms[i].qy += self.vars.atoms[i].py * dt2;
            self.vars.atoms[i].qz += self.vars.atoms[i].pz * dt2;
        }

    }

    pub fn calculate_force(&mut self) {

        let pn = self.vars.atoms.len();

        for i in 0..pn-1 {
            for j in i+1..pn {
                let mut dx = self.vars.atoms[j].qx - self.vars.atoms[i].qx;
                let mut dy = self.vars.atoms[j].qy - self.vars.atoms[i].qy;
                let mut dz = self.vars.atoms[j].qz - self.vars.atoms[i].qz;
                (dx, dy, dz) = adjust_periodic(dz, dy, dz);
                let r2 = dx * dx + dy * dy + dz * dz;
                if r2 > params::CL2 { continue }
                let r6 = r2 * r2 * r2;
                let r13 = r2.powf(6.5);
                let r7 = r2.powf(3.5);
                //let df = (24.0 * r6 - 48.0) / (r6 * r6 * r2) * params::dt;
                let df = params::epsilon *  (48.0 - 24.0 * r6) / r13 * params::dt;
                self.vars.atoms[i].px += df * dx;
                self.vars.atoms[i].py += df * dy;
                self.vars.atoms[i].pz += df * dz;
                self.vars.atoms[j].px -= df * dx;
                self.vars.atoms[j].py -= df * dy;
                self.vars.atoms[j].pz -= df * dz;
            }
        }

    }

    pub fn periodic(&mut self) {

        let L = params::L;
        for i in 0..self.vars.atoms.len() {
            if self.vars.atoms[i].qx < 0.0 { self.vars.atoms[i].qx += L; }
            if self.vars.atoms[i].qy < 0.0 { self.vars.atoms[i].qy += L; }
            if self.vars.atoms[i].qz < 0.0 { self.vars.atoms[i].qz += L; }
            if self.vars.atoms[i].qx > L { self.vars.atoms[i].qx -= L; }
            if self.vars.atoms[i].qy > L { self.vars.atoms[i].qy -= L; }
            if self.vars.atoms[i].qz > L { self.vars.atoms[i].qz -= L; }

            assert!(self.vars.atoms[i].qx < L, "qx > L : {}", &self.vars.atoms[i].qx);
            assert!(self.vars.atoms[i].qy < L, "qy > L : {}", &self.vars.atoms[i].qy);
            assert!(self.vars.atoms[i].qz < L, "qz > L : {}", &self.vars.atoms[i].qz);
        }
    }

    pub fn calculate(&mut self) {
        self.update_position();
        self.periodic(); // add
        self.calculate_force();
        self.update_position();
        self.periodic();
        self.vars.time += params::dt;
    }

    pub fn run(&mut self) {
        self.makeconf();
        let STEPS: i32 = 10000;
        let OBSERVE: i32 = 100;

        let mut energy_file = File::create("out.energy").unwrap();
        for i in 0..STEPS {
            if i % OBSERVE == 0 {
                let k = self.obs.kinetic_energy(&self.vars);
                let v = self.obs.potential_energy(&self.vars);
                energy_file.write_all(
                    format!("{} {} {} {}\n", self.vars.time as i32, k, v, k+v).as_bytes()).unwrap();
                self.vars.export_cdview(i);
                //self.vars.export_vmd(i, self.vars.time, params::L);
            }

            self.calculate();

        }
    }
}


