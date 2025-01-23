//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use alloy_rpc_types::Account;
use sp1_sdk::{include_elf, HashableKey, ProverClient, SP1Proof, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const KECCAK_PROVE_ELF: &[u8] = include_elf!("keccak-prove-program");
pub const KECCAK_VERIFY_ELF: &[u8] = include_elf!("keccak-verify-program");
pub const NATIVE_KECCAK_PROVE_ELF: &[u8] = include_elf!("native-keccak-prove-program");

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Setup the prover client.
    let client = ProverClient::from_env();

    let acc = Account::default();

    tracing::info_span!("execute keccak prove").in_scope(|| {
        let mut stdin = SP1Stdin::new();
        stdin.write(&acc);
        let (_, execution_report) = client
            .execute(KECCAK_PROVE_ELF, &stdin)
            .run()
            .expect("proving failed");

        println!(
            "Executed KECCAK_PROVE_ELF with {} cycles",
            execution_report.total_instruction_count() + execution_report.total_syscall_count()
        );
    });

    tracing::info_span!("execute native keccak prove").in_scope(|| {
        let mut stdin = SP1Stdin::new();
        stdin.write(&acc);
        let (_, execution_report) = client
            .execute(NATIVE_KECCAK_PROVE_ELF, &stdin)
            .run()
            .expect("proving failed");

        println!(
            "Executed NATIVE_KECCAK_PROVE_ELF with {} cycles",
            execution_report.total_instruction_count() + execution_report.total_syscall_count()
        );
    });

    let (pk1, vk1) = client.setup(KECCAK_PROVE_ELF);
    // Generate the fibonacci proofs.
    let proof_1 = tracing::info_span!("generate fibonacci proof n=10").in_scope(|| {
        let mut stdin = SP1Stdin::new();
        stdin.write(&acc);
        client
            .prove(&pk1, &stdin)
            .compressed()
            .run()
            .expect("proving failed")
    });
    // let _proof_2 = tracing::info_span!("generate fibonacci proof n=20").in_scope(|| {
    //     let (pk, _) = client.setup(NATIVE_KECCAK_PROVE_ELF);
    //     let mut stdin = SP1Stdin::new();
    //     stdin.write(&acc);
    //     client
    //         .prove(&pk, &stdin)
    //         .compressed()
    //         .run()
    //         .expect("proving failed")
    // });

    let verify = |times: usize| {
        let vk1 = vk1.clone();
        let proof_1 = proof_1.clone();
        tracing::info_span!("execute keccak verify {times} times").in_scope(|| {
            let mut stdin = SP1Stdin::new();
            let vks = (0..times).map(|_| vk1.hash_u32()).collect::<Vec<_>>();
            let public_values: Vec<[u8; 32]> = (0..times)
                .map(|_| proof_1.public_values.hash().try_into().unwrap())
                .collect();
            stdin.write(&vks);
            stdin.write(&public_values);
            let SP1Proof::Compressed(proof) = proof_1.proof else {
                panic!()
            };
            for _ in 0..times {
                stdin.write_proof(*proof.clone(), vk1.vk.clone());
            }
            let (_, execution_report) = client
                .execute(KECCAK_VERIFY_ELF, &stdin)
                .run()
                .expect("proving failed");

            println!(
                "Executed KECCAK_VERIFY_ELF {times} times with {} cycles",
                execution_report.total_instruction_count() + execution_report.total_syscall_count()
            );
        });
    };
    verify(1);
    verify(10);
}
