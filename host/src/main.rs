use host::prove_mod_exp_within_range;
use risc0_zkvm::serde::from_slice;
use std::io;

fn main() {
    // Define variables to store user input
    let mut base = String::new();
    let mut modulus = String::new();
    let mut range = String::new();
    let mut exp = String::new();

    // Prompt the user for input
    println!("Enter the base (u64):");
    io::stdin()
        .read_line(&mut base)
        .expect("Failed to read base");

    println!("Enter the modulus (u64):");
    io::stdin()
        .read_line(&mut modulus)
        .expect("Failed to read modulus");

    println!("Enter the range (u64):");
    io::stdin()
        .read_line(&mut range)
        .expect("Failed to read range");

    println!("Enter the exponent (u64):");
    io::stdin()
        .read_line(&mut exp)
        .expect("Failed to read exponent");

    // Parse the input into u64 values
    let base: u64 = base.trim().parse().expect("Invalid base");
    let modulus: u64 = modulus.trim().parse().expect("Invalid modulus");
    let range: u64 = range.trim().parse().expect("Invalid range");
    let exp: u64 = exp.trim().parse().expect("Invalid exponent");

    println!("Calculating proof...");
    // Generate and verify a proof
    let (receipt, proof_time) = prove_mod_exp_within_range(base, modulus, range, exp).unwrap();

    let (base_res, modulus_res, range_res, result) =
        from_slice::<(u64, u64, u64, u64), _>(&receipt.journal)
            .unwrap()
            .try_into()
            .unwrap();

    println!("Proof complete!");
    println!("base: {:?}", base_res);
    println!("modulus: {:?}", modulus_res);
    println!("range: {:?}", range_res);
    println!("result: {:?}", result);
    println!("Elapsed time: {:?}", proof_time);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_exp() {
        let base: u64 = 4;
        let modulus: u64 = 497;
        let range: u64 = 14;
        let exp: u64 = 13;

        let (receipt, _) = prove_mod_exp_within_range(base, modulus, range, exp).unwrap();

        let (base_res, modulus_res, range_res, result) =
            from_slice::<(u64, u64, u64, u64), _>(&receipt.journal)
                .unwrap()
                .try_into()
                .unwrap();

        assert_eq!(base, base_res);
        assert_eq!(modulus, modulus_res);
        assert_eq!(range, range_res);
        assert_eq!(result, 445);
    }

    #[test]
    #[should_panic(expected = "Exp is not in range")]
    fn test_invalid_exp() {
        let base: u64 = 4;
        let modulus: u64 = 497;
        let range: u64 = 12;
        let exp: u64 = 13;

        prove_mod_exp_within_range(base, modulus, range, exp).unwrap();
    }
}
