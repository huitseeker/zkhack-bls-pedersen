use bls_pedersen::bls::verify;
use bls_pedersen::data::puzzle_data;
use bls_pedersen::PUZZLE_DESCRIPTION;
use prompt::{puzzle, welcome};

use ark_bls12_381::G1Affine;
use std::io::{Cursor};
use ark_serialize::CanonicalDeserialize;


fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (pk, ms, sigs) = puzzle_data();
    for (m, sig) in ms.iter().zip(sigs.iter()) {
        verify(pk, m, *sig);
    }

    let sig = G1Affine::deserialize(&mut Cursor::new(hex::decode("d5499efe8cdaeb4f43f917671504de242f23444a8a391de130afac4d7bff6da14067fe172f9b532189ad608e79acf498").unwrap()));
    let m = b"huitseeker";
    verify(pk, m, sig.unwrap());
}
