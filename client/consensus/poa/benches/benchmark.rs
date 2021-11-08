use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

use cc_consensus_poa::{ChunkProof, ChunkProofBuilder};
use cp_permastore::CHUNK_SIZE;

fn generate_chunk_proof(data: Vec<u8>, offset: u32) -> ChunkProof {
    ChunkProofBuilder::new(data, CHUNK_SIZE, offset)
        .build()
        .expect("failed to build chunk proof")
}

fn random