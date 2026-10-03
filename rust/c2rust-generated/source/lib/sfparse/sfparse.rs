extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn abort() -> !;
}
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type size_t = usize;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct sfparse_type(pub ::core::ffi::c_uint);
impl sfparse_type {
    pub const SFPARSE_TYPE_BOOLEAN: Self = Self(0);
    pub const SFPARSE_TYPE_INTEGER: Self = Self(1);
    pub const SFPARSE_TYPE_DECIMAL: Self = Self(2);
    pub const SFPARSE_TYPE_STRING: Self = Self(3);
    pub const SFPARSE_TYPE_TOKEN: Self = Self(4);
    pub const SFPARSE_TYPE_BYTESEQ: Self = Self(5);
    pub const SFPARSE_TYPE_INNER_LIST: Self = Self(6);
    pub const SFPARSE_TYPE_DATE: Self = Self(7);
    pub const SFPARSE_TYPE_DISPSTRING: Self = Self(8);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfparse_vec {
    pub base: *mut uint8_t,
    pub len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfparse_decimal {
    pub numer: int64_t,
    pub denom: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfparse_value {
    pub r#type: sfparse_type,
    pub flags: uint32_t,
    pub c2rust_unnamed: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub boolean: ::core::ffi::c_int,
    pub integer: int64_t,
    pub decimal: sfparse_decimal,
    pub vec: sfparse_vec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfparse_parser {
    pub pos: *const uint8_t,
    pub end: *const uint8_t,
    pub state: uint32_t,
}
pub const SFPARSE_ERR_PARSE: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const SFPARSE_ERR_EOF: ::core::ffi::c_int = -2 as ::core::ffi::c_int;
pub const SFPARSE_VALUE_FLAG_NONE: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const SFPARSE_VALUE_FLAG_ESCAPED_STRING: ::core::ffi::c_uint = 0x1
    as ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const SFPARSE_STATE_DICT: ::core::ffi::c_uint = 0x8 as ::core::ffi::c_uint;
pub const SFPARSE_STATE_LIST: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const SFPARSE_STATE_ITEM: ::core::ffi::c_uint = 0x18 as ::core::ffi::c_uint;
pub const SFPARSE_STATE_INNER_LIST: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const SFPARSE_STATE_BEFORE: ::core::ffi::c_uint = 0;
pub const SFPARSE_STATE_BEFORE_PARAMS: ::core::ffi::c_uint = 1;
pub const SFPARSE_STATE_PARAMS: ::core::ffi::c_uint = 2;
pub const SFPARSE_STATE_AFTER: ::core::ffi::c_uint = 0x3 as ::core::ffi::c_uint;
pub const SFPARSE_STATE_OP_MASK: ::core::ffi::c_uint = 0x3 as ::core::ffi::c_uint;
pub const SFPARSE_STATE_DICT_AFTER: ::core::ffi::c_uint = 11;
pub const SFPARSE_STATE_DICT_BEFORE_PARAMS: ::core::ffi::c_uint = SFPARSE_STATE_DICT
    | SFPARSE_STATE_BEFORE_PARAMS;
pub const SFPARSE_STATE_DICT_INNER_LIST_BEFORE: ::core::ffi::c_uint = SFPARSE_STATE_DICT
    | SFPARSE_STATE_INNER_LIST | SFPARSE_STATE_BEFORE;
pub const SFPARSE_STATE_LIST_AFTER: ::core::ffi::c_uint = 19;
pub const SFPARSE_STATE_LIST_BEFORE_PARAMS: ::core::ffi::c_uint = SFPARSE_STATE_LIST
    | SFPARSE_STATE_BEFORE_PARAMS;
pub const SFPARSE_STATE_LIST_INNER_LIST_BEFORE: ::core::ffi::c_uint = SFPARSE_STATE_LIST
    | SFPARSE_STATE_INNER_LIST | SFPARSE_STATE_BEFORE;
pub const SFPARSE_STATE_ITEM_AFTER: ::core::ffi::c_uint = 27;
pub const SFPARSE_STATE_ITEM_BEFORE_PARAMS: ::core::ffi::c_uint = SFPARSE_STATE_ITEM
    | SFPARSE_STATE_BEFORE_PARAMS;
pub const SFPARSE_STATE_ITEM_INNER_LIST_BEFORE: ::core::ffi::c_uint = SFPARSE_STATE_ITEM
    | SFPARSE_STATE_INNER_LIST | SFPARSE_STATE_BEFORE;
pub const SFPARSE_STATE_INITIAL: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
unsafe extern "C" fn is_ws(mut c: uint8_t) -> ::core::ffi::c_int {
    match c as ::core::ffi::c_int {
        32 | 9 => return 1 as ::core::ffi::c_int,
        _ => return 0 as ::core::ffi::c_int,
    };
}
unsafe extern "C" fn parser_eof(mut sfp: *mut sfparse_parser) -> ::core::ffi::c_int {
    return ((*sfp).pos == (*sfp).end) as ::core::ffi::c_int;
}
unsafe extern "C" fn parser_discard_ows(mut sfp: *mut sfparse_parser) {
    while parser_eof(sfp) == 0 && is_ws(*(*sfp).pos) != 0 {
        (*sfp).pos = (*sfp).pos.offset(1);
    }
}
unsafe extern "C" fn parser_discard_sp(mut sfp: *mut sfparse_parser) {
    while parser_eof(sfp) == 0
        && *(*sfp).pos as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
    {
        (*sfp).pos = (*sfp).pos.offset(1);
    }
}
unsafe extern "C" fn parser_set_op_state(
    mut sfp: *mut sfparse_parser,
    mut op: uint32_t,
) {
    (*sfp).state = ((*sfp).state as ::core::ffi::c_uint & !SFPARSE_STATE_OP_MASK)
        as uint32_t;
    (*sfp).state |= op;
}
unsafe extern "C" fn parser_unset_inner_list_state(mut sfp: *mut sfparse_parser) {
    (*sfp).state = ((*sfp).state as ::core::ffi::c_uint & !SFPARSE_STATE_INNER_LIST)
        as uint32_t;
}
static mut key_tbl: [uint8_t; 256] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1 as uint8_t,
    0,
    0,
    2 as uint8_t,
    2 as uint8_t,
    0,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    2 as uint8_t,
    0,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
unsafe extern "C" fn parser_key(
    mut sfp: *mut sfparse_parser,
    mut dest: *mut sfparse_vec,
) -> ::core::ffi::c_int {
    let mut base: *const uint8_t = ::core::ptr::null::<uint8_t>();
    if key_tbl[*(*sfp).pos as usize] as ::core::ffi::c_int != 1 as ::core::ffi::c_int {
        return SFPARSE_ERR_PARSE;
    }
    let c2rust_fresh1 = (*sfp).pos;
    (*sfp).pos = (*sfp).pos.offset(1);
    base = c2rust_fresh1;
    while parser_eof(sfp) == 0
        && key_tbl[*(*sfp).pos as usize] as ::core::ffi::c_int != 0
    {
        (*sfp).pos = (*sfp).pos.offset(1);
    }
    if !dest.is_null() {
        (*dest).base = base as *mut uint8_t;
        (*dest).len = (*sfp).pos.offset_from((*dest).base) as size_t;
    }
    return 0 as ::core::ffi::c_int;
}
static mut number_tbl: [uint8_t; 256] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
unsafe extern "C" fn parser_number(
    mut sfp: *mut sfparse_parser,
    mut dest: *mut sfparse_value,
) -> ::core::ffi::c_int {
    let mut sign: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut value: int64_t = 0 as int64_t;
    let mut len: size_t = 0 as size_t;
    let mut fpos: size_t = 0 as size_t;
    if *(*sfp).pos as ::core::ffi::c_int == '-' as ::core::ffi::c_int {
        (*sfp).pos = (*sfp).pos.offset(1);
        if parser_eof(sfp) != 0 {
            return SFPARSE_ERR_PARSE;
        }
        sign = -1 as ::core::ffi::c_int;
    }
    '_c2rust_label: {
        if parser_eof(sfp) == 0 {} else {
            __assert_fail(
                b"!parser_eof(sfp)\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/sfparse/sfparse.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                245 as ::core::ffi::c_uint,
                b"int parser_number(sfparse_parser *, sfparse_value *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    while parser_eof(sfp) == 0
        && number_tbl[*(*sfp).pos as usize] as ::core::ffi::c_int != 0
    {
        len = len.wrapping_add(1);
        if len > 15 as size_t {
            return SFPARSE_ERR_PARSE;
        }
        value *= 10 as int64_t;
        value
            += (*(*sfp).pos as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                as int64_t;
        (*sfp).pos = (*sfp).pos.offset(1);
    }
    if len == 0 as size_t {
        return SFPARSE_ERR_PARSE;
    }
    if parser_eof(sfp) != 0
        || *(*sfp).pos as ::core::ffi::c_int != '.' as ::core::ffi::c_int
    {
        if !dest.is_null() {
            (*dest).r#type = sfparse_type::SFPARSE_TYPE_INTEGER;
            (*dest).flags = SFPARSE_VALUE_FLAG_NONE as uint32_t;
            (*dest).c2rust_unnamed.integer = value * sign as int64_t;
        }
        return 0 as ::core::ffi::c_int;
    }
    if len > 12 as size_t {
        return SFPARSE_ERR_PARSE;
    }
    fpos = len;
    (*sfp).pos = (*sfp).pos.offset(1);
    while parser_eof(sfp) == 0
        && number_tbl[*(*sfp).pos as usize] as ::core::ffi::c_int != 0
    {
        len = len.wrapping_add(1);
        if len > 15 as size_t {
            return SFPARSE_ERR_PARSE;
        }
        value *= 10 as int64_t;
        value
            += (*(*sfp).pos as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                as int64_t;
        (*sfp).pos = (*sfp).pos.offset(1);
    }
    if fpos == len || len.wrapping_sub(fpos) > 3 as size_t {
        return SFPARSE_ERR_PARSE;
    }
    if !dest.is_null() {
        (*dest).r#type = sfparse_type::SFPARSE_TYPE_DECIMAL;
        (*dest).flags = SFPARSE_VALUE_FLAG_NONE as uint32_t;
        (*dest).c2rust_unnamed.decimal.numer = value * sign as int64_t;
        match len.wrapping_sub(fpos) {
            1 => {
                (*dest).c2rust_unnamed.decimal.denom = 10 as int64_t;
            }
            2 => {
                (*dest).c2rust_unnamed.decimal.denom = 100 as int64_t;
            }
            3 => {
                (*dest).c2rust_unnamed.decimal.denom = 1000 as int64_t;
            }
            _ => {}
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn parser_date(
    mut sfp: *mut sfparse_parser,
    mut dest: *mut sfparse_value,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    let mut val: sfparse_value = sfparse_value {
        r#type: sfparse_type::SFPARSE_TYPE_BOOLEAN,
        flags: 0,
        c2rust_unnamed: C2Rust_Unnamed { boolean: 0 },
    };
    '_c2rust_label: {
        if '@' as ::core::ffi::c_int == *(*sfp).pos as ::core::ffi::c_int {} else {
            __assert_fail(
                b"'@' == *sfp->pos\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/sfparse/sfparse.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                322 as ::core::ffi::c_uint,
                b"int parser_date(sfparse_parser *, sfparse_value *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*sfp).pos = (*sfp).pos.offset(1);
    if parser_eof(sfp) != 0 {
        return SFPARSE_ERR_PARSE;
    }
    rv = parser_number(sfp, &raw mut val);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    if val.r#type.0 != sfparse_type::SFPARSE_TYPE_INTEGER.0 {
        return SFPARSE_ERR_PARSE;
    }
    if !dest.is_null() {
        *dest = val;
        (*dest).r#type = sfparse_type::SFPARSE_TYPE_DATE;
    }
    return 0 as ::core::ffi::c_int;
}
static mut string_tbl: [uint8_t; 256] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1 as uint8_t,
    1 as uint8_t,
    3 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    2 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
unsafe extern "C" fn parser_string(
    mut sfp: *mut sfparse_parser,
    mut dest: *mut sfparse_value,
) -> ::core::ffi::c_int {
    let mut base: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut flags: uint32_t = SFPARSE_VALUE_FLAG_NONE as uint32_t;
    '_c2rust_label: {
        if '"' as ::core::ffi::c_int == *(*sfp).pos as ::core::ffi::c_int {} else {
            __assert_fail(
                b"'\"' == *sfp->pos\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/sfparse/sfparse.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                391 as ::core::ffi::c_uint,
                b"int parser_string(sfparse_parser *, sfparse_value *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*sfp).pos = (*sfp).pos.offset(1);
    base = (*sfp).pos;
    '_fin: {
        while parser_eof(sfp) == 0 {
            match string_tbl[*(*sfp).pos as usize] as ::core::ffi::c_int {
                0 => return SFPARSE_ERR_PARSE,
                2 => {
                    (*sfp).pos = (*sfp).pos.offset(1);
                    if parser_eof(sfp) != 0 {
                        return SFPARSE_ERR_PARSE;
                    }
                    match *(*sfp).pos as ::core::ffi::c_int {
                        34 | 92 => {
                            flags = SFPARSE_VALUE_FLAG_ESCAPED_STRING as uint32_t;
                        }
                        _ => return SFPARSE_ERR_PARSE,
                    }
                }
                3 => {
                    break '_fin;
                }
                1 | _ => {}
            }
            (*sfp).pos = (*sfp).pos.offset(1);
        }
        return SFPARSE_ERR_PARSE;
    }
    if !dest.is_null() {
        (*dest).r#type = sfparse_type::SFPARSE_TYPE_STRING;
        (*dest).flags = flags;
        (*dest).c2rust_unnamed.vec.len = (*sfp).pos.offset_from(base) as size_t;
        (*dest).c2rust_unnamed.vec.base = if (*dest).c2rust_unnamed.vec.len
            == 0 as size_t
        {
            ::core::ptr::null_mut::<uint8_t>()
        } else {
            base as *mut uint8_t
        };
    }
    (*sfp).pos = (*sfp).pos.offset(1);
    return 0 as ::core::ffi::c_int;
}
static mut token_tbl: [uint8_t; 256] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1 as uint8_t,
    0,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    0,
    0,
    1 as uint8_t,
    1 as uint8_t,
    0,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    0,
    0,
    0,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    0,
    1 as uint8_t,
    0,
    1 as uint8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
unsafe extern "C" fn parser_token(
    mut sfp: *mut sfparse_parser,
    mut dest: *mut sfparse_value,
) -> ::core::ffi::c_int {
    let mut base: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let c2rust_fresh0 = (*sfp).pos;
    (*sfp).pos = (*sfp).pos.offset(1);
    base = c2rust_fresh0;
    while parser_eof(sfp) == 0
        && token_tbl[*(*sfp).pos as usize] as ::core::ffi::c_int != 0
    {
        (*sfp).pos = (*sfp).pos.offset(1);
    }
    if !dest.is_null() {
        (*dest).r#type = sfparse_type::SFPARSE_TYPE_TOKEN;
        (*dest).flags = SFPARSE_VALUE_FLAG_NONE as uint32_t;
        (*dest).c2rust_unnamed.vec.base = base as *mut uint8_t;
        (*dest).c2rust_unnamed.vec.len = (*sfp).pos.offset_from(base) as size_t;
    }
    return 0 as ::core::ffi::c_int;
}
static mut byteseq_tbl: [uint8_t; 256] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1 as uint8_t,
    0,
    0,
    0,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    3 as uint8_t,
    0,
    0,
    2 as uint8_t,
    0,
    0,
    0,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
unsafe extern "C" fn parser_byteseq(
    mut sfp: *mut sfparse_parser,
    mut dest: *mut sfparse_value,
) -> ::core::ffi::c_int {
    let mut base: *const uint8_t = ::core::ptr::null::<uint8_t>();
    '_c2rust_label: {
        if ':' as ::core::ffi::c_int == *(*sfp).pos as ::core::ffi::c_int {} else {
            __assert_fail(
                b"':' == *sfp->pos\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/sfparse/sfparse.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                619 as ::core::ffi::c_uint,
                b"int parser_byteseq(sfparse_parser *, sfparse_value *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*sfp).pos = (*sfp).pos.offset(1);
    base = (*sfp).pos;
    '_fin: {
        while parser_eof(sfp) == 0 {
            match byteseq_tbl[*(*sfp).pos as usize] as ::core::ffi::c_int {
                0 => return SFPARSE_ERR_PARSE,
                2 => {
                    match (*sfp).pos.offset_from(base) & 0x3isize {
                        0 | 1 => return SFPARSE_ERR_PARSE,
                        2 => {
                            (*sfp).pos = (*sfp).pos.offset(1);
                            if parser_eof(sfp) != 0 {
                                return SFPARSE_ERR_PARSE;
                            }
                            if *(*sfp).pos as ::core::ffi::c_int
                                == '=' as ::core::ffi::c_int
                            {
                                (*sfp).pos = (*sfp).pos.offset(1);
                            }
                        }
                        3 => {
                            (*sfp).pos = (*sfp).pos.offset(1);
                        }
                        _ => {}
                    }
                    if parser_eof(sfp) != 0
                        || *(*sfp).pos as ::core::ffi::c_int != ':' as ::core::ffi::c_int
                    {
                        return SFPARSE_ERR_PARSE;
                    }
                    break '_fin;
                }
                3 => {
                    if (*sfp).pos.offset_from(base) & 0x3isize == 1isize {
                        return SFPARSE_ERR_PARSE;
                    }
                    break '_fin;
                }
                1 | _ => {
                    (*sfp).pos = (*sfp).pos.offset(1);
                }
            }
        }
        return SFPARSE_ERR_PARSE;
    }
    if !dest.is_null() {
        (*dest).r#type = sfparse_type::SFPARSE_TYPE_BYTESEQ;
        (*dest).flags = SFPARSE_VALUE_FLAG_NONE as uint32_t;
        (*dest).c2rust_unnamed.vec.len = (*sfp).pos.offset_from(base) as size_t;
        (*dest).c2rust_unnamed.vec.base = if (*dest).c2rust_unnamed.vec.len
            == 0 as size_t
        {
            ::core::ptr::null_mut::<uint8_t>()
        } else {
            base as *mut uint8_t
        };
    }
    (*sfp).pos = (*sfp).pos.offset(1);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn parser_boolean(
    mut sfp: *mut sfparse_parser,
    mut dest: *mut sfparse_value,
) -> ::core::ffi::c_int {
    let mut b: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if '?' as ::core::ffi::c_int == *(*sfp).pos as ::core::ffi::c_int {} else {
            __assert_fail(
                b"'?' == *sfp->pos\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/sfparse/sfparse.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                692 as ::core::ffi::c_uint,
                b"int parser_boolean(sfparse_parser *, sfparse_value *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*sfp).pos = (*sfp).pos.offset(1);
    if parser_eof(sfp) != 0 {
        return SFPARSE_ERR_PARSE;
    }
    match *(*sfp).pos as ::core::ffi::c_int {
        48 => {
            b = 0 as ::core::ffi::c_int;
        }
        49 => {
            b = 1 as ::core::ffi::c_int;
        }
        _ => return SFPARSE_ERR_PARSE,
    }
    (*sfp).pos = (*sfp).pos.offset(1);
    if !dest.is_null() {
        (*dest).r#type = sfparse_type::SFPARSE_TYPE_BOOLEAN;
        (*dest).flags = SFPARSE_VALUE_FLAG_NONE as uint32_t;
        (*dest).c2rust_unnamed.boolean = b;
    }
    return 0 as ::core::ffi::c_int;
}
static mut pct_tbl: [uint8_t; 256] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
unsafe extern "C" fn pctdecode(
    mut pc: *mut uint8_t,
    mut ppos: *mut *const uint8_t,
) -> ::core::ffi::c_int {
    let mut c: uint8_t = 0;
    let mut b: uint8_t = **ppos;
    match pct_tbl[b as usize] as ::core::ffi::c_int {
        0 => return -1 as ::core::ffi::c_int,
        1 => {
            c = ((b as ::core::ffi::c_int - '0' as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int) as uint8_t;
        }
        2 => {
            c = ((b as ::core::ffi::c_int - 'a' as ::core::ffi::c_int
                + 10 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint8_t;
        }
        _ => {
            '_c2rust_label: {
                __assert_fail(
                    b"0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/tmp.cM4Hy2HYmu/src/lib/sfparse/sfparse.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    744 as ::core::ffi::c_uint,
                    b"int pctdecode(uint8_t *, const uint8_t **)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            abort();
        }
    }
    *ppos = (*ppos).offset(1);
    b = **ppos;
    match pct_tbl[b as usize] as ::core::ffi::c_int {
        0 => return -1 as ::core::ffi::c_int,
        1 => {
            c = (c as ::core::ffi::c_int
                | (b as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as uint8_t
                    as ::core::ffi::c_int) as uint8_t;
        }
        2 => {
            c = (c as ::core::ffi::c_int
                | (b as ::core::ffi::c_int - 'a' as ::core::ffi::c_int
                    + 10 as ::core::ffi::c_int) as uint8_t as ::core::ffi::c_int)
                as uint8_t;
        }
        _ => {}
    }
    *pc = c;
    *ppos = (*ppos).offset(1);
    return 0 as ::core::ffi::c_int;
}
pub const UTF8_ACCEPT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const UTF8_REJECT: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
static mut utf8d: [uint8_t; 364] = [
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    0 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    9 as uint8_t,
    9 as uint8_t,
    9 as uint8_t,
    9 as uint8_t,
    9 as uint8_t,
    9 as uint8_t,
    9 as uint8_t,
    9 as uint8_t,
    9 as uint8_t,
    9 as uint8_t,
    9 as uint8_t,
    9 as uint8_t,
    9 as uint8_t,
    9 as uint8_t,
    9 as uint8_t,
    9 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    7 as uint8_t,
    8 as uint8_t,
    8 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    10 as uint8_t,
    3 as uint8_t,
    3 as uint8_t,
    3 as uint8_t,
    3 as uint8_t,
    3 as uint8_t,
    3 as uint8_t,
    3 as uint8_t,
    3 as uint8_t,
    3 as uint8_t,
    3 as uint8_t,
    3 as uint8_t,
    3 as uint8_t,
    4 as uint8_t,
    3 as uint8_t,
    3 as uint8_t,
    11 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    5 as uint8_t,
    8 as uint8_t,
    8 as uint8_t,
    8 as uint8_t,
    8 as uint8_t,
    8 as uint8_t,
    8 as uint8_t,
    8 as uint8_t,
    8 as uint8_t,
    8 as uint8_t,
    8 as uint8_t,
    8 as uint8_t,
    0 as uint8_t,
    12 as uint8_t,
    24 as uint8_t,
    36 as uint8_t,
    60 as uint8_t,
    96 as uint8_t,
    84 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    48 as uint8_t,
    72 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    0 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    0 as uint8_t,
    12 as uint8_t,
    0 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    24 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    24 as uint8_t,
    12 as uint8_t,
    24 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    24 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    24 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    24 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    36 as uint8_t,
    12 as uint8_t,
    36 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    36 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    36 as uint8_t,
    12 as uint8_t,
    36 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    36 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
    12 as uint8_t,
];
unsafe extern "C" fn utf8_decode(mut state: *mut uint32_t, mut byte: uint8_t) {
    *state = utf8d[(256 as uint32_t)
        .wrapping_add(*state)
        .wrapping_add(utf8d[byte as usize] as uint32_t) as usize] as uint32_t;
}
static mut dispstring_tbl: [uint8_t; 256] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1 as uint8_t,
    1 as uint8_t,
    3 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    2 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    1 as uint8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
unsafe extern "C" fn parser_dispstring(
    mut sfp: *mut sfparse_parser,
    mut dest: *mut sfparse_value,
) -> ::core::ffi::c_int {
    let mut base: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut c: uint8_t = 0;
    let mut utf8state: uint32_t = UTF8_ACCEPT as uint32_t;
    '_c2rust_label: {
        if '%' as ::core::ffi::c_int == *(*sfp).pos as ::core::ffi::c_int {} else {
            __assert_fail(
                b"'%' == *sfp->pos\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/sfparse/sfparse.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                844 as ::core::ffi::c_uint,
                b"int parser_dispstring(sfparse_parser *, sfparse_value *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*sfp).pos = (*sfp).pos.offset(1);
    if parser_eof(sfp) != 0
        || *(*sfp).pos as ::core::ffi::c_int != '"' as ::core::ffi::c_int
    {
        return SFPARSE_ERR_PARSE;
    }
    (*sfp).pos = (*sfp).pos.offset(1);
    base = (*sfp).pos;
    while parser_eof(sfp) == 0 {
        match dispstring_tbl[*(*sfp).pos as usize] as ::core::ffi::c_int {
            0 => return SFPARSE_ERR_PARSE,
            1 => {
                (*sfp).pos = (*sfp).pos.offset(1);
            }
            2 => {
                loop {
                    (*sfp).pos = (*sfp).pos.offset(1);
                    if (*sfp).pos.offset(2 as ::core::ffi::c_int as isize) > (*sfp).end
                        || pctdecode(&raw mut c, &raw mut (*sfp).pos)
                            != 0 as ::core::ffi::c_int
                    {
                        return SFPARSE_ERR_PARSE;
                    }
                    utf8_decode(&raw mut utf8state, c);
                    if utf8state == UTF8_ACCEPT as uint32_t {
                        if !((*sfp).pos != (*sfp).end
                            && *(*sfp).pos as ::core::ffi::c_int
                                == '%' as ::core::ffi::c_int)
                        {
                            break;
                        }
                    } else if utf8state == UTF8_REJECT as uint32_t
                        || (*sfp).pos.offset(1 as ::core::ffi::c_int as isize)
                            > (*sfp).end
                        || *(*sfp).pos as ::core::ffi::c_int != '%' as ::core::ffi::c_int
                    {
                        return SFPARSE_ERR_PARSE
                    }
                }
            }
            3 => {
                '_c2rust_label_0: {
                    if utf8state == 0 as uint32_t {} else {
                        __assert_fail(
                            b"utf8state == UTF8_ACCEPT\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/tmp.cM4Hy2HYmu/src/lib/sfparse/sfparse.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            887 as ::core::ffi::c_uint,
                            b"int parser_dispstring(sfparse_parser *, sfparse_value *)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                };
                if !dest.is_null() {
                    (*dest).r#type = sfparse_type::SFPARSE_TYPE_DISPSTRING;
                    (*dest).flags = SFPARSE_VALUE_FLAG_NONE as uint32_t;
                    (*dest).c2rust_unnamed.vec.len = (*sfp).pos.offset_from(base)
                        as size_t;
                    (*dest).c2rust_unnamed.vec.base = if (*dest).c2rust_unnamed.vec.len
                        == 0 as size_t
                    {
                        ::core::ptr::null_mut::<uint8_t>()
                    } else {
                        base as *mut uint8_t
                    };
                }
                (*sfp).pos = (*sfp).pos.offset(1);
                return 0 as ::core::ffi::c_int;
            }
            _ => {}
        }
    }
    return SFPARSE_ERR_PARSE;
}
static mut bare_item_tbl: [uint8_t; 256] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1 as uint8_t,
    0,
    0,
    7 as uint8_t,
    0,
    0,
    0,
    0,
    6 as uint8_t,
    0,
    0,
    2 as uint8_t,
    0,
    0,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    2 as uint8_t,
    4 as uint8_t,
    0,
    0,
    0,
    0,
    5 as uint8_t,
    3 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    6 as uint8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
unsafe extern "C" fn parser_bare_item(
    mut sfp: *mut sfparse_parser,
    mut dest: *mut sfparse_value,
) -> ::core::ffi::c_int {
    match bare_item_tbl[*(*sfp).pos as usize] as ::core::ffi::c_int {
        0 => return SFPARSE_ERR_PARSE,
        1 => return parser_string(sfp, dest),
        2 => return parser_number(sfp, dest),
        3 => return parser_date(sfp, dest),
        4 => return parser_byteseq(sfp, dest),
        5 => return parser_boolean(sfp, dest),
        6 => return parser_token(sfp, dest),
        7 => return parser_dispstring(sfp, dest),
        _ => {
            '_c2rust_label: {
                __assert_fail(
                    b"0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/tmp/tmp.cM4Hy2HYmu/src/lib/sfparse/sfparse.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    929 as ::core::ffi::c_uint,
                    b"int parser_bare_item(sfparse_parser *, sfparse_value *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            };
            abort();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn sfparse_parser_param(
    mut sfp: *mut sfparse_parser,
    mut dest_key: *mut sfparse_vec,
    mut dest_value: *mut sfparse_value,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    's_39: {
        match (*sfp).state & SFPARSE_STATE_OP_MASK as uint32_t {
            SFPARSE_STATE_BEFORE => {
                rv = parser_skip_inner_list(sfp);
                if rv != 0 as ::core::ffi::c_int {
                    return rv;
                }
            }
            SFPARSE_STATE_BEFORE_PARAMS => {}
            SFPARSE_STATE_PARAMS => {
                break 's_39;
            }
            _ => {
                '_c2rust_label: {
                    __assert_fail(
                        b"0\0".as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/tmp.cM4Hy2HYmu/src/lib/sfparse/sfparse.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        955 as ::core::ffi::c_uint,
                        b"int sfparse_parser_param(sfparse_parser *, sfparse_vec *, sfparse_value *)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                abort();
            }
        }
        parser_set_op_state(sfp, SFPARSE_STATE_PARAMS as uint32_t);
    }
    if parser_eof(sfp) != 0
        || *(*sfp).pos as ::core::ffi::c_int != ';' as ::core::ffi::c_int
    {
        parser_set_op_state(sfp, SFPARSE_STATE_AFTER as uint32_t);
        return SFPARSE_ERR_EOF;
    }
    (*sfp).pos = (*sfp).pos.offset(1);
    parser_discard_sp(sfp);
    if parser_eof(sfp) != 0 {
        return SFPARSE_ERR_PARSE;
    }
    rv = parser_key(sfp, dest_key);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    if parser_eof(sfp) != 0
        || *(*sfp).pos as ::core::ffi::c_int != '=' as ::core::ffi::c_int
    {
        if !dest_value.is_null() {
            (*dest_value).r#type = sfparse_type::SFPARSE_TYPE_BOOLEAN;
            (*dest_value).flags = SFPARSE_VALUE_FLAG_NONE as uint32_t;
            (*dest_value).c2rust_unnamed.boolean = 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
    (*sfp).pos = (*sfp).pos.offset(1);
    if parser_eof(sfp) != 0 {
        return SFPARSE_ERR_PARSE;
    }
    return parser_bare_item(sfp, dest_value);
}
unsafe extern "C" fn parser_skip_params(
    mut sfp: *mut sfparse_parser,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    loop {
        rv = sfparse_parser_param(
            sfp,
            ::core::ptr::null_mut::<sfparse_vec>(),
            ::core::ptr::null_mut::<sfparse_value>(),
        );
        match rv {
            0 => {}
            SFPARSE_ERR_EOF => return 0 as ::core::ffi::c_int,
            SFPARSE_ERR_PARSE => return rv,
            _ => {
                '_c2rust_label: {
                    __assert_fail(
                        b"0\0".as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/tmp.cM4Hy2HYmu/src/lib/sfparse/sfparse.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1009 as ::core::ffi::c_uint,
                        b"int parser_skip_params(sfparse_parser *)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
                abort();
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn sfparse_parser_inner_list(
    mut sfp: *mut sfparse_parser,
    mut dest: *mut sfparse_value,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    's_90: {
        match (*sfp).state & SFPARSE_STATE_OP_MASK as uint32_t {
            SFPARSE_STATE_BEFORE => {
                parser_discard_sp(sfp);
                if parser_eof(sfp) != 0 {
                    return SFPARSE_ERR_PARSE;
                }
                break 's_90;
            }
            SFPARSE_STATE_BEFORE_PARAMS => {
                rv = parser_skip_params(sfp);
                if rv != 0 as ::core::ffi::c_int {
                    return rv;
                }
            }
            SFPARSE_STATE_AFTER => {}
            _ => {
                '_c2rust_label: {
                    __assert_fail(
                        b"0\0".as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/tmp.cM4Hy2HYmu/src/lib/sfparse/sfparse.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1058 as ::core::ffi::c_uint,
                        b"int sfparse_parser_inner_list(sfparse_parser *, sfparse_value *)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                abort();
            }
        }
        if parser_eof(sfp) != 0 {
            return SFPARSE_ERR_PARSE;
        }
        match *(*sfp).pos as ::core::ffi::c_int {
            32 => {
                parser_discard_sp(sfp);
                if parser_eof(sfp) != 0 {
                    return SFPARSE_ERR_PARSE;
                }
            }
            41 => {}
            _ => return SFPARSE_ERR_PARSE,
        }
    }
    if *(*sfp).pos as ::core::ffi::c_int == ')' as ::core::ffi::c_int {
        (*sfp).pos = (*sfp).pos.offset(1);
        parser_unset_inner_list_state(sfp);
        parser_set_op_state(sfp, SFPARSE_STATE_BEFORE_PARAMS as uint32_t);
        return SFPARSE_ERR_EOF;
    }
    rv = parser_bare_item(sfp, dest);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    parser_set_op_state(sfp, SFPARSE_STATE_BEFORE_PARAMS as uint32_t);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn parser_skip_inner_list(
    mut sfp: *mut sfparse_parser,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    loop {
        rv = sfparse_parser_inner_list(sfp, ::core::ptr::null_mut::<sfparse_value>());
        match rv {
            0 => {}
            SFPARSE_ERR_EOF => return 0 as ::core::ffi::c_int,
            SFPARSE_ERR_PARSE => return rv,
            _ => {
                '_c2rust_label: {
                    __assert_fail(
                        b"0\0".as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/tmp.cM4Hy2HYmu/src/lib/sfparse/sfparse.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1094 as ::core::ffi::c_uint,
                        b"int parser_skip_inner_list(sfparse_parser *)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                };
                abort();
            }
        }
    };
}
unsafe extern "C" fn parser_next_key_or_item(
    mut sfp: *mut sfparse_parser,
) -> ::core::ffi::c_int {
    parser_discard_ows(sfp);
    if parser_eof(sfp) != 0 {
        return SFPARSE_ERR_EOF;
    }
    if *(*sfp).pos as ::core::ffi::c_int != ',' as ::core::ffi::c_int {
        return SFPARSE_ERR_PARSE;
    }
    (*sfp).pos = (*sfp).pos.offset(1);
    parser_discard_ows(sfp);
    if parser_eof(sfp) != 0 {
        return SFPARSE_ERR_PARSE;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn parser_dict_value(
    mut sfp: *mut sfparse_parser,
    mut dest: *mut sfparse_value,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    if parser_eof(sfp) != 0
        || *(*sfp).pos as ::core::ffi::c_int != '=' as ::core::ffi::c_int
    {
        if !dest.is_null() {
            (*dest).r#type = sfparse_type::SFPARSE_TYPE_BOOLEAN;
            (*dest).flags = SFPARSE_VALUE_FLAG_NONE as uint32_t;
            (*dest).c2rust_unnamed.boolean = 1 as ::core::ffi::c_int;
        }
        (*sfp).state = SFPARSE_STATE_DICT_BEFORE_PARAMS as uint32_t;
        return 0 as ::core::ffi::c_int;
    }
    (*sfp).pos = (*sfp).pos.offset(1);
    if parser_eof(sfp) != 0 {
        return SFPARSE_ERR_PARSE;
    }
    if *(*sfp).pos as ::core::ffi::c_int == '(' as ::core::ffi::c_int {
        if !dest.is_null() {
            (*dest).r#type = sfparse_type::SFPARSE_TYPE_INNER_LIST;
            (*dest).flags = SFPARSE_VALUE_FLAG_NONE as uint32_t;
        }
        (*sfp).pos = (*sfp).pos.offset(1);
        (*sfp).state = SFPARSE_STATE_DICT_INNER_LIST_BEFORE as uint32_t;
        return 0 as ::core::ffi::c_int;
    }
    rv = parser_bare_item(sfp, dest);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    (*sfp).state = SFPARSE_STATE_DICT_BEFORE_PARAMS as uint32_t;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sfparse_parser_dict(
    mut sfp: *mut sfparse_parser,
    mut dest_key: *mut sfparse_vec,
    mut dest_value: *mut sfparse_value,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    's_78: {
        'c_6249: {
            match (*sfp).state {
                SFPARSE_STATE_DICT_INNER_LIST_BEFORE => {
                    rv = parser_skip_inner_list(sfp);
                    if rv != 0 as ::core::ffi::c_int {
                        return rv;
                    }
                }
                SFPARSE_STATE_DICT_BEFORE_PARAMS => {}
                SFPARSE_STATE_DICT_AFTER => {
                    break 'c_6249;
                }
                SFPARSE_STATE_INITIAL => {
                    parser_discard_sp(sfp);
                    if parser_eof(sfp) != 0 {
                        return SFPARSE_ERR_EOF;
                    }
                    break 's_78;
                }
                _ => {
                    '_c2rust_label: {
                        __assert_fail(
                            b"0\0".as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/tmp.cM4Hy2HYmu/src/lib/sfparse/sfparse.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1201 as ::core::ffi::c_uint,
                            b"int sfparse_parser_dict(sfparse_parser *, sfparse_vec *, sfparse_value *)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    };
                    abort();
                }
            }
            rv = parser_skip_params(sfp);
            if rv != 0 as ::core::ffi::c_int {
                return rv;
            }
        }
        rv = parser_next_key_or_item(sfp);
        if rv != 0 as ::core::ffi::c_int {
            return rv;
        }
    }
    rv = parser_key(sfp, dest_key);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    return parser_dict_value(sfp, dest_value);
}
#[no_mangle]
pub unsafe extern "C" fn sfparse_parser_list(
    mut sfp: *mut sfparse_parser,
    mut dest: *mut sfparse_value,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    's_78: {
        'c_6574: {
            match (*sfp).state {
                SFPARSE_STATE_LIST_INNER_LIST_BEFORE => {
                    rv = parser_skip_inner_list(sfp);
                    if rv != 0 as ::core::ffi::c_int {
                        return rv;
                    }
                }
                SFPARSE_STATE_LIST_BEFORE_PARAMS => {}
                SFPARSE_STATE_LIST_AFTER => {
                    break 'c_6574;
                }
                SFPARSE_STATE_INITIAL => {
                    parser_discard_sp(sfp);
                    if parser_eof(sfp) != 0 {
                        return SFPARSE_ERR_EOF;
                    }
                    break 's_78;
                }
                _ => {
                    '_c2rust_label: {
                        __assert_fail(
                            b"0\0".as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/tmp.cM4Hy2HYmu/src/lib/sfparse/sfparse.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1247 as ::core::ffi::c_uint,
                            b"int sfparse_parser_list(sfparse_parser *, sfparse_value *)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    };
                    abort();
                }
            }
            rv = parser_skip_params(sfp);
            if rv != 0 as ::core::ffi::c_int {
                return rv;
            }
        }
        rv = parser_next_key_or_item(sfp);
        if rv != 0 as ::core::ffi::c_int {
            return rv;
        }
    }
    if *(*sfp).pos as ::core::ffi::c_int == '(' as ::core::ffi::c_int {
        if !dest.is_null() {
            (*dest).r#type = sfparse_type::SFPARSE_TYPE_INNER_LIST;
            (*dest).flags = SFPARSE_VALUE_FLAG_NONE as uint32_t;
        }
        (*sfp).pos = (*sfp).pos.offset(1);
        (*sfp).state = SFPARSE_STATE_LIST_INNER_LIST_BEFORE as uint32_t;
        return 0 as ::core::ffi::c_int;
    }
    rv = parser_bare_item(sfp, dest);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    (*sfp).state = SFPARSE_STATE_LIST_BEFORE_PARAMS as uint32_t;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sfparse_parser_item(
    mut sfp: *mut sfparse_parser,
    mut dest: *mut sfparse_value,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    's_80: {
        'c_6798: {
            match (*sfp).state {
                SFPARSE_STATE_INITIAL => {
                    parser_discard_sp(sfp);
                    if parser_eof(sfp) != 0 {
                        return SFPARSE_ERR_PARSE;
                    }
                    break 's_80;
                }
                SFPARSE_STATE_ITEM_INNER_LIST_BEFORE => {
                    rv = parser_skip_inner_list(sfp);
                    if rv != 0 as ::core::ffi::c_int {
                        return rv;
                    }
                }
                SFPARSE_STATE_ITEM_BEFORE_PARAMS => {}
                SFPARSE_STATE_ITEM_AFTER => {
                    break 'c_6798;
                }
                _ => {
                    '_c2rust_label: {
                        __assert_fail(
                            b"0\0".as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/tmp.cM4Hy2HYmu/src/lib/sfparse/sfparse.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1309 as ::core::ffi::c_uint,
                            b"int sfparse_parser_item(sfparse_parser *, sfparse_value *)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    };
                    abort();
                }
            }
            rv = parser_skip_params(sfp);
            if rv != 0 as ::core::ffi::c_int {
                return rv;
            }
        }
        parser_discard_sp(sfp);
        if parser_eof(sfp) == 0 {
            return SFPARSE_ERR_PARSE;
        }
        return SFPARSE_ERR_EOF;
    }
    if *(*sfp).pos as ::core::ffi::c_int == '(' as ::core::ffi::c_int {
        if !dest.is_null() {
            (*dest).r#type = sfparse_type::SFPARSE_TYPE_INNER_LIST;
            (*dest).flags = SFPARSE_VALUE_FLAG_NONE as uint32_t;
        }
        (*sfp).pos = (*sfp).pos.offset(1);
        (*sfp).state = SFPARSE_STATE_ITEM_INNER_LIST_BEFORE as uint32_t;
        return 0 as ::core::ffi::c_int;
    }
    rv = parser_bare_item(sfp, dest);
    if rv != 0 as ::core::ffi::c_int {
        return rv;
    }
    (*sfp).state = SFPARSE_STATE_ITEM_BEFORE_PARAMS as uint32_t;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sfparse_parser_init(
    mut sfp: *mut sfparse_parser,
    mut data: *const uint8_t,
    mut datalen: size_t,
) {
    if datalen == 0 as size_t {
        (*sfp).end = ::core::ptr::null::<uint8_t>();
        (*sfp).pos = (*sfp).end;
    } else {
        (*sfp).pos = data;
        (*sfp).end = data.offset(datalen as isize);
    }
    (*sfp).state = SFPARSE_STATE_INITIAL as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn sfparse_unescape(
    mut dest: *mut sfparse_vec,
    mut src: *const sfparse_vec,
) {
    let mut p: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut q: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut o: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut len: size_t = 0;
    let mut slen: size_t = 0;
    if (*src).len == 0 as size_t {
        (*dest).len = 0 as size_t;
        return;
    }
    o = (*dest).base;
    p = (*src).base;
    len = (*src).len;
    loop {
        q = memchr(p as *const ::core::ffi::c_void, '\\' as ::core::ffi::c_int, len)
            as *const uint8_t;
        if q.is_null() {
            memcpy(o as *mut ::core::ffi::c_void, p as *const ::core::ffi::c_void, len);
            o = o.offset(len as isize);
            (*dest).len = o.offset_from((*dest).base) as size_t;
            return;
        }
        slen = q.offset_from(p) as size_t;
        memcpy(o as *mut ::core::ffi::c_void, p as *const ::core::ffi::c_void, slen);
        o = o.offset(slen as isize);
        p = q.offset(1 as ::core::ffi::c_int as isize);
        let c2rust_fresh2 = p;
        p = p.offset(1);
        let c2rust_fresh3 = o;
        o = o.offset(1);
        *c2rust_fresh3 = *c2rust_fresh2;
        len = len.wrapping_sub(slen.wrapping_add(2 as size_t));
    };
}
#[no_mangle]
pub unsafe extern "C" fn sfparse_base64decode(
    mut dest: *mut sfparse_vec,
    mut src: *const sfparse_vec,
) {
    static mut index_tbl: [::core::ffi::c_int; 256] = [
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        62 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        63 as ::core::ffi::c_int,
        52 as ::core::ffi::c_int,
        53 as ::core::ffi::c_int,
        54 as ::core::ffi::c_int,
        55 as ::core::ffi::c_int,
        56 as ::core::ffi::c_int,
        57 as ::core::ffi::c_int,
        58 as ::core::ffi::c_int,
        59 as ::core::ffi::c_int,
        60 as ::core::ffi::c_int,
        61 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        5 as ::core::ffi::c_int,
        6 as ::core::ffi::c_int,
        7 as ::core::ffi::c_int,
        8 as ::core::ffi::c_int,
        9 as ::core::ffi::c_int,
        10 as ::core::ffi::c_int,
        11 as ::core::ffi::c_int,
        12 as ::core::ffi::c_int,
        13 as ::core::ffi::c_int,
        14 as ::core::ffi::c_int,
        15 as ::core::ffi::c_int,
        16 as ::core::ffi::c_int,
        17 as ::core::ffi::c_int,
        18 as ::core::ffi::c_int,
        19 as ::core::ffi::c_int,
        20 as ::core::ffi::c_int,
        21 as ::core::ffi::c_int,
        22 as ::core::ffi::c_int,
        23 as ::core::ffi::c_int,
        24 as ::core::ffi::c_int,
        25 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        26 as ::core::ffi::c_int,
        27 as ::core::ffi::c_int,
        28 as ::core::ffi::c_int,
        29 as ::core::ffi::c_int,
        30 as ::core::ffi::c_int,
        31 as ::core::ffi::c_int,
        32 as ::core::ffi::c_int,
        33 as ::core::ffi::c_int,
        34 as ::core::ffi::c_int,
        35 as ::core::ffi::c_int,
        36 as ::core::ffi::c_int,
        37 as ::core::ffi::c_int,
        38 as ::core::ffi::c_int,
        39 as ::core::ffi::c_int,
        40 as ::core::ffi::c_int,
        41 as ::core::ffi::c_int,
        42 as ::core::ffi::c_int,
        43 as ::core::ffi::c_int,
        44 as ::core::ffi::c_int,
        45 as ::core::ffi::c_int,
        46 as ::core::ffi::c_int,
        47 as ::core::ffi::c_int,
        48 as ::core::ffi::c_int,
        49 as ::core::ffi::c_int,
        50 as ::core::ffi::c_int,
        51 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
        -1 as ::core::ffi::c_int,
    ];
    let mut o: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut p: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut end: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut n: uint32_t = 0;
    let mut i: size_t = 0;
    let mut left: size_t = 0;
    let mut idx: ::core::ffi::c_int = 0;
    if (*src).len == 0 as size_t {
        (*dest).len = 0 as size_t;
        return;
    }
    o = (*dest).base;
    p = (*src).base;
    left = (*src).len & 0x3 as size_t;
    if left == 0 as size_t
        && *(*src).base.offset((*src).len.wrapping_sub(1 as size_t) as isize)
            as ::core::ffi::c_int == '=' as ::core::ffi::c_int
    {
        left = 4 as size_t;
    }
    end = (*src).base.offset((*src).len as isize).offset(-(left as isize));
    while p != end {
        n = 0 as uint32_t;
        i = 1 as size_t;
        while i <= 4 as size_t {
            idx = index_tbl[*p as usize];
            '_c2rust_label: {
                if idx != -1 as ::core::ffi::c_int {} else {
                    __assert_fail(
                        b"idx != -1\0".as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/tmp.cM4Hy2HYmu/src/lib/sfparse/sfparse.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1426 as ::core::ffi::c_uint,
                        b"void sfparse_base64decode(sfparse_vec *, const sfparse_vec *)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            };
            n = n
                .wrapping_add(
                    (idx << (24 as size_t).wrapping_sub(i.wrapping_mul(6 as size_t)))
                        as uint32_t,
                );
            i = i.wrapping_add(1);
            p = p.offset(1);
        }
        let c2rust_fresh4 = o;
        o = o.offset(1);
        *c2rust_fresh4 = (n >> 16 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh5 = o;
        o = o.offset(1);
        *c2rust_fresh5 = (n >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
        let c2rust_fresh6 = o;
        o = o.offset(1);
        *c2rust_fresh6 = (n & 0xff as uint32_t) as uint8_t;
    }
    '_fin: {
        match left {
            0 => {
                break '_fin;
            }
            1 => {
                '_c2rust_label_0: {
                    __assert_fail(
                        b"0\0".as_ptr() as *const ::core::ffi::c_char,
                        b"/tmp/tmp.cM4Hy2HYmu/src/lib/sfparse/sfparse.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        1440 as ::core::ffi::c_uint,
                        b"void sfparse_base64decode(sfparse_vec *, const sfparse_vec *)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
                abort();
            }
            3 => {
                if *(*src).base.offset((*src).len.wrapping_sub(1 as size_t) as isize)
                    as ::core::ffi::c_int == '=' as ::core::ffi::c_int
                {
                    left = 2 as size_t;
                }
            }
            4 => {
                '_c2rust_label_1: {
                    if '=' as ::core::ffi::c_int
                        == *(*src)
                            .base
                            .offset((*src).len.wrapping_sub(1 as size_t) as isize)
                            as ::core::ffi::c_int
                    {} else {
                        __assert_fail(
                            b"'=' == src->base[src->len - 1]\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"/tmp/tmp.cM4Hy2HYmu/src/lib/sfparse/sfparse.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1449 as ::core::ffi::c_uint,
                            b"void sfparse_base64decode(sfparse_vec *, const sfparse_vec *)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                };
                if *(*src).base.offset((*src).len.wrapping_sub(2 as size_t) as isize)
                    as ::core::ffi::c_int == '=' as ::core::ffi::c_int
                {
                    left = 2 as size_t;
                } else {
                    left = 3 as size_t;
                }
            }
            _ => {}
        }
        match left {
            2 => {
                let c2rust_fresh7 = p;
                p = p.offset(1);
                *o = (index_tbl[*c2rust_fresh7 as usize] << 2 as ::core::ffi::c_int)
                    as uint8_t;
                let c2rust_fresh8 = p;
                p = p.offset(1);
                let c2rust_fresh9 = o;
                o = o.offset(1);
                let c2rust_lvalue_ptr = &raw mut *c2rust_fresh9;
                *c2rust_lvalue_ptr = (*c2rust_lvalue_ptr as ::core::ffi::c_int
                    | (index_tbl[*c2rust_fresh8 as usize] >> 4 as ::core::ffi::c_int)
                        as uint8_t as ::core::ffi::c_int) as uint8_t;
            }
            3 => {
                let c2rust_fresh10 = p;
                p = p.offset(1);
                n = (index_tbl[*c2rust_fresh10 as usize] << 10 as ::core::ffi::c_int)
                    as uint32_t;
                let c2rust_fresh11 = p;
                p = p.offset(1);
                n = n
                    .wrapping_add(
                        (index_tbl[*c2rust_fresh11 as usize] << 4 as ::core::ffi::c_int)
                            as uint32_t,
                    );
                let c2rust_fresh12 = p;
                p = p.offset(1);
                n = n
                    .wrapping_add(
                        (index_tbl[*c2rust_fresh12 as usize] >> 2 as ::core::ffi::c_int)
                            as uint32_t,
                    );
                let c2rust_fresh13 = o;
                o = o.offset(1);
                *c2rust_fresh13 = (n >> 8 as ::core::ffi::c_int & 0xff as uint32_t)
                    as uint8_t;
                let c2rust_fresh14 = o;
                o = o.offset(1);
                *c2rust_fresh14 = (n & 0xff as uint32_t) as uint8_t;
            }
            _ => {}
        }
    }
    (*dest).len = o.offset_from((*dest).base) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn sfparse_pctdecode(
    mut dest: *mut sfparse_vec,
    mut src: *const sfparse_vec,
) {
    let mut p: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut q: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut o: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut len: size_t = 0;
    let mut slen: size_t = 0;
    if (*src).len == 0 as size_t {
        (*dest).len = 0 as size_t;
        return;
    }
    o = (*dest).base;
    p = (*src).base;
    len = (*src).len;
    loop {
        q = memchr(p as *const ::core::ffi::c_void, '%' as ::core::ffi::c_int, len)
            as *const uint8_t;
        if q.is_null() {
            memcpy(o as *mut ::core::ffi::c_void, p as *const ::core::ffi::c_void, len);
            o = o.offset(len as isize);
            (*dest).len = o.offset_from((*dest).base) as size_t;
            return;
        }
        slen = q.offset_from(p) as size_t;
        memcpy(o as *mut ::core::ffi::c_void, p as *const ::core::ffi::c_void, slen);
        o = o.offset(slen as isize);
        p = q.offset(1 as ::core::ffi::c_int as isize);
        let c2rust_fresh15 = o;
        o = o.offset(1);
        pctdecode(c2rust_fresh15, &raw mut p);
        len = len.wrapping_sub(slen.wrapping_add(3 as size_t));
    };
}
