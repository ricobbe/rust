#![feature(raw_dylib)]

mod a {
    #[link(name = "extern_1", kind = "raw-dylib")]
    extern {
        pub fn f();
    }
}

mod b {
    #[link(name = "extern_2", kind = "raw-dylib")]
    extern {
        pub fn f();
    }
}

pub fn library_function() {
    unsafe {
        a::f();
        b::f();
    }
}
