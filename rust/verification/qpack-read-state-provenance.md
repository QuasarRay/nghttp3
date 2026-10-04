# QPACK decoder read-state ownership provenance

This slice converts an implementation-lifetime invariant into Rust ownership.

## Historical source

nghttp3 commit
`ecfae7acf1813f83843b78b607a6abd1fd47be86` fixed a null dereference on
allocation failure. The C decoder decremented temporary reference-counted
`rstate.name` / `rstate.value` owners after attempting dynamic-table
insertion, but did not always clear the corresponding raw pointer slots.

The dynamic-table insertion path itself retains independent references via
`nghttp3_qpack_entry_init`. The decoder read state therefore owns only its
temporary references.

## Rust invariant

`DecoderReadState` stores those temporaries as `Option<Vec<u8>>`.

- `take_value` moves the value owner out and leaves `None`.
- `take_literal` first checks that both owners exist, then moves both out.
- if downstream insertion fails, dropping the extracted value/literal releases
  it while the read state remains empty;
- `reset` can only drop owners still actually stored in the state.

This is stronger than reproducing the C repair with explicit null assignments:
safe Rust cannot retain the same moved owner in the state.

## Authority boundary

RFC 9204 remains authoritative for QPACK protocol behavior. Reference counting,
raw-pointer nulling, and this ownership representation are implementation
details; the historical commit is the regression oracle for this safety
property.
