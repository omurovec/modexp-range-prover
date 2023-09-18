#![no_main]
#![no_std]

extern crate num_bigint as bigint;
extern crate num_traits;

use bigint::{BigUint, ToBigUint};
use risc0_zkvm::guest::env;
use num_traits::{One, Zero};

risc0_zkvm::guest::entry!(main);

fn mod_pow(base: BigUint, exp: BigUint, modulus: BigUint) -> BigUint {
    let one = BigUint::one();
    if modulus == one {
        return BigUint::zero();
    }

    let mut result = one.clone();
    let mut base = base % &modulus;
    let mut exp = exp.clone();

    while exp > one {
        if &exp % 2u32.to_biguint().unwrap() == one {
            result = (&result * &base) % &modulus;
        }
        exp = exp >> 1;
        base = (&base * &base) % &modulus;
    }
    result
}

pub fn main() {
    let base: BigUint = env::read();
    let modulus: BigUint = env::read();
    let range: BigUint = env::read();
    let exp: BigUint = env::read();

    let result = mod_pow(base.clone(), exp.clone(), modulus.clone());

    if exp > range {
        panic!("Exp is not in range");
    }

    env::commit(&(base, modulus, range, result));
}
