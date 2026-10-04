# Priority parser verification provenance

The safe parser in `nghttp3-core::priority` is intentionally narrower than a
general-purpose Structured Fields implementation.

- RFC 9218 defines the Priority field and the standardized `u` and `i`
  dictionary members.
- RFC 8941 supplies the Structured Fields dictionary/item syntax referenced by
  RFC 9218.
- The preserved C implementation plus sfparse is an executable compatibility
  oracle for supported forms.
- Historical commit
  `aed3107f9104eae77d97ed8093caa8f3b7ef64d4` is the regression authority for
  the trailing-`=` bounds bug.

The Rust parser uses checked slice cursors and never performs pointer arithmetic.
Unknown dictionary members and parameters are consumed but ignored. The module
does not claim to replace a complete reusable Structured Fields library; that
would require broader RFC 8941/9651 coverage and its own differential corpus.
