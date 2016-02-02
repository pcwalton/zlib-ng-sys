// zlib-ng-sys/lib.rs

#![allow(non_camel_case_types, non_snake_case)]

extern crate libc;

use libc::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_void, off_t, size_t, uint32_t};
use std::mem;

pub const Z_NO_FLUSH: c_int = 0;
pub const Z_PARTIAL_FLUSH: c_int = 1;
pub const Z_SYNC_FLUSH: c_int = 2;
pub const Z_FULL_FLUSH: c_int = 3;
pub const Z_FINISH: c_int = 4;
pub const Z_BLOCK: c_int = 5;
pub const Z_TREES: c_int = 6;

pub const Z_OK: c_int = 0;
pub const Z_STREAM_END: c_int = 1;
pub const Z_NEED_DICT: c_int = 2;
pub const Z_ERRNO: c_int = -1;
pub const Z_STREAM_ERROR: c_int = -2;
pub const Z_DATA_ERROR: c_int = -3;
pub const Z_MEM_ERROR: c_int = -4;
pub const Z_BUF_ERROR: c_int = -5;
pub const Z_VERSION_ERROR: c_int = -6;

pub const Z_NO_COMPRESSION: c_int = 0;
pub const Z_BEST_SPEED: c_int = 1;
pub const Z_BEST_COMPRESSION: c_int = 9;
pub const Z_DEFAULT_COMPRESSION: c_int = -1;

pub const Z_FILTERED: c_int = 1;
pub const Z_HUFFMAN_ONLY: c_int = 2;
pub const Z_RLE: c_int = 3;
pub const Z_FIXED: c_int = 4;
pub const Z_DEFAULT_STRATEGY: c_int = 0;

pub const Z_BINARY: c_int = 0;
pub const Z_TEXT: c_int = 1;
pub const Z_ASCII: c_int = Z_TEXT;
pub const Z_UNKNOWN: c_int = 2;

pub const Z_DEFLATED: c_int = 8;

pub static ZLIB_VERSION: &'static [u8] = b"1.2.8.zlib-ng\0";

pub type alloc_func = Option<unsafe extern "C" fn(opaque: *mut c_void,
                                                  items: c_uint,
                                                  size: c_uint)
                                                  -> *mut c_void>;

pub type free_func = Option<unsafe extern "C" fn(opaque: *mut c_void, address: *mut c_void)>;

pub enum Struct_internal_state {}

#[repr(C)]
#[derive(Copy)]
pub struct Struct_z_stream_s {
    pub next_in: *const c_uchar,
    pub avail_in: uint32_t,
    pub total_in: size_t,
    pub next_out: *mut c_uchar,
    pub avail_out: uint32_t,
    pub total_out: size_t,
    pub msg: *const c_char,
    pub state: *mut Struct_internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: *mut c_void,
    pub data_type: c_int,
    pub adler: uint32_t,
    pub reserved: c_ulong,
}

