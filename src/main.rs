#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(main, register_tool)]
extern "C" {
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn putchar(__c: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn cos(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
}
#[no_mangle]
pub static mut k: libc::c_int = 0;
unsafe fn main_0() -> libc::c_int {
    let mut A: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut B: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut i: libc::c_float = 0.;
    let mut j: libc::c_float = 0.;
    let mut z: [libc::c_float; 1760] = [0.; 1760];
    let mut b: [libc::c_char; 1760] = [0; 1760];
    printf(b"\x1b[2J\x00" as *const u8 as *const libc::c_char);
    loop  {
        memset(b.as_mut_ptr() as *mut libc::c_void, 32 as libc::c_int,
               1760 as libc::c_int as libc::c_ulong);
        memset(z.as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int,
               7040 as libc::c_int as libc::c_ulong);
        j = 0 as libc::c_int as libc::c_float;
        while 6.28f64 > j as libc::c_double {
            i = 0 as libc::c_int as libc::c_float;
            while 6.28f64 > i as libc::c_double {
                let mut sini: libc::c_float =
                    sin(i as libc::c_double) as libc::c_float;
                let mut cosj: libc::c_float =
                    cos(j as libc::c_double) as libc::c_float;
                let mut sinA: libc::c_float =
                    sin(A as libc::c_double) as libc::c_float;
                let mut sinj: libc::c_float =
                    sin(j as libc::c_double) as libc::c_float;
                let mut cosA: libc::c_float =
                    cos(A as libc::c_double) as libc::c_float;
                let mut cosj2: libc::c_float =
                    cosj + 2 as libc::c_int as libc::c_float;
                let mut mess: libc::c_float =
                    1 as libc::c_int as libc::c_float /
                        (sini * cosj2 * sinA + sinj * cosA +
                             5 as libc::c_int as libc::c_float);
                let mut cosi: libc::c_float =
                    cos(i as libc::c_double) as libc::c_float;
                let mut cosB: libc::c_float =
                    cos(B as libc::c_double) as libc::c_float;
                let mut sinB: libc::c_float =
                    sin(B as libc::c_double) as libc::c_float;
                let mut t: libc::c_float = sini * cosj2 * cosA - sinj * sinA;
                let mut x: libc::c_int =
                    (40 as libc::c_int as libc::c_float +
                         30 as libc::c_int as libc::c_float * mess *
                             (cosi * cosj2 * cosB - t * sinB)) as libc::c_int;
                let mut y: libc::c_int =
                    (12 as libc::c_int as libc::c_float +
                         15 as libc::c_int as libc::c_float * mess *
                             (cosi * cosj2 * sinB + t * cosB)) as libc::c_int;
                let mut o: libc::c_int = x + 80 as libc::c_int * y;
                let mut N: libc::c_int =
                    (8 as libc::c_int as libc::c_float *
                         ((sinj * sinA - sini * cosj * cosA) * cosB -
                              sini * cosj * sinA - sinj * cosA -
                              cosi * cosj * sinB)) as libc::c_int;
                if 22 as libc::c_int > y && y > 0 as libc::c_int &&
                       x > 0 as libc::c_int && 80 as libc::c_int > x &&
                       mess > z[o as usize] {
                    z[o as usize] = mess;
                    b[o as usize] =
                        (*::std::mem::transmute::<&[u8; 13],
                                                  &[libc::c_char; 13]>(b".,-~:;=!*#$@\x00"))[if N
                                                                                                    >
                                                                                                    0
                                                                                                        as
                                                                                                        libc::c_int
                                                                                                {
                                                                                                 N
                                                                                             } else {
                                                                                                 0
                                                                                                     as
                                                                                                     libc::c_int
                                                                                             }
                                                                                                 as
                                                                                                 usize]
                }
                i = (i as libc::c_double + 0.02f64) as libc::c_float
            }
            j = (j as libc::c_double + 0.07f64) as libc::c_float
        }
        printf(b"\x1b[d\x00" as *const u8 as *const libc::c_char);
        k = 0 as libc::c_int;
        while 1761 as libc::c_int > k {
            putchar(if k % 80 as libc::c_int != 0 {
                        b[k as usize] as libc::c_int
                    } else { 10 as libc::c_int });
            k += 1
        }
        A = (A as libc::c_double + 0.04f64) as libc::c_float;
        B = (B as libc::c_double + 0.02f64) as libc::c_float;
        sleep(0.2f64 as libc::c_uint);
    };
}
#[main]
pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }
