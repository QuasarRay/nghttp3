#[cfg(test)]
mod tests {
    use super::*;

    struct Noop;
    impl Callbacks for Noop {}

    #[test]
    fn header_flags_never_expose_no_copy_bits() {
        let header = Header::new(b":method".to_vec(), b"GET".to_vec())
            .with_flags(HeaderFlags::NEVER_INDEX.union(HeaderFlags::TRY_INDEX));
        let raw = header.as_raw();
        assert_eq!(raw.flags & sys::NGHTTP3_NV_FLAG_NO_COPY_NAME as u8, 0);
        assert_eq!(raw.flags & sys::NGHTTP3_NV_FLAG_NO_COPY_VALUE as u8, 0);
    }

    #[test]
    fn validates_common_headers() {
        assert!(is_valid_header_name(b"content-type"));
        assert!(!is_valid_header_name(b"Content-Type"));
        assert!(is_valid_header_value(b"application/json"));
    }

    #[test]
    fn creates_and_drops_client_connection() {
        let conn = Connection::new(Role::Client, &Settings::default(), Noop).unwrap();
        assert_eq!(conn.role(), Role::Client);
    }

    #[test]
    fn rejects_assertion_only_stream_preconditions() {
        let mut conn = Connection::new(Role::Client, &Settings::default(), Noop).unwrap();
        let err = conn.bind_control_stream(0).unwrap_err();
        assert_eq!(err.code(), sys::NGHTTP3_ERR_INVALID_ARGUMENT);
        let err = conn.read_stream(-1, &[], false, 0).unwrap_err();
        assert_eq!(err.code(), sys::NGHTTP3_ERR_INVALID_ARGUMENT);
    }

    #[test]
    fn creates_standalone_qpack_objects() {
        let mut encoder = QpackEncoder::new(4096, 1).unwrap();
        encoder.set_max_table_capacity(4096);
        let decoder = QpackDecoder::new(4096, 16).unwrap();
        let context = QpackStreamContext::new(0).unwrap();
        assert_eq!(encoder.blocked_streams(), 0);
        assert_eq!(decoder.insert_count(), 0);
        assert_eq!(context.required_insert_count(), 0);
    }

    #[test]
    fn uvarint_round_trips_boundaries() {
        for value in [0, 63, 64, 16_383, 16_384, 1_073_741_823, MAX_VARINT] {
            let encoded = encode_uvarint(value).unwrap();
            let (decoded, consumed) = decode_uvarint(&encoded).unwrap();
            assert_eq!(decoded, value);
            assert_eq!(consumed, encoded.len());
        }
    }

    #[test]
    fn runtime_version_is_available() {
        assert_ne!(version(), "unknown");
    }
}
