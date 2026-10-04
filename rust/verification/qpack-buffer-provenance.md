# QPACK buffer-growth verification provenance

This slice addresses implementation safety, not an RFC-mandated allocation
strategy.

- RFC 9204 remains the protocol authority for QPACK behavior and wire format.
- Historical nghttp3 commit
  `8a8d45cb734eb8087773988cafee3a15906e7f52` is the authority for the
  implementation-specific 2^31 pre-rounding ceiling.
- The old C code rounded a size to a 32-bit power of two. Accepting values above
  2^31 could require 2^32 and overflow the shift.
- The Rust replacement performs checked size addition and
  `checked_next_power_of_two`, while preserving the 2^31 compatibility ceiling.

The 2^31 limit must not be presented as a QPACK protocol requirement.
