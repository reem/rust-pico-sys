#![feature(test)]

extern crate pico_sys as pico;

extern crate test;

// From picohttpparser/bench.c#L31
const REQ: &'static [u8] = b"\
GET /wp-content/uploads/2010/03/hello-kitty-darth-vader-pink.jpg HTTP/1.1\r\n\
Host: www.kittyhell.com\r\n\
User-Agent: Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10.6; ja-JP-mac; rv:1.9.2.3) Gecko/20100401 Firefox/3.6.3 Pathtraq/0.9\r\n\
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\r\n\
Accept-Language: ja,en-us;q=0.7,en;q=0.3\r\n\
Accept-Encoding: gzip,deflate\r\n\
Accept-Charset: Shift_JIS,utf-8;q=0.7,*;q=0.7\r\n\
Keep-Alive: 115\r\n\
Connection: keep-alive\r\n\
Cookie: wp_ozh_wsa_visits=2; wp_ozh_wsa_visit_lasttime=xxxxxxxxxx; __utma=xxxxxxxxx.xxxxxxxxxx.xxxxxxxxxx.xxxxxxxxxx.xxxxxxxxxx.x; __utmz=xxxxxxxxx.xxxxxxxxxx.x.x.utmccn=(referral)|utmcsr=reader.livedoor.com|utmcct=/reader/|utmcmd=referral\r\n\r\n";

#[bench]
fn parse(b: &mut test::Bencher) {
    use std::mem;

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Header<'a>(&'a [u8], &'a [u8]);

    #[repr(C)]
    struct Headers<'a>(&'a mut [Header<'a>]);
    let method = [0i8; 16];
    let path = [0i8; 16];
    let mut minor_version = 0;
    let mut h = [Header(&[], &[]); 16];
    let mut h_len = h.len();
    let headers = Headers(&mut h);
    let prev_buf_len = 0;

    b.iter(|| {
        let ret = unsafe {
            pico::ffi::phr_parse_request(
                REQ.as_ptr() as *const _,
                REQ.len(),
                &mut method.as_ptr(),
                &mut 16,
                &mut path.as_ptr(),
                &mut 16,
                &mut minor_version,
                mem::transmute::<*mut Header, *mut pico::ffi::phr_header>(headers.0.as_mut_ptr()),
                &mut h_len as *mut usize as *mut _,
                prev_buf_len
            )
        };
        assert_eq!(ret, REQ.len() as i32);
    });
    b.bytes = REQ.len() as u64;
}
