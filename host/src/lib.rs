use methods::{MOD_EXP_IN_RANGE_ELF, MOD_EXP_IN_RANGE_ID};
use num_bigint::BigUint;
use risc0_zkvm::serde::{to_vec, Error};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use std::time::{Duration, Instant};

pub fn prove_mod_exp_within_range(
    base: BigUint,
    modulus: BigUint,
    range: BigUint,
    exp: BigUint,
) -> Result<(Receipt, Duration), Error> {
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&base.clone()).unwrap()) // base
        .add_input(&to_vec(&modulus.clone()).unwrap()) // modulus
        .add_input(&to_vec(&range.clone()).unwrap()) // range
        .add_input(&to_vec(&exp.clone()).unwrap()) // exp
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let start_time = Instant::now();
    let receipt = prover.prove_elf(env, MOD_EXP_IN_RANGE_ELF).unwrap();
    let end_time = Instant::now();

    receipt.verify(MOD_EXP_IN_RANGE_ID).unwrap();

    Ok((receipt, end_time.duration_since(start_time)))
}
