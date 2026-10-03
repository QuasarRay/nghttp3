extern "C" {
    fn nghttp3_mem_free(mem: *const nghttp3_mem, ptr: *mut ::core::ffi::c_void);
    fn nghttp3_mem_realloc(
        mem: *const nghttp3_mem,
        ptr: *mut ::core::ffi::c_void,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
}
pub type size_t = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_pq_entry {
    pub index: size_t,
}
pub type nghttp3_pq_less = Option<
    unsafe extern "C" fn(
        *const nghttp3_pq_entry,
        *const nghttp3_pq_entry,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp3_pq {
    pub q: *mut *mut nghttp3_pq_entry,
    pub mem: *const nghttp3_mem,
    pub length: size_t,
    pub capacity: size_t,
    pub less: nghttp3_pq_less,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const NGHTTP3_ERR_NOMEM: ::core::ffi::c_int = -901 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn nghttp3_max_unsigned_long_int(
    mut a: ::core::ffi::c_ulong,
    mut b: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    return if a < b { b } else { a };
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_pq_init(
    mut pq: *mut nghttp3_pq,
    mut less: nghttp3_pq_less,
    mut mem: *const nghttp3_mem,
) {
    (*pq).q = ::core::ptr::null_mut::<*mut nghttp3_pq_entry>();
    (*pq).mem = mem;
    (*pq).length = 0 as size_t;
    (*pq).capacity = 0 as size_t;
    (*pq).less = less;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_pq_free(mut pq: *mut nghttp3_pq) {
    if pq.is_null() {
        return;
    }
    nghttp3_mem_free((*pq).mem, (*pq).q as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn swap(mut pq: *mut nghttp3_pq, mut i: size_t, mut j: size_t) {
    let mut a: *mut nghttp3_pq_entry = *(*pq).q.offset(i as isize);
    let mut b: *mut nghttp3_pq_entry = *(*pq).q.offset(j as isize);
    *(*pq).q.offset(i as isize) = b;
    (*b).index = i;
    *(*pq).q.offset(j as isize) = a;
    (*a).index = j;
}
unsafe extern "C" fn bubble_up(mut pq: *mut nghttp3_pq, mut index: size_t) {
    let mut parent: size_t = 0;
    while index != 0 {
        parent = index.wrapping_sub(1 as size_t).wrapping_div(2 as size_t);
        if (*pq)
            .less
            .expect(
                "non-null function pointer",
            )(*(*pq).q.offset(index as isize), *(*pq).q.offset(parent as isize)) == 0
        {
            return;
        }
        swap(pq, parent, index);
        index = parent;
    }
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_pq_push(
    mut pq: *mut nghttp3_pq,
    mut item: *mut nghttp3_pq_entry,
) -> ::core::ffi::c_int {
    if (*pq).capacity <= (*pq).length {
        let mut nq: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        let mut ncapacity: size_t = 0;
        ncapacity = nghttp3_max_unsigned_long_int(
            4 as ::core::ffi::c_ulong,
            ((*pq).capacity as ::core::ffi::c_ulong)
                .wrapping_mul(2 as ::core::ffi::c_ulong),
        ) as size_t;
        nq = nghttp3_mem_realloc(
            (*pq).mem,
            (*pq).q as *mut ::core::ffi::c_void,
            ncapacity.wrapping_mul(::core::mem::size_of::<*mut nghttp3_pq_entry>()),
        );
        if nq.is_null() {
            return NGHTTP3_ERR_NOMEM;
        }
        (*pq).capacity = ncapacity;
        (*pq).q = nq as *mut *mut nghttp3_pq_entry;
    }
    *(*pq).q.offset((*pq).length as isize) = item;
    (*item).index = (*pq).length;
    (*pq).length = (*pq).length.wrapping_add(1);
    bubble_up(pq, (*item).index);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_pq_top(
    mut pq: *const nghttp3_pq,
) -> *mut nghttp3_pq_entry {
    '_c2rust_label: {
        if (*pq).length != 0 {} else {
            __assert_fail(
                b"pq->length\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_pq.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                100 as ::core::ffi::c_uint,
                b"nghttp3_pq_entry *nghttp3_pq_top(const nghttp3_pq *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return *(*pq).q.offset(0isize);
}
unsafe extern "C" fn bubble_down(mut pq: *mut nghttp3_pq, mut index: size_t) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut minindex: size_t = 0;
    loop {
        j = index.wrapping_mul(2 as size_t).wrapping_add(1 as size_t);
        minindex = index;
        i = 0 as size_t;
        while i < 2 as size_t {
            if j >= (*pq).length {
                break;
            }
            if (*pq)
                .less
                .expect(
                    "non-null function pointer",
                )(*(*pq).q.offset(j as isize), *(*pq).q.offset(minindex as isize)) != 0
            {
                minindex = j;
            }
            i = i.wrapping_add(1);
            j = j.wrapping_add(1);
        }
        if minindex == index {
            return;
        }
        swap(pq, index, minindex);
        index = minindex;
    };
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_pq_pop(mut pq: *mut nghttp3_pq) {
    '_c2rust_label: {
        if (*pq).length != 0 {} else {
            __assert_fail(
                b"pq->length\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_pq.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                131 as ::core::ffi::c_uint,
                b"void nghttp3_pq_pop(nghttp3_pq *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    *(*pq).q.offset(0isize) = *(*pq)
        .q
        .offset((*pq).length.wrapping_sub(1 as size_t) as isize);
    (**(*pq).q.offset(0isize)).index = 0 as size_t;
    (*pq).length = (*pq).length.wrapping_sub(1);
    bubble_down(pq, 0 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_pq_remove(
    mut pq: *mut nghttp3_pq,
    mut item: *mut nghttp3_pq_entry,
) {
    '_c2rust_label: {
        if *(*pq).q.offset((*item).index as isize) == item {} else {
            __assert_fail(
                b"pq->q[item->index] == item\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/tmp.cM4Hy2HYmu/src/lib/nghttp3_pq.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                140 as ::core::ffi::c_uint,
                b"void nghttp3_pq_remove(nghttp3_pq *, nghttp3_pq_entry *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*item).index == 0 as size_t {
        nghttp3_pq_pop(pq);
        return;
    }
    if (*item).index == (*pq).length.wrapping_sub(1 as size_t) {
        (*pq).length = (*pq).length.wrapping_sub(1);
        return;
    }
    *(*pq).q.offset((*item).index as isize) = *(*pq)
        .q
        .offset((*pq).length.wrapping_sub(1 as size_t) as isize);
    (**(*pq).q.offset((*item).index as isize)).index = (*item).index;
    (*pq).length = (*pq).length.wrapping_sub(1);
    if (*pq)
        .less
        .expect(
            "non-null function pointer",
        )(item, *(*pq).q.offset((*item).index as isize)) != 0
    {
        bubble_down(pq, (*item).index);
    } else {
        bubble_up(pq, (*item).index);
    };
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_pq_empty(
    mut pq: *const nghttp3_pq,
) -> ::core::ffi::c_int {
    return ((*pq).length == 0 as size_t) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_pq_size(mut pq: *const nghttp3_pq) -> size_t {
    return (*pq).length;
}
#[no_mangle]
pub unsafe extern "C" fn nghttp3_pq_clear(mut pq: *mut nghttp3_pq) {
    (*pq).length = 0 as size_t;
}
