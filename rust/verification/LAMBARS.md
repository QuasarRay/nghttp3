# Lambars metaprogramming policy

Lambars is pinned to commit
`6b79ace7ebd04910c953497c85baa42ff9cee0dc`.

## Why it is a separate contract layer

The verified protocol implementation in `nghttp3-core` remains ordinary,
unsafe-free Rust with minimal dependencies.  Repeated proof/test declarations
belong in `nghttp3-contracts`, where Lambars can aggressively compress them
without making the production state machine harder for Verus or Kani to model.

The first conversion uses:

- `pipe!` to express contract evaluation as a compact data-flow pipeline;
- `#[verification_case]` to attach canonical source/provenance IDs;
- `#[derive(VerificationModel)]` to register reusable verification models;
- `boundary_cases!` to generate named runtime regressions and Kani companions
  from one semantic predicate;
- `dual_verify!` to generate an ordinary regression and Kani proof from one
  predicate body.

This pattern is intended to replace repeated hand-written verification shapes as
the Rust reimplementation grows.

## Runtime rule

Do **not** enable Lambars' `async` feature in the nghttp3/ioxide path.  That
feature currently brings Tokio.  The contract crate enables only synchronous
composition features, and CI fails if Tokio appears in its dependency tree.

When transport integration begins, runtime scheduling belongs behind an
ioxide/GenHTTP-compatible interface.  Protocol parsing, QPACK, stream state, and
verification remain runtime-independent.

## Proof rule

Metaprogramming may generate proof obligations, but it may not hide or weaken
them.  Every generated obligation keeps a canonical RFC/IANA/history identifier,
and exhaustive Kani/Verus proofs in lower-level crates remain authoritative when
a generated concrete boundary test overlaps them.
