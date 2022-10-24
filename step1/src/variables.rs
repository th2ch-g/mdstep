
use rand::Rng;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct Atom {

    pub qx: f64,
    pub qy: f64,
    pub qz: f64,
    pub px: f64,
    pub py: f64,
    pub pz: f64,

}

impl Atom {
    pub fn init() -> Atom {
        Atom {
            qx: 0.0,
            qy: 0.0,
            qz: 0.0,
            px: 0.0,
            py: 0.0,
            pz: 0.0,
        }
    }
}


pub struct Variables{

    pub atoms: Vec<Atom>,
    pub time: f64,

}

impl Variables {

    pub fn init_time(&mut self) {
        self.time = 0.0;
    }

    pub fn add_atoms(&mut self, x: f64, y: f64, z: f64) {

        let mut a: Atom = Atom::init();
        a.qx = x;
        a.qy = y;
        a.qz = z;
        a.px = 0.0;
        a.py = 0.0;
        a.pz = 0.0;
        self.atoms.push(a);

    }

    pub fn set_initial_velocity(&mut self, v0: f64) {

        let mut rng = rand::thread_rng();

        let mut avx = 0.0;
        let mut avy = 0.0;
        let mut avz = 0.0;

        for i in 0..self.atoms.len() {

            let ud: f64 = rng.gen();
            let z = ud * 2.0 - 1.0;
            let phi = 2.0 * ud * std::f64::consts::PI;
            let vx = v0 * (1.0 - z * z).sqrt() * phi.cos();
            let vy = v0 * (1.0 - z * z).sqrt() * phi.sin();
            let vz = v0 * z;
            self.atoms[i].px = vx;
            self.atoms[i].py = vy;
            self.atoms[i].pz = vz;
            avx += vx;
            avy += vy;
            avz += vz;

        }

        let pn = self.atoms.len() as f64;
        avx /= pn;
        avy /= pn;
        avz /= pn;

        for i in 0..self.atoms.len() {
            self.atoms[i].px -= avx;
            self.atoms[i].py -= avy;
            self.atoms[i].pz -= avz;
        }

    }


    pub fn export_cdview(&self, id: i32) {

        let mut cdv_file = File::create(&format!("out.{}.cdv", id)).unwrap();

        for i in 0..self.atoms.len() {

            cdv_file.write_all(format!("{} 0 {} {} {}\n",
                     i, self.atoms[i].qx, self.atoms[i].qy, self.atoms[i].qz).as_bytes()).unwrap();

        }

    }

    // pub fn export_vmd(&self, id: i32, time: f64, box_size: f64) {
    //
    //     let mut vmd_file = File::create(&format!("out.{}.gro", id)).unwrap();
    //
    //     vmd_file.write_all(format!("MD of H, t={}\n{}\n", time, &self.atoms.len()).as_bytes()).unwrap();
    //
    //     for i in 0..self.atoms.len() {
    //         vmd_file.write_all(format!("{}TIP3 A {} {} {} {}\n", i+1, i+1, &self.atoms[i].qx, &self.atoms[i].qy, &self.atoms[i].qz).as_bytes()).unwrap();
    //     }
    //
    //     vmd_file.write_all(format!("{} {} {} {} {} {}\n", box_size, box_size, box_size, 0.0, 0.0, 0.0).as_bytes()).unwrap();
    //
    // }
    //

}

