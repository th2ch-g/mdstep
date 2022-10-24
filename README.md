# mdstep
![rust](https://img.shields.io/badge/rust-1.64.0+-blueviolet.svg?logo=rust)
![license](https://img.shields.io/github/license/th2ch-g/mdstep)
![language](https://img.shields.io/github/languages/top/th2ch-g/mdstep)
![last-commit](https://img.shields.io/github/last-commit/th2ch-g/mdstep)

practice of MD simulation
- Rewrite of [kaityo256/mdstep](https://github.com/kaityo256/mdstep) in Rust

- [mdstep](#mdstep)
  - [Usage](#usage)
    - [Caution](#caution)
  - [Condition](#condition)
  - [Algorithms](#algorithms)
  - [Reference](#reference)

## Usage
~~~
cd step1/result  # step2..6 is also same way
cargo run --release
~~~

### Caution
MD may occur an error if the initial speed is not oriented properly.

But it can be done to the end after several runs. (Or due to parameters value)


## Condition
- consider only LJ potential
- only one type atom
- cubic simulationi box
- PB condition (Periodic boundary)


## Algorithms
- Neighbor particle search by mesh
- Using list of interacting particles by bookkeeping method
- Sorting by interaction partners


## Reference
- [kaityo256/mdstep](https://github.com/kaityo256/mdstep)
- [passive-radio/md_in_rust](https://github.com/passive-radio/md_in_rust)