impl Clone for Struct_z_stream_s {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for Struct_z_stream_s {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub type z_stream = Struct_z_stream_s;

type z_streamp = *mut z_stream;

#[repr(C)]
#[derive(Copy)]
pub struct Struct_gz_header_s {
    pub text: c_int,
    pub time: c_ulong,
    pub xflags: c_int,
    pub os: c_int,
    pub extra: *mut c_uchar,
    pub extra_len: c_uint,
    pub extra_max: c_uint,
    pub name: *mut c_uchar,
    pub name_max: c_uint,
    pub comment: *mut c_uchar,
    pub comm_max: c_uint,
    pub hcrc: c_int,
    pub done: c_int,
}

impl Clone for Struct_gz_header_s {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for Struct_gz_header_s {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub type gz_header = Struct_gz_header_s;
pub type gz_headerp = *mut gz_header;

pub type in_func = Option<unsafe extern "C" fn(arg1: *mut c_void, arg2: *mut *const c_uchar)
                                               -> uint32_t>;

pub type out_func = Option<unsafe extern "C" fn(arg1: *mut c_void,
                                                arg2: *mut c_uchar,
                                                arg3: uint32_t)
                                                -> c_int>;

pub type gzFile = *mut Struct_gzFile_s;

#[repr(C)]
#[derive(Copy)]
pub struct Struct_gzFile_s {
    pub have: c_uint,
    pub next: *mut c_uchar,
    pub pos: off_t,
}

impl Clone for Struct_gzFile_s {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for Struct_gzFile_s {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[inline]
pub unsafe fn deflateInit(strm: *mut z_stream, level: c_int) -> c_int {
    deflateInit_(strm,
                 level,
                 &ZLIB_VERSION[0] as *const u8 as *const i8,
                 mem::size_of::<z_stream>() as c_int)
}

#[inline]
pub unsafe fn inflateInit(strm: *mut z_stream) -> c_int {
    inflateInit_(strm,
                 &ZLIB_VERSION[0] as *const u8 as *const i8,
                 mem::size_of::<z_stream>() as c_int)
}

extern "C" {
    pub fn zlibVersion() -> *const c_char;
    pub fn deflate(strm: *mut z_stream, flush: c_int) -> c_int;
    pub fn deflateEnd(strm: *mut z_stream) -> c_int;
    pub fn inflate(strm: *mut z_stream, flush: c_int) -> c_int;
    pub fn inflateEnd(strm: *mut z_stream) -> c_int;
    pub fn deflateSetDictionary(strm: *mut z_stream,
                                dictionary: *const c_uchar,
                                dictLength: c_uint)
                                -> c_int;
    pub fn deflateCopy(dest: *mut z_stream, source: *mut z_stream) -> c_int;
    pub fn deflateReset(strm: *mut z_stream) -> c_int;
    pub fn deflateParams(strm: *mut z_stream, level: c_int, strategy: c_int) -> c_int;
    pub fn deflateTune(strm: *mut z_stream,
                       good_length: c_int,
                       max_lazy: c_int,
                       nice_length: c_int,
                       max_chain: c_int)
                       -> c_int;
    pub fn deflateBound(strm: *mut z_stream, sourceLen: c_ulong) -> c_ulong;
    pub fn deflatePending(strm: *mut z_stream, pending: *mut uint32_t, bits: *mut c_int) -> c_int;
    pub fn deflatePrime(strm: *mut z_stream, bits: c_int, value: c_int) -> c_int;
    pub fn deflateSetHeader(strm: *mut z_stream, head: gz_headerp) -> c_int;
    pub fn inflateSetDictionary(strm: *mut z_stream,
                                dictionary: *const c_uchar,
                                dictLength: c_uint)
                                -> c_int;
    pub fn inflateGetDictionary(strm: *mut z_stream,
                                dictionary: *mut c_uchar,
                                dictLength: *mut c_uint)
                                -> c_int;
    pub fn inflateSync(strm: *mut z_stream) -> c_int;
    pub fn inflateCopy(dest: *mut z_stream, source: *mut z_stream) -> c_int;
    pub fn inflateReset(strm: *mut z_stream) -> c_int;
    pub fn inflateReset2(strm: *mut z_stream, windowBits: c_int) -> c_int;
    pub fn inflatePrime(strm: *mut z_stream, bits: c_int, value: c_int) -> c_int;
    pub fn inflateMark(strm: *mut z_stream) -> c_long;
    pub fn inflateGetHeader(strm: *mut z_stream, head: gz_headerp) -> c_int;
    pub fn inflateBack(strm: *mut z_stream,
                       _in: in_func,
                       in_desc: *mut c_void,
                       out: out_func,
                       out_desc: *mut c_void)
                       -> c_int;
    pub fn inflateBackEnd(strm: *mut z_stream) -> c_int;
    pub fn zlibCompileFlags() -> c_ulong;
    pub fn compress(dest: *mut c_uchar,
                    destLen: *mut size_t,
                    source: *const c_uchar,
                    sourceLen: size_t)
                    -> c_int;
    pub fn compress2(dest: *mut c_uchar,
                     destLen: *mut size_t,
                     source: *const c_uchar,
                     sourceLen: size_t,
                     level: c_int)
                     -> c_int;
    pub fn compressBound(sourceLen: size_t) -> size_t;
    pub fn uncompress(dest: *mut c_uchar,
                      destLen: *mut size_t,
                      source: *const c_uchar,
                      sourceLen: size_t)
                      -> c_int;
    pub fn gzdopen(fd: c_int, mode: *const c_char) -> gzFile;
    pub fn gzbuffer(file: gzFile, size: c_uint) -> c_int;
    pub fn gzsetparams(file: gzFile, level: c_int, strategy: c_int) -> c_int;
    pub fn gzread(file: gzFile, buf: *mut c_void, len: c_uint) -> c_int;
    pub fn gzwrite(file: gzFile, buf: *const c_void, len: c_uint) -> c_int;
    pub fn gzprintf(file: gzFile, format: *const c_char, ...) -> c_int;
    pub fn gzputs(file: gzFile, s: *const c_char) -> c_int;
    pub fn gzgets(file: gzFile, buf: *mut c_char, len: c_int) -> *mut c_char;
    pub fn gzputc(file: gzFile, c: c_int) -> c_int;
    pub fn gzgetc(file: gzFile) -> c_int;
    pub fn gzungetc(c: c_int, file: gzFile) -> c_int;
    pub fn gzflush(file: gzFile, flush: c_int) -> c_int;
    pub fn gzrewind(file: gzFile) -> c_int;
    pub fn gzeof(file: gzFile) -> c_int;
    pub fn gzdirect(file: gzFile) -> c_int;
    pub fn gzclose(file: gzFile) -> c_int;
    pub fn gzclose_r(file: gzFile) -> c_int;
    pub fn gzclose_w(file: gzFile) -> c_int;
    pub fn gzerror(file: gzFile, errnum: *mut c_int) -> *const c_char;
    pub fn gzclearerr(file: gzFile);
    pub fn adler32(adler: uint32_t, buf: *const c_uchar, len: uint32_t) -> uint32_t;
    pub fn crc32(crc: uint32_t, buf: *const c_uchar, len: off_t) -> uint32_t;
    pub fn deflateInit_(strm: *mut z_stream,
                        level: c_int,
                        version: *const c_char,
                        stream_size: c_int)
                        -> c_int;
    pub fn inflateInit_(strm: *mut z_stream, version: *const c_char, stream_size: c_int) -> c_int;
    pub fn deflateInit2_(strm: *mut z_stream,
                         level: c_int,
                         method: c_int,
                         windowBits: c_int,
                         memLevel: c_int,
                         strategy: c_int,
                         version: *const c_char,
                         stream_size: c_int)
                         -> c_int;
    pub fn inflateInit2_(strm: *mut z_stream,
                         windowBits: c_int,
                         version: *const c_char,
                         stream_size: c_int)
                         -> c_int;
    pub fn inflateBackInit_(strm: *mut z_stream,
                            windowBits: c_int,
                            window: *mut c_uchar,
                            version: *const c_char,
                            stream_size: c_int)
                            -> c_int;
    pub fn gzgetc_(file: gzFile) -> c_int;
    pub fn gzopen(arg1: *const c_char, arg2: *const c_char) -> gzFile;
    pub fn gzseek(arg1: gzFile, arg2: c_long, arg3: c_int) -> c_long;
    pub fn gztell(arg1: gzFile) -> c_long;
    pub fn gzoffset(arg1: gzFile) -> c_long;
    pub fn adler32_combine(arg1: uint32_t, arg2: uint32_t, arg3: c_long) -> uint32_t;
    pub fn crc32_combine(arg1: uint32_t, arg2: uint32_t, arg3: c_long) -> uint32_t;
}

