// zpipe.c: example of proper use of zlib's inflate() and deflate()
// Not copyrighted -- provided to the public domain
// Version 1.4  11 December 2005  Mark Adler
// Rust port: 2 February 2016  Patrick Walton

extern crate libc;
extern crate zlib_ng_sys;

use libc::{FILE, c_int, c_void};
use std::ffi::CString;
use std::process;
use std::ptr;
use zlib_ng_sys::{Z_DATA_ERROR, Z_ERRNO, Z_OK, Z_MEM_ERROR, Z_NEED_DICT, Z_NO_FLUSH, Z_STREAM_END};
use zlib_ng_sys::{Z_STREAM_ERROR, z_stream};

const CHUNK: usize = 16384;

unsafe fn def(source: *mut FILE, dest: *mut FILE) -> c_int {
    let mut have;
    let mut strm = z_stream::default();
    let mut input = [0; CHUNK];
    let mut out = [0; CHUNK];

    /* allocate inflate state */
    strm.zalloc = None;
    strm.zfree = None;
    strm.opaque = ptr::null_mut();
    strm.avail_in = 0;
    strm.next_in = ptr::null();

    let ret = zlib_ng_sys::inflateInit(&mut strm);
    if ret != Z_OK {
        return ret
    }

    /* decompress until deflate stream ends or end of file */
    loop {
        strm.avail_in = libc::fread(&mut input[0] as *mut _ as *mut c_void,
                                    1,
                                    CHUNK,
                                    source) as u32;
        if libc::ferror(source) != 0 {
            zlib_ng_sys::inflateEnd(&mut strm);
            return Z_ERRNO
        }
        if strm.avail_in == 0 {
            break
        }
        strm.next_in = &input[0];

        /* run inflate() on input until output buffer not full */
        loop {
            strm.avail_out = CHUNK as u32;
            strm.next_out = &mut out[0];
            let ret = zlib_ng_sys::inflate(&mut strm, Z_NO_FLUSH);
            assert!(ret != Z_STREAM_ERROR);  /* state not clobbered */
            match ret {
                Z_NEED_DICT => {
                    zlib_ng_sys::inflateEnd(&mut strm);
                    return Z_DATA_ERROR
                }
                Z_DATA_ERROR | Z_MEM_ERROR => {
                    zlib_ng_sys::inflateEnd(&mut strm);
                    return ret
                }
                _ => {}
            }

            have = (CHUNK as u32) - strm.avail_out;
            if libc::fwrite(&out[0] as *const u8 as *const c_void,
                            1,
                            have as usize,
                            dest) != (have as usize) || libc::ferror(dest) != 0 {
                zlib_ng_sys::inflateEnd(&mut strm);
                return Z_ERRNO
            }

            if strm.avail_out != 0 {
                break
            }
        }

        /* done when inflate() says it's done */
        if ret == Z_STREAM_END {
            break
        }
    }

    /* clean up and return */
    zlib_ng_sys::inflateEnd(&mut strm);
    if ret == Z_STREAM_END {
        Z_OK
    } else {
        Z_DATA_ERROR
    }
}

fn main() {
    unsafe {
        let r = CString::new("r").unwrap();
        let w = CString::new("w").unwrap();
        process::exit(def(libc::fdopen(0, r.as_ptr()), libc::fdopen(1, w.as_ptr())))
    }
}

