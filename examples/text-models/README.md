# text-models

The end-to-end text-component example (SPEC.md §17.12, §17.22, `LM-11`): a
longest-match and a byte-level tokenizer over one content-addressed vocabulary,
two context windows, a greedy and a seeded sampling generator, and a classifier
of each tokenizer, written from language-1.2 source over the package `std`, with
nine production roots certified for `rust-std`.

Its kernel-evaluated facts are stated on bytes and tokens, because the kernel
does not reduce Lean's string primitives; the longest-match scheme and every
root's behavior on text are covered by the generated bridges and the oracle
differential of the roots (`LM-10`), not by a kernel fact.

`expected/` is written by `cargo xtask verify-examples --write`.
