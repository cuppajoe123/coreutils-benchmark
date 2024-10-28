pub const EXIT_SUCCESS: u32 = 0;

extern "C" {
    pub fn usage(status: ::std::os::raw::c_int);
}

extern "C" {
    pub fn set_program_name(argv0: *const ::std::os::raw::c_char);
}

extern "C" {
    pub fn setlocale(
        __category: ::std::os::raw::c_int,
        __locale: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}

extern "C" {
    pub fn bindtextdomain(
        __domainname: *const ::std::os::raw::c_char,
        __dirname: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}

extern "C" {
    pub fn textdomain(__domainname: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}

extern "C" {
    pub fn atexit(__func: ::std::option::Option<unsafe extern "C" fn()>) -> ::std::os::raw::c_int;
}

extern "C" {
    pub fn version_etc(
        stream: *mut FILE,
        command_name: *const ::std::os::raw::c_char,
        package: *const ::std::os::raw::c_char,
        version: *const ::std::os::raw::c_char,
        ...
    );
}
