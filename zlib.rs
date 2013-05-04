//
// zlib bindings for rust (0.6)
//
// based on https://github.com/thestinger/rust-snappy
//
// Author: S Moon <ssamoon@ucla.edu>
// Date:   2013-05-03
//

#[link(name = "zlib", vers = "0.1.0")];

use core::libc::{c_char, c_int};
use core::vec;

static BUF_SIZE:u64      =  4096;
static Z_OK:c_int        =     0;
static Z_BUF_ERROR:c_int =    -5;

extern mod z {
    fn compressBound(srclen: u64) -> u64;
    fn compress(dest: *mut u8, destlen: *mut u64, src: *u8, srclen: u64) -> c_int;
    fn uncompress(dest: *mut u8, destlen: *mut u64, src: *u8, srclen: u64) -> c_int;
    fn zlibVersion() -> *c_char;
}

pub fn zlib_version() -> ~str {
    unsafe {
        let s = z::zlibVersion();
        str::raw::from_c_str(s)
    }
}

pub fn compress(src: &[u8]) -> ~[u8] {
    unsafe {
        let mut len     = vec::len(src) as u64;
        let psrc        = vec::raw::to_ptr(src);
        let mut destlen = z::compressBound(len);
        let mut dest    = vec::with_capacity(destlen as uint);
        let pdest       = vec::raw::to_mut_ptr(dest);

        z::compress(pdest, &mut destlen, psrc, len);

        vec::raw::set_len(&mut dest, destlen as uint);
        dest
    }
}

fn _uncompress(src: &[u8], bufsize: u64) -> Option<~[u8]> {
    unsafe {
        let mut len     = vec::len(src) as u64;
        let mut destlen = bufsize;
        let mut dest    = vec::with_capacity(destlen as uint);
        let pdest       = vec::raw::to_mut_ptr(dest);
        let psrc        = vec::raw::to_ptr::<u8>(src);

        let r = z::uncompress(pdest, &mut destlen, psrc, len);

        if r == Z_OK {
            vec::raw::set_len(&mut dest, destlen as uint);
            Some(dest)
        } else if r == Z_BUF_ERROR {
            // try again with a larger buffer
            // TODO: append with previously read vector
            _uncompress(src, bufsize * 2)
        } else {
            None
        }
    }
}

pub fn uncompress(src: &[u8]) -> Option<~[u8]> {
    _uncompress(src, BUF_SIZE)
}

#[test]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        let d = ~[0xde, 0xad, 0xd0, 0x0d];
        let c = compress(d);
        assert!(uncompress(c) == Some(d));
    }

    #[test]
    fn invalid() {
        let d = ~[0, 0, 0, 0];
        assert!(uncompress(d).is_none());
    }

    #[test]
    fn empty() {
        let d: ~[u8] = ~[];
        assert!(uncompress(d).is_none());
        let c = compress(d);
        assert!(uncompress(c) == Some(d));
    }
}
