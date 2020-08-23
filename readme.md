# λ-Calculus 

This is an implementation of the untyped λ-calculus[1] in Rust. The λ-calculus 
is a model for computation that is based on mathematical logic.
This implementation represents abstractions, 
applications, and variables in an enum. Rust's enums allow associated types[2],
meaning that applications form a binary tree-like structure.

This program uses lalrpop to parse lambda terms. It is based off of the
implementation used in "Types and Programming Languages" [3], but uses symbolic
on-the-fly renaming rather than De Brujin indices.

[1] `wikipedia.org/wiki/Lambda_calculus`

[2] `doc.rust-lang.org/stable/book/ch06-01-defining-an-enum.html`

[3] `cis.upenn.edu/~bcpierce/tapl/`
