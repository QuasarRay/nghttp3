# Verification foundations

The Rust reimplementation uses several independent oracles. They are deliberately
not conflated.

## Authority order

1. IETF Standards Track RFCs and IANA registries are the protocol authority.
2. Normalized machine-readable properties retain provenance to the exact RFC
   section or registry entry.
3. The original C implementation and its tests/history are executable
   compatibility and regression oracles.
4. The C2Rust output is a translation baseline, not a specification.

## Kani

Kani is pinned in foundations.toml. It model-checks the real Rust
implementation for panics, arithmetic or UB hazards, and explicit properties.

Every bug discovered in the C implementation, generated Rust, differential
testing, fuzzing, or formal proof should become a permanent Kani harness when
the state space is suitable. Counterexamples should additionally be materialized
as ordinary regression tests so future builds retain a cheap concrete guard.

CI runs Kani through `run-kani.sh`. If verification fails, the script reruns
Kani with experimental concrete playback in print mode and preserves the
generated counterexample test source as an artifact. A reviewed counterexample
is then committed as an ordinary unit regression beside the corresponding proof
harness; symbolic verification remains the stronger continuing guard.

## Verus

Verus is pinned to a weekly point release and its corresponding Rust toolchain.
It proves functional properties that are awkward or unbounded for model
checking. Proof models must cite the normative rule they encode and are migrated
toward direct specifications over refactored Rust as components become
Verus-friendly.

## VerusBelt

VerusBelt is not an application verifier invoked on arbitrary nghttp3 Rust.
It is the Rocq semantic development validating Verus proof-oriented extensions
to the Rust type system. We pin its exact commit and tool versions as a
foundation. Where nghttp3 proofs later rely on Verus proof-oriented types
covered by VerusBelt, the mapping and assumptions must be recorded here.

Pure integer proofs such as QUIC varint classification do not gain an additional
claim merely by rebuilding VerusBelt; Kani and Verus prove those program
properties. A foundation job prevents silent VerusBelt drift, and future
proof-oriented type usage will add concrete Rocq mapping obligations.

## Initial coverage

| Component | Differential C | Kani | Verus | Normative source |
| --- | --- | --- | --- | --- |
| QUIC varint | yes | roundtrip, width, truncation | width/domain | RFC 9000 section 16 |
| HTTP/3 frames | planned | planned | planned | RFC 9114 |
| QPACK | existing FFI oracle | planned | planned | RFC 9204 |
| HTTP semantics | existing C tests | planned | planned | RFC 9110/9114 |
