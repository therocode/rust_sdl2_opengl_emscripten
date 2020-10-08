extern crate gl_generator;

use gl_generator::{Api, Fallbacks, Profile, Registry, StructGenerator};
use std::fs::File;
use std::path::Path;
use std::{env, path::PathBuf};

fn main() {
    // GL Generator
    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("bindings.rs")).unwrap();

    Registry::new(Api::Gl, (4, 3), Profile::Core, Fallbacks::All, [])
        .write_bindings(StructGenerator, &mut file)
        .unwrap();

    // Sort out SDL2 linking for windows
    //
    // For linking to work, SDL2 development libraries for windows must be downloaded from
    // https://libsdl.org/download-2.0.php and placed in the 'gnu-mingw' and 'msvc'
    // directories as follows:
    //
    // mingw:
    // i686-w64-mingw32/bin/* -> gnu-mingw/dll/32/bin/
    // i686-w64-mingw32/lib/* -> gnu-mingw/dll/32/lib/
    // x86_64-w64-mingw32/bin/* -> gnu-mingw/dll/64/bin/
    // x86_64-w64-mingw32/lib/* -> gnu-mingw/dll/64/lib/
    //
    // VC:
    // lib/x86/SDL2.dll -> msvc/dll/32/SDL2.dll
    // lib/x86/*.lib -> msvc/lib/32/
    // lib/x64/SDL2.dll -> msvc/dll/64/SDL2.dll
    // lib/x64/*.lib -> msvc/lib/64/
    //
    // An alternative to this is to set sdl2's 'features' attribute in Cargo.toml to ["bundled"]
    // but that does not work with wasm32-unknown-emscripten as of now

    let target = env::var("TARGET").unwrap();
    if target.contains("pc-windows") {
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let mut lib_dir = manifest_dir.clone();
        let mut dll_dir = manifest_dir.clone();
        if target.contains("msvc") {
            lib_dir.push("msvc");
            dll_dir.push("msvc");
        } else {
            lib_dir.push("gnu-mingw");
            dll_dir.push("gnu-mingw");
        }
        lib_dir.push("lib");
        dll_dir.push("dll");
        if target.contains("x86_64") {
            lib_dir.push("64");
            dll_dir.push("64");
        } else {
            lib_dir.push("32");
            dll_dir.push("32");
        }
        println!("cargo:rustc-link-search=all={}", lib_dir.display());
        for entry in std::fs::read_dir(dll_dir).expect("Can't read DLL dir") {
            let entry_path = entry.expect("Invalid fs entry").path();
            let file_name_result = entry_path.file_name();
            let mut new_file_path = manifest_dir.clone();
            if let Some(file_name) = file_name_result {
                let file_name = file_name.to_str().unwrap();
                if file_name.ends_with(".dll") {
                    new_file_path.push(file_name);
                    std::fs::copy(&entry_path, new_file_path.as_path())
                        .expect("Can't copy from DLL dir");
                }
            }
        }
    }
}
