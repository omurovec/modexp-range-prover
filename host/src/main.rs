use host::prove_mod_exp_within_range;
use num_bigint::{BigUint, ToBigUint};
use risc0_zkvm::serde::from_slice;
// use std::io;

fn main() {
    // // Define variables to store user input
    // let mut base = String::new();
    // let mut modulus = String::new();
    // let mut range = String::new();
    // let mut exp = String::new();

    // // Prompt the user for input
    // println!("Enter the base (u64):");
    // io::stdin()
    //     .read_line(&mut base)
    //     .expect("Failed to read base");

    // println!("Enter the modulus (u64):");
    // io::stdin()
    //     .read_line(&mut modulus)
    //     .expect("Failed to read modulus");

    // println!("Enter the range (u64):");
    // io::stdin()
    //     .read_line(&mut range)
    //     .expect("Failed to read range");

    // println!("Enter the exponent (u64):");
    // io::stdin()
    //     .read_line(&mut exp)
    //     .expect("Failed to read exponent");

    // // Parse the input into u64 values
    // let base: u64 = base.trim().parse().expect("Invalid base");
    // let modulus: u64 = modulus.trim().parse().expect("Invalid modulus");
    // let range: u64 = range.trim().parse().expect("Invalid range");
    // let exp: u64 = exp.trim().parse().expect("Invalid exponent");

    // println!("Calculating proof...");
    // // Generate and verify a proof
    // let (receipt, proof_time) = prove_mod_exp_within_range(base, modulus, range, exp).unwrap();

    // let (base_res, modulus_res, range_res, result) =
    //     from_slice::<(u64, u64, u64, u64), _>(&receipt.journal)
    //         .unwrap()
    //         .try_into()
    //         .unwrap();

    // println!("Proof complete!");
    // println!("base: {:?}", base_res);
    // println!("modulus: {:?}", modulus_res);
    // println!("range: {:?}", range_res);
    // println!("result: {:?}", result);
    // println!("Elapsed time: {:?}", proof_time);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_exp() {
        let base: BigUint = BigUint::parse_bytes(b"4", 10).unwrap();
        let modulus: BigUint = BigUint::parse_bytes(b"5abf93ea1bb517b8885481e500f46bf338ab3ae1a02ebe459527c8247df5f5d12d40bad8dd834820c5bf7ee23efdaf8f6e04968ea323d656589212bb8c885364b48ba8e5cb05187127fb1963defdfb3dbeefaa0cad61ddd1861eadaa036edcab59d1575a0a8b1ff91b125e4a830051aab98ea28e5cde5809f06d751a06d05710da0b68aa568729d7272bf92c971f8c09417ba9cfc4ba962e041f15a1c9ff081d2ea5c315e4abf7dbc3a83a0a944858bb675472823ae67e7676cb14a3d711e2543d983e314ef76254a740ea7190b0ef26cdbd146d5b79a180995e44c40ccd935468d02dad475ba65de29fc64bc35b6cfaa491c75e10cc12bc39fabcdd2d03d13f39e73f37b82c2d4f808ef2098fd921fcb69eeff5593b92a51309439ab307e1f76068f1a862dca0399b7e633f2f75e39f97e7732849efc69f78a04e93555adf8b8c5ea278b713e9c322826433e4154b9fc243d09f4d71bdc287afd9b0b5a861d15b66968821532d5dbdb5e5e7a8e152a2f2a1a61eb1b483e25bf46b2af415b1e1a73c14942c189422147feaabc4c56795e52286ba070349cac7bcfe588371cff1907fdbbb4c2bb86eeedc250af277b82ef6c88ad0bf2c138e83db903bf832ccc8f9fdf95e2445342d8a9fefd591478d011158cd2a99fda4abbad19e7c840b587366cdbe1c4430b4228fe148da96fe3b606859e54f4d9f8925e077348fe3b7c8d9", 16).unwrap();
        let range: BigUint = modulus.clone() / BigUint::parse_bytes(b"5000000", 10).unwrap();
        let exp: BigUint = BigUint::parse_bytes(b"b7c8d9", 16).unwrap();

        let (receipt, proof_time) =
            prove_mod_exp_within_range(base.clone(), modulus.clone(), range.clone(), exp.clone())
                .unwrap();

        let (base_res, modulus_res, range_res, result) =
            from_slice::<(BigUint, BigUint, BigUint, BigUint), _>(&receipt.journal)
                .unwrap()
                .try_into()
                .unwrap();

        println!("time: {:?}", proof_time);
        assert_eq!(base, base_res);
        assert_eq!(modulus, modulus_res);
        assert_eq!(range, range_res);
        assert_eq!(result, 445u64.to_biguint().unwrap());
    }

    // #[test]
    // #[should_panic(expected = "Exp is not in range")]
    // fn test_invalid_exp() {
    //     let base: u64 = 4;
    //     let modulus: u64 = 497;
    //     let range: u64 = 12;
    //     let exp: u64 = 13;

    //     prove_mod_exp_within_range(base, modulus, range, exp).unwrap();
    // }
}
