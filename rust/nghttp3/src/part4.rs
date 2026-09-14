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
    fn runtime_version_is_available() {
        assert_ne!(version(), "unknown");
    }
}
