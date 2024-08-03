# FHEathon

Implementing different FHE schemes from e2e. Naming inspired by 0xPARC's plonkathon.

## TFHE

I have been following the wonderful guide at https://www.daniellowengrub.com/blog/2024/01/03/fully-homomorphic-encryption, that builds TFHE from scratch in Python. I am using Rust to improve my abilities and to (hopefully) be significantly faster than the Python implementation (which is 20ms on a single core of an Intel i5 processor). I am also hoping to add NTT-based negacyclic polynomial multiplication and parallelization with rayon to get further speedups.

This tutorial builds a NAND gate, which I'm hoping to build a full adder and a uint8 comparator with. I may write up a guide on how to do that.

### Running tests

- The initial LWE encrypt/decrypt tests: `cargo test --package fheathon --lib -- tfhe::lwe::tests::test_lwe --exact --show-output`

## BFV

I will be implementing BFV e2e next. There is no wonderful guide to follow (AFAIK) so I will be doing a mix of following implementations by Janmajaya (https://github.com/Janmajayamall/bfv) and Lattigo (https://github.com/tuneinsight/lattigo).

I am also hoping to implement multi-party BFV to understand how multi-party FHE is built and how/why performance differs from standard FHE.
