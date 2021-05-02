# mysql-client-bindings-rs

A more complete autogenerated Rust bindings for mysql/mariadb client library `#include <mysql.h>`

## Navigation

<ul>
    <li><a href="#limitations" > <span style="font-size: 16pt"> Limitations </span> </a> </li> 
    <li> <a href="#build" > <span style="font-size: 16pt"> Build </span> </a> </li> 
    <li> <a href="#usage" > <span style="font-size: 16pt"> Usage </span> </a> </li> 
    <li> 
        <a href="#differences" ><span style="font-size: 16pt"> Differences </span> </a> 
        <span style="font-size: 16pt"> from </span> 
        <span style="font-size: 16pt"> <a href="https://github.com/sgrif/mysqlclient-sys"> mysqlclient-sys </a></span>
    </li> 
    <li> <a href="#license" > <span style="font-size: 16pt"> Build </span> </a> </li> 

</ul>

## Limitations

Works and tested only on UNIX systems.

Tested on Ubuntu 20.04 (and corresponding docker image), `mysql:8.0.24` and `mariadb:10` docker images.

Any further contributions are welcome in order to

- Test build on other UNIX systems and extend installation instructions and list of tested systems.
- Add support of **Windows**/**MacOS**, but any contributions **should not** break UNIX support.

## Build

For build, you need to install corresponding client libraries,  
depends on needed features (default path if you choose this crate as a dependency at your `Cargo.toml`), 
or you need only docker (path `#2`) 
if you want to build is locally by some reason or in order to reproduce these bindings.


### #1 Build on host system

In order to generate bindings on host you need to:

1. Install bindgen dependencies
   - `sudo apt-get update && sudo apt-get install llvm-dev libclang-dev clang`
2. Install needed client libraries
   - `mysql` - `sudo apt-get install libmysqlclient-dev`
   - `mariadb` - `sudo apt-get install libmariadb3 libmariadb-dev`

Then, if you want to just use this crate as a dependency, put in to your `Cargo.toml` and select `mysql` or `mariadb`
as a feature.

If you want to create standalone bindings:
   - `mysql` - `cargo build --release` (`mysql` is default feature)
   - `mariadb` - `cargo build --release --no-default-features --features mariadb`

### #2 Docker

In order to generate standalone bindings through docker you need to:

1. Install docker
    - Ubuntu: `sudo apt-get update && sudo apt-get install docker.io`
2. Adjust docker settings

   2.1 Add your user in `docker` group in order to use docker as non-root user - `sudo usermod -aG docker $USER`

   2.2 If you want to continue using docker, add docker to systemctl in order to start it on the boot
   - `sudo systemctl enable docker`

3. Run build script - it will rebuild both kind of bindings for you and store them at `./src` directory.
    - `bash ./build.sh`


## Usage 

Ensure that your `Cargo.toml` contains dependency with needed feature, and you have installed needed libraries, 
listed at section <a href="#build" > <span style="font-size: 12pt"> Build</span></a>:
```toml
# ...

[dependencies]
mysql-client-bindings-rs = { version = "0.1.2" }
```

Then add `use`,  it would be wise to use a shorter alias:
```rust
use mysql_client_bindings_rs as mysql; // alias
use mysql::{UDF_INIT, UDF_ARGS};  // whatever you need
```

## Differences from [mysqlclient-sys](https://github.com/sgrif/mysqlclient-sys)

<ul style="list-style-type: none">
  <li> <b>+</b> More complete bindings, including more types, definitions, structs, and UDF support:
  ~6500 LoC with <b>mysql</b> and ~9700 LoC (~8500 with line_width=120) with <b>mariadb</b> 
  against only mysql ~2700 LoC for <b>Windows</b> and ~1400 LoC for <b>MacOS (identical with UNIX)</b> </li>

  <li> <b>-</b> Works&tested only on UNIX systems.</li>
</ul>

---
## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.