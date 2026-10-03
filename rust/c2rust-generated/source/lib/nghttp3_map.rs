extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn nghttp3_mem_free(mem: *const nghttp3_mem, ptr: *mut ::core::ffi::c_void);
    fn nghttp3_mem_calloc(
        mem: *const nghttp3_mem,
        nmemb: size_t,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type uint8_t = u8;
pub type uint64_t = u64;
pub type ptrdiff_t = isize;
pub type nghttp3_ssize = ptrdiff_t;
pub type nghttp3_malloc = Option<
    unsafe extern "C" fn(size_t, *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
>;
pub type nghttp3_free = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
>;
pub type nghttp3_calloc = Option<
    unsafe extern "C" fn(
        size_t,
        size_t,
        *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void,
>;
pub type nghttp3_realloc = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        size_t,
        *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_mem {
    pub user_data: *mut ::core::ffi::c_void,
    pub malloc: nghttp3_malloc,
    pub free: nghttp3_free,
    pub calloc: nghttp3_calloc,
    pub realloc: nghttp3_realloc,
}
pub type nghttp3_map_key_type = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_map {
    pub keys: *mut nghttp3_map_key_type,
    pub data: *mut *mut ::core::ffi::c_void,
    pub psl: *mut uint8_t,
    pub mem: *const nghttp3_mem,
    pub seed: uint64_t,
    pub size: size_t,
    pub hashbits: size_t,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub const NGHTTP3_ERR_INVALID_ARGUMENT: ::core::ffi::c_int = -101 as ::core::ffi::c_int;
pub const NGHTTP3_ERR_NOMEM: ::core::ffi::c_int = -901 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const NGHTTP3_INITIAL_HASHBITS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn nghttp3_map_init(
    mut map: *mut nghttp3_map,
    mut seed: uint64_t,
    mut mem: *const nghttp3_mem,
) {
    *map = nghttp3_map {
        keys: ::core::ptr::null_mut::<nghttp3_map_key_type>(),
        data: ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        psl: ::core::ptr::null_mut::<uint8_t>(),
        mem: mem,
        seed: seed,
        size: 0,
        hashbits: 0,
    };
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_map_free(mut map: *mut nghttp3_map) {
    if map.is_null() {
        return;
    }
    nghttp3_mem_free((*map).mem, (*map).keys as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_map_each(
    mut map: *const nghttp3_map,
    mut func: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    mut ptr: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    let mut i: size_t = 0;
    let mut tablelen: size_t = 0;
    if (*map).size == 0 as size_t {
        return 0 as ::core::ffi::c_int;
    }
    tablelen = (1 as ::core::ffi::c_int as size_t) << (*map).hashbits;
    i = 0 as size_t;
    while i < tablelen {
        if *(*map).psl.offset(i as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            rv = func
                .expect(
                    "non-null function pointer",
                )(*(*map).data.offset(i as isize), ptr);
            if rv != 0 as ::core::ffi::c_int {
                return rv;
            }
        }
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
pub const NGHTTP3_MAP_HASHER: ::core::ffi::c_ulonglong = 0xf1357aea2e62a9c5
    as ::core::ffi::c_ulonglong;
pub const NGHTTP3_MAP_FIBO: ::core::ffi::c_ulonglong = 0x9e3779b97f4a7c15
    as ::core::ffi::c_ulonglong;
unsafe extern "C" fn map_index(
    mut map: *const nghttp3_map,
    mut key: nghttp3_map_key_type,
) -> size_t {
    key = key.wrapping_add((*map).seed);
    key = (key as ::core::ffi::c_ulonglong).wrapping_mul(NGHTTP3_MAP_HASHER)
        as nghttp3_map_key_type;
    return ((key as ::core::ffi::c_ulonglong).wrapping_mul(NGHTTP3_MAP_FIBO)
        >> (64 as size_t).wrapping_sub((*map).hashbits)) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_map_print_distance(mut map: *const nghttp3_map) {
    let mut i: size_t = 0;
    let mut idx: size_t = 0;
    let mut tablelen: size_t = 0;
    if (*map).size == 0 as size_t {
        return;
    }
    tablelen = (1 as ::core::ffi::c_int as size_t) << (*map).hashbits;
    i = 0 as size_t;
    while i < tablelen {
        if *(*map).psl.offset(i as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            fprintf(
                stderr,
                b"@%zu <EMPTY>\n\0".as_ptr() as *const ::core::ffi::c_char,
                i,
            );
        } else {
            idx = map_index(map, *(*map).keys.offset(i as isize));
            fprintf(
                stderr,
                b"@%zu key=%lu base=%zu distance=%u\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                i,
                *(*map).keys.offset(i as isize),
                idx,
                *(*map).psl.offset(i as isize) as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int,
            );
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn map_set_entry(
    mut map: *mut nghttp3_map,
    mut idx: size_t,
    mut key: nghttp3_map_key_type,
    mut data: *mut ::core::ffi::c_void,
    mut psl: size_t,
) {
    *(*map).keys.offset(idx as isize) = key;
    *(*map).data.offset(idx as isize) = data;
    *(*map).psl.offset(idx as isize) = psl as uint8_t;
}
unsafe extern "C" fn map_insert(
    mut map: *mut nghttp3_map,
    mut key: nghttp3_map_key_type,
    mut data: *mut ::core::ffi::c_void,
) -> nghttp3_ssize {
    let mut idx: size_t = map_index(map, key);
    let mut mask: size_t = ((1 as ::core::ffi::c_int as size_t) << (*map).hashbits)
        .wrapping_sub(1 as size_t);
    let mut psl: size_t = 1 as size_t;
    let mut kpsl: size_t = 0;
    loop {
        kpsl = *(*map).psl.offset(idx as isize) as size_t;
        if kpsl == 0 as size_t {
            map_set_entry(map, idx, key, data, psl);
            (*map).size = (*map).size.wrapping_add(1);
            return idx as nghttp3_ssize;
        }
        if psl > kpsl {
            let mut t: nghttp3_map_key_type = key;
            key = *(*map).keys.offset(idx as isize);
            *(*map).keys.offset(idx as isize) = t;
            let mut t_0: *mut ::core::ffi::c_void = data;
            data = *(*map).data.offset(idx as isize);
            *(*map).data.offset(idx as isize) = t_0;
            let mut t_1: uint8_t = psl as uint8_t;
            psl = *(*map).psl.offset(idx as isize) as size_t;
            *(*map).psl.offset(idx as isize) = t_1;
        } else if *(*map).keys.offset(idx as isize) == key {
            return NGHTTP3_ERR_INVALID_ARGUMENT as nghttp3_ssize
        }
        psl = psl.wrapping_add(1);
        idx = idx.wrapping_add(1 as size_t) & mask;
    };
}
pub const NGHTTP3_MAP_MAX_HASHBITS: usize = ::core::mem::size_of::<size_t>()
    .wrapping_mul(8usize)
    .wrapping_sub(1usize);
unsafe extern "C" fn map_resize(
    mut map: *mut nghttp3_map,
    mut new_hashbits: size_t,
) -> ::core::ffi::c_int {
    let mut i: size_t = 0;
    let mut tablelen: size_t = 0;
    let mut idx: nghttp3_ssize = 0;
    let mut new_map: nghttp3_map = nghttp3_map {
        keys: ::core::ptr::null_mut::<nghttp3_map_key_type>(),
        data: ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        psl: ::core::ptr::null_mut::<uint8_t>(),
        mem: (*map).mem,
        seed: (*map).seed,
        size: 0,
        hashbits: new_hashbits,
    };
    let mut buf: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    if new_hashbits > NGHTTP3_MAP_MAX_HASHBITS {
        return NGHTTP3_ERR_NOMEM;
    }
    tablelen = (1 as ::core::ffi::c_int as size_t) << new_hashbits;
    buf = nghttp3_mem_calloc(
        (*map).mem,
        tablelen,
        ::core::mem::size_of::<nghttp3_map_key_type>()
            .wrapping_add(::core::mem::size_of::<*mut ::core::ffi::c_void>())
            .wrapping_add(::core::mem::size_of::<uint8_t>()),
    );
    if buf.is_null() {
        return NGHTTP3_ERR_NOMEM;
    }
    new_map.keys = buf as *mut nghttp3_map_key_type;
    new_map.data = (new_map.keys as *mut uint8_t)
        .offset(
            tablelen.wrapping_mul(::core::mem::size_of::<nghttp3_map_key_type>())
                as isize,
        ) as *mut ::core::ffi::c_void as *mut *mut ::core::ffi::c_void;
    new_map.psl = (new_map.data as *mut uint8_t)
        .offset(
            tablelen.wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>())
                as isize,
        );
    if (*map).size != 0 {
        tablelen = (1 as ::core::ffi::c_int as size_t) << (*map).hashbits;
        i = 0 as size_t;
        while i < tablelen {
            if *(*map).psl.offset(i as isize) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            {
                idx = map_insert(
                    &raw mut new_map,
                    *(*map).keys.offset(i as isize),
                    *(*map).data.offset(i as isize),
                );
                '_c2rust_label: {
                    if idx >= 0 as nghttp3_ssize {} else {
                        __assert_fail(
                            b"idx >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                            b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_map.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            221 as ::core::ffi::c_uint,
                            b"int map_resize(nghttp3_map *, size_t)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                };
            }
            i = i.wrapping_add(1);
        }
    }
    nghttp3_mem_free((*map).mem, (*map).keys as *mut ::core::ffi::c_void);
    (*map).keys = new_map.keys;
    (*map).data = new_map.data;
    (*map).psl = new_map.psl;
    (*map).hashbits = new_hashbits;
    return 0 as ::core::ffi::c_int;
}
pub const NGHTTP3_MAX_PSL_RESIZE_THRESH: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn nghttp3_map_insert(
    mut map: *mut nghttp3_map,
    mut key: nghttp3_map_key_type,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut rv: ::core::ffi::c_int = 0;
    let mut tablelen: size_t = 0;
    let mut idx: nghttp3_ssize = 0;
    '_c2rust_label: {
        if !data.is_null() {} else {
            __assert_fail(
                b"data\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.F3V9xfB3h8/src/lib/nghttp3_map.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                243 as ::core::ffi::c_uint,
                b"int nghttp3_map_insert(nghttp3_map *, nghttp3_map_key_type, void *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    tablelen = (1 as ::core::ffi::c_int as size_t) << (*map).hashbits;
    if (*map).size.wrapping_add(1 as size_t)
        >= tablelen.wrapping_sub(tablelen >> 3 as ::core::ffi::c_int)
    {
        rv = map_resize(
            map,
            if (*map).hashbits != 0 {
                (*map).hashbits.wrapping_add(1 as size_t)
            } else {
                NGHTTP3_INITIAL_HASHBITS as size_t
            },
        );
        if rv != 0 as ::core::ffi::c_int {
            return rv;
        }
        idx = map_insert(map, key, data);
        if idx < 0 as nghttp3_ssize {
            return idx as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
    idx = map_insert(map, key, data);
    if idx < 0 as nghttp3_ssize {
        return idx as ::core::ffi::c_int;
    }
    if (*(*map).psl.offset(idx as isize) as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        < NGHTTP3_MAX_PSL_RESIZE_THRESH
    {
        return 0 as ::core::ffi::c_int;
    }
    rv = map_resize(map, (*map).hashbits.wrapping_add(1 as size_t));
    if rv != 0 as ::core::ffi::c_int {
        nghttp3_map_remove(map, key);
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_map_find(
    mut map: *const nghttp3_map,
    mut key: nghttp3_map_key_type,
) -> *mut ::core::ffi::c_void {
    let mut idx: size_t = 0;
    let mut psl: size_t = 1 as size_t;
    let mut mask: size_t = 0;
    if (*map).size == 0 as size_t {
        return NULL;
    }
    idx = map_index(map, key);
    mask = ((1 as ::core::ffi::c_int as size_t) << (*map).hashbits)
        .wrapping_sub(1 as size_t);
    loop {
        if psl > *(*map).psl.offset(idx as isize) as size_t {
            return NULL;
        }
        if *(*map).keys.offset(idx as isize) == key {
            return *(*map).data.offset(idx as isize);
        }
        psl = psl.wrapping_add(1);
        idx = idx.wrapping_add(1 as size_t) & mask;
    };
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_map_remove(
    mut map: *mut nghttp3_map,
    mut key: nghttp3_map_key_type,
) -> ::core::ffi::c_int {
    let mut idx: size_t = 0;
    let mut dest: size_t = 0;
    let mut psl: size_t = 1 as size_t;
    let mut kpsl: size_t = 0;
    let mut mask: size_t = 0;
    if (*map).size == 0 as size_t {
        return NGHTTP3_ERR_INVALID_ARGUMENT;
    }
    idx = map_index(map, key);
    mask = ((1 as ::core::ffi::c_int as size_t) << (*map).hashbits)
        .wrapping_sub(1 as size_t);
    loop {
        if psl > *(*map).psl.offset(idx as isize) as size_t {
            return NGHTTP3_ERR_INVALID_ARGUMENT;
        }
        if *(*map).keys.offset(idx as isize) == key {
            dest = idx;
            idx = idx.wrapping_add(1 as size_t) & mask;
            loop {
                kpsl = *(*map).psl.offset(idx as isize) as size_t;
                if kpsl <= 1 as size_t {
                    *(*map).psl.offset(dest as isize) = 0 as uint8_t;
                    break;
                } else {
                    map_set_entry(
                        map,
                        dest,
                        *(*map).keys.offset(idx as isize),
                        *(*map).data.offset(idx as isize),
                        kpsl.wrapping_sub(1 as size_t),
                    );
                    dest = idx;
                    idx = idx.wrapping_add(1 as size_t) & mask;
                }
            }
            (*map).size = (*map).size.wrapping_sub(1);
            return 0 as ::core::ffi::c_int;
        }
        psl = psl.wrapping_add(1);
        idx = idx.wrapping_add(1 as size_t) & mask;
    };
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_map_clear(mut map: *mut nghttp3_map) {
    if (*map).size == 0 as size_t {
        return;
    }
    memset(
        (*map).psl as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<uint8_t>()
            .wrapping_mul((1 as ::core::ffi::c_int as size_t) << (*map).hashbits),
    );
    (*map).size = 0 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_map_size(mut map: *const nghttp3_map) -> size_t {
    return (*map).size;
}
