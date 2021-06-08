#![feature(raw_dylib)]

#[repr(C)]
struct S {
    x: u8,
    y: i32,
}

#[link(name = "extern", kind = "raw-dylib")]
extern "stdcall" {
    fn stdcall_fn_1(i: i32);
    fn stdcall_fn_2(c: u8, f: f32);
    fn stdcall_fn_3(d: f64);
    fn stdcall_fn_4(i: u8, j: u8, f: f32);
    fn stdcall_fn_5(a: S, b: i32);
    fn stdcall_fn_6(a: Option<&S>);
}

pub fn library_function() {
    let s = S { x: 1, y: 2 };
    unsafe {
        stdcall_fn_1(14);
        stdcall_fn_2(16, 3.5);
        stdcall_fn_3(3.5);
        stdcall_fn_4(1, 2, 3.0);
        stdcall_fn_5(s, 16);
        stdcall_fn_6(Some(&S { x: 10, y: 12 }));
    }
}
