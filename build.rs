#[cfg(all(feature = "fresh_bindings", feature = "mysql", not(feature = "mariadb")))]
mod constants {
    pub(crate) const FILE_NAME: &str = "src/mysql.rs";
    pub(crate) const BINDGEN_EXTRA_CLANG_ARGS: &str = r#"-I"/usr/include/mysql""#;
}

#[cfg(all(feature = "fresh_bindings", feature = "mariadb", not(feature = "mysql")))]
mod constants {
    pub(crate) const FILE_NAME: &str = "src/mariadb.rs";
    pub(crate) const BINDGEN_EXTRA_CLANG_ARGS: &str = r#"-I"/usr/include/mariadb""#;
}

fn main() {
    #[cfg(all(feature = "fresh_bindings", feature = "bindgen"))]
    build();
}

#[cfg(any(
    all(not(feature = "fresh_bindings"), feature = "bindgen"),
    all(feature = "fresh_bindings", not(feature = "bindgen"))
))]
compile_error!("Features `fresh_bindings` and `bindgen` must be enabled ONLY together!");

#[cfg(all(feature = "fresh_bindings", feature = "bindgen"))]
fn build() {
    use std::env;
    use std::path::PathBuf;

    // Tell cargo to tell rustc to link the mysql or mariadb shared library.
    println!("{}", format!("cargo:rustc-link-lib={}", "mysqlclient"));

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    println!("cargo:rerun-if-changed=build.rs");

    std::env::set_var("BINDGEN_EXTRA_CLANG_ARGS", constants::BINDGEN_EXTRA_CLANG_ARGS);

    // The bindgen::Builder is the main entry point to bindgen,
    // and lets you build up options for the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // reformat bindings with rustfmt
        .rustfmt_bindings(true)
        .rustfmt_configuration_file(Some("./rustfmt.toml".into()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    bindings
        .write_to_file(out_path.join(constants::FILE_NAME))
        .expect("Couldn't write bindings!");
}

// It might be helpful in future
// fn mysql_config() -> Option<HashMap<String, std::collections::VecDeque<String>>> {
//     use std::collections::{VecDeque, HashMap};
//
//     if let Ok(output) = std::process::Command::new("mysql_config").output() {
//         let hm = String::from_utf8(output.stdout).unwrap()
//             .split('\n')
//             .map(|s| s.to_string())
//             .filter(|x| x.contains("--") && !x.contains("--variable"))
//             .map(|s| s.trim_start().to_string())
//             .map(|s| s.replace(" ]", "]").replace("]", "").replace("[", ""))
//             .map(|s| s.split_whitespace().map(|x| x.to_string()).collect::<VecDeque<String>>())
//             .map(|mut v| {let key = v.pop_front().unwrap(); let value = v; (key, value)})
//             .fold(HashMap::new(), |mut acc, (k, v)| { acc.insert(k ,v); acc});
//         Some(hm)
//     } else {
//         None
//     }
// }
