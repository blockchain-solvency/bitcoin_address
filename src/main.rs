use bitcoin::secp256k1::{rand, Secp256k1};
use bitcoin::{Address, Network, PublicKey};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::{Error, Write};
//TODO: Make it go faster, i shouldn't take 10 minutes
fn main() -> Result<(), Error> {
    let path = "addresses.txt";
    let pb = ProgressBar::new(1000000);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})").unwrap()
        .progress_chars("#>-"));

    let mut output = File::create(path)?;
    for x in 1..1000000 {
        // Generate random key pair.
        let s = Secp256k1::new();

        let public_key = PublicKey::new(s.generate_keypair(&mut rand::thread_rng()).1);

        // Generate pay-to-pubkey-hash address.
        let address = Address::p2pkh(&public_key, Network::Bitcoin).to_string();
        writeln!(output, "{:?}", address)?;
        pb.set_position(x);
    }
    pb.finish_with_message("Addresses generated");
    Ok(())
}
