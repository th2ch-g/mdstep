
pub struct params {

    L : f64,
    dt : f64,
    CUTOFF : f64,
    CL2 : f64,
    RC2 : f64,
    RC6 : f64,
    RC12 : f64,
    C0 : f64,
    epsilon: f64,

}

impl params {

    pub const L : f64 = 10.0;
    pub const dt : f64 = 0.01;
    pub const CUTOFF : f64 = 2.0;
    pub const CL2 : f64 = Self::CUTOFF * Self::CUTOFF;
    pub const RC2 : f64 = 1.0 / Self::CL2;
    pub const RC6 : f64 = Self::RC2 * Self::RC2 * Self::RC2;
    pub const RC12 : f64 = Self::RC6 * Self::RC6;
    pub const C0 : f64 = - 4.0 * (Self::RC12 - Self::RC6);
    pub const epsilon: f64 = 1e-38;

}


pub fn adjust_periodic(mut dx: f64, mut dy: f64, mut dz: f64) -> (f64, f64, f64){

    let L = params::L;
    let LH: f64 = L * 0.5;

    if dx < -LH { dx += L; }
    if dx > LH { dx -= L; }
    if dy < -LH { dy += L; }
    if dy > LH { dy -= L; }
    if dz < -LH { dz += L; }
    if dz > LH { dz -= L; }

    (dx, dy, dz)
}


