#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]

//! # pico-sys
//!
//! Bindings to the PicoHTTPParser.
//!

extern crate libc;

/// Contains the ffi bindings to pico http parser.
pub mod ffi {
    use libc::{c_char, c_int, size_t, ssize_t};

    /// A header, pointing into an existing buffer.
    #[repr(C)]
    pub struct phr_header {
        name: *const c_char,
        name_len: size_t,
        value: *const c_char,
        value_len: size_t
    }

    /// A decoder for decoding chunked encoded data.
    #[repr(C)]
    pub struct phr_chunked_decoder {
        bytes_left_in_chunk: size_t,
        consume_trailer: c_char,
        hex_count: c_char,
        state: c_char,
    }

    extern {
        /// Parse a request, bit by bit.
        pub fn phr_parse_request(
            buf_start: *const c_char,
            buf_len: size_t,
            method: *mut *const c_char,
            method_len: *mut size_t,
            path: *mut *const c_char,
            path_len: *mut size_t,
            minor_version: *mut c_int,
            headers: *mut phr_header,
            num_headers: *mut size_t,
            prev_buf_len: size_t
        ) -> c_int;

        /// Parse a response, bit by bit.
        pub fn phr_parse_response(
            buf_start: *const c_char,
            buf_len: size_t,
            minor_version: *mut c_int,
            status: *mut c_int,
            message: *mut *const c_char,
            message_len: *mut size_t,
            headers: *mut phr_header,
            num_headers: *mut size_t,
            prev_buf_len: size_t
        ) -> c_int;

        /// Parse the headers, bit by bit.
        pub fn phr_parse_headers(
            buf_start: *const c_char,
            buf_len: size_t,
            headers: *mut phr_header,
            num_headers: *mut size_t,
            prev_buf_len: size_t
        ) -> c_int;

        /// Decode a chunked reqest body, bit by bit.
        pub fn phr_decode_chunked(
            decoder: *mut phr_chunked_decoder,
            buf: *mut c_char,
            buf_len: *mut size_t
        ) -> ssize_t;
    }
}

