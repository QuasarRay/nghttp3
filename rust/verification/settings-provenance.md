# Settings verification provenance

This file separates protocol authority from nghttp3 implementation policy.

| Rust field / invariant | Authority | Classification |
| --- | --- | --- |
| max_field_section_size wire domain | RFC 9114 + IANA HTTP/3 settings registry + RFC 9000 varint domain | protocol |
| qpack_max_table_capacity | RFC 9204 + IANA HTTP/3 settings registry | protocol |
| qpack_blocked_streams | RFC 9204 + IANA HTTP/3 settings registry | protocol |
| enable_connect_protocol | RFC 9220 + IANA HTTP/3 settings registry | protocol |
| h3_datagram | RFC 9297 + IANA HTTP/3 settings registry | protocol |
| origin_list owned payload semantics | RFC 9412 | protocol extension |
| qpack_encoder_max_table_capacity = 4096 default | original nghttp3 settings implementation | implementation policy |
| glitch rate defaults 1000 / 33 | original nghttp3 settings implementation | implementation policy |
| V3 -> V4 qpack_indexing_strategy = NONE | original nghttp3 version-conversion semantics/tests | implementation compatibility |

The C source and tests remain an executable compatibility oracle. They do not
override an RFC or IANA registry when protocol behavior differs.
