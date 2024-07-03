```sh
RUST_LOG=info RUST_BACKTRACE=full cargo run --release
```

Attempt to extract a storage proving circuit from [`axiom-eth`](https://github.com/axiom-crypto/axiom-eth)..

- Currently stuck at `let snark_header = gen_snark_shplonk(...)` (main.rs line 442) failing with "SNARK proof failed to verify"
- Suspect something is wrong with our MMR? (`fn mmr_1` in utils)
  - for this initial hacky version we are just trying to construct a MMR containing only one actual blockhash
  - we are computing a merke root of the single-blockhash array padded to 1024 items, then adding that as single leaf to the MMR
- What is `InputSubqueryAggregation.promise_commit_keccak` supposed to be?