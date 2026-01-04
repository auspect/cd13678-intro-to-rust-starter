fn main() {
    cc::Build::new()
        .file("../opengl_wrapper_lib/opengl_wrapper_lib.c").
        include("../opengl_wrapper_lib")
        .compile("openglwrapper");
    println!("cargo:rustc-link-search=native=../c_output");
    println!("cargo:rustc-link-lib=openglwrapper");
    println!("cargo:rustc-link-arg=-Wl,-rpath,../c_output");
    println!("cargo:rustc-link-lib=dylib=glfw");
    println!("cargo:rustc-link-lib=dylib=GL");
}
