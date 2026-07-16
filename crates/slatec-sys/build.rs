use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=SLATEC_NATIVE_LIB_DIR");
    println!("cargo:rerun-if-env-changed=SLATEC_GFORTRAN_RUNTIME_DIR");
    if let Some(native_dir) = env::var_os("SLATEC_NATIVE_LIB_DIR") {
        if env::var_os("CARGO_FEATURE_FFI_PROFILE_GNU_MINGW_X86_64").is_none() {
            panic!("SLATEC_NATIVE_LIB_DIR requires the ffi-profile-gnu-mingw-x86_64 feature");
        }
        let target = env::var("TARGET").unwrap_or_default();
        if target != "x86_64-pc-windows-gnu" {
            panic!(
                "the GNU MinGW SLATEC profile may link only for x86_64-pc-windows-gnu, found {target}"
            );
        }
        println!(
            "cargo:rustc-link-search=native={}",
            native_dir.to_string_lossy()
        );
        println!("cargo:rustc-link-lib=static=slatec_selected");
        if let Some(runtime_dir) = env::var_os("SLATEC_GFORTRAN_RUNTIME_DIR") {
            println!(
                "cargo:rustc-link-search=native={}",
                runtime_dir.to_string_lossy()
            );
        }
        println!("cargo:rustc-link-lib=dylib=gfortran");
        println!("cargo:rustc-link-lib=dylib=quadmath");
    }
}
