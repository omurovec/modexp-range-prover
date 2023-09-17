#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 { return 0 }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

pub fn main() {
    let base: u64 = env::read();
    let modulus: u64 = env::read();
    let range: u64 = env::read();
    let exp: u64 = env::read();

    let result = mod_pow(base, exp, modulus);

    if exp > range {
        panic!("Exp is not in range")
    }

    env::commit(&(base, modulus, range, result));
}
