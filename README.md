# FHEathon

Implementing different FHE schemes from e2e

## TFHE

I am following the wonderful guide at https://www.daniellowengrub.com/blog/2024/01/03/fully-homomorphic-encryption, that builds TFHE from scratch in Python. To improve my Rust and to (hopefully) be significantly faster than the Python implementation (which is 20ms on a single core of an Intel i5 processor). I am also hoping to add NTT-based negacyclic polynomial multiplication and parallelization with rayon to get further speedups.
