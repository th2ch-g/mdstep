
pub mod md;
pub mod observer;
pub mod systemparam;
pub mod variables;

use crate::md::{*};
use crate::variables::{*};
use crate::observer::{*};
use crate::systemparam::{*};

fn main() {

    let mut vars = Variables { atoms: Vec::<Atom>::new(), time: 0.0 };
    vars.init_time();

    let obs = Observer {};

    let mut md = MD {
        vars: vars,
        obs: obs,
        pairs: Vec::new(),
        margin_length: 0.5,
    };

    md.run();
}


