#![allow(non_camel_case_types, non_snake_case, improper_ctypes, non_upper_case_globals)]

#[cfg(all(feature = "mysql", not(feature = "mariadb")))]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/mysql.rs"));

#[cfg(all(feature = "mariadb", not(feature = "mysql")))]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/mariadb.rs"));
