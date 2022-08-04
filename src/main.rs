use std::env;
use rayon::prelude::*;
use solana_sdk::signature::{Keypair};
use solana_sdk::signer::Signer;

fn get_vanity_input() -> String {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    println!("Usage: solana-grinder <vanity_prefix>");
    std::process::exit(1);
  }
  let prefix = String::from(args[1].clone());
  return prefix;
}

fn generate_loop() {
  println!("Thread started");
  let prefix = get_vanity_input();
  for _ in 0..1_000_000_000 {
    let wallet = Keypair::new();
    let public_key = Signer::pubkey(&wallet).to_string();
    if public_key.starts_with(&prefix) {
      println!("Found! Your new public key: {}", public_key);
      println!("{:?}", wallet);
      std::process::exit(1);
    }
  }
}

fn main() {
  let prefix = get_vanity_input();
  let cpu_count = num_cpus::get();
  let thread_count = cpu_count * 2;
  let thread_array = vec![0; thread_count];

  println!("Searching for vanity address with prefix: {}", prefix);
  println!("Using {} cores and {} threads", cpu_count, thread_count);

  thread_array.par_iter().for_each(|_| {
    generate_loop();
  });
}