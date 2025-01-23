#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_rpc_types::Account;

pub fn main() {
    let acc = sp1_zkvm::io::read::<Account>();
    let bytes = acc.trie_hash_slow();
    sp1_zkvm::io::commit(&bytes.0);
}
