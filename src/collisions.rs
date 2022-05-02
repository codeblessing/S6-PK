//! Utilities for testing collisions on first n bits in generated hashes.
//!
//! # Requirements
//!
//! Hash-generating functions must implement HashGenerator trait defined in this create.
//!

use std::collections::{HashMap, HashSet};

use crate::traits::HashGenerator;

fn generate_hash_set(
    generator: &mut dyn HashGenerator,
    dataset: &[impl AsRef<str>],
) -> Vec<String> {
    dataset
        .iter()
        .map(|msg| generator.generate(msg.as_ref().as_bytes()))
        .collect()
}

pub fn find_collision_on(
    bit_length: usize,
    generator: &mut dyn HashGenerator,
    dataset: &[impl AsRef<str>],
) -> HashMap<String, usize> {
    let mut hashes = generate_hash_set(generator, dataset);

    let mut shorts: Vec<String> = hashes
        .drain(..)
        .map(|hash| hash.chars().take(bit_length).collect())
        .collect();

    let hashes: HashSet<String> = shorts.iter().cloned().collect();

    let mut collisions: HashMap<String, usize> = HashMap::new();

    for hash in hashes {
        let count = shorts.iter().filter(|&other| other == &hash).count() - 1;
        shorts = shorts.drain(..).filter(|short| short != &hash).collect();
        collisions.insert(hash, count);
    }

    collisions
}
