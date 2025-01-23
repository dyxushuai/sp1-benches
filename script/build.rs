use sp1_build::build_program_with_args;

fn main() {
    build_program_with_args("../program-keccak-verify", Default::default());
    build_program_with_args("../program-native-keccak-prove", Default::default());
    build_program_with_args("../program-keccak-prove", Default::default());
}
