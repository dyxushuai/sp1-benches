#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    let vkeys = sp1_zkvm::io::read::<Vec<[u32; 8]>>();
    let public_values = sp1_zkvm::io::read::<Vec<[u8; 32]>>();
    for i in 0..vkeys.len() {
        sp1_zkvm::lib::verify::verify_sp1_proof(&vkeys[i], &public_values[i]);
    }
}
