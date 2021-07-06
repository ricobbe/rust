// only-windows-msvc
// compile-flags: --crate-type lib
#![feature(raw_dylib)]
#![allow(clashing_extern_declarations, incomplete_features)]
#[link(name = "foo", kind = "raw-dylib")]
extern "C" {
    fn f(x: i32);
}

pub fn lib_main() {
    #[link(name = "foo", kind = "raw-dylib")]
    extern "stdcall" {
        fn f(x: i32);
    }

    unsafe { f(42); }
}
