# Learn RUST on Github
Rust is a modern systems programming language developed by the Mozilla Corporation. It is intended to be a language for highly concurrent and highly secure systems. It compiles to native code; hence, it is blazingly fast like C and C++. This tutorial adopts a simple and practical approach to describe the concepts of Rust programming.

**Setup RUST environment for windows**

* install [rust-init.exe](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)
* rustup setup
    * rustup install nightly-msvc
    * rustup default nightly-msvc
    * rustup self update
    * rustup update nightly //or stable
* rustup self uninstall

**Hello-world**
* This is very simple rust project.
    * cargo new hello-world
    * cargo build
    * cargo run
    
**Module Std:io**
    Because they are traits, Read and Write are implemented by a number of other types, and you can implement them for your types too. As such, you’ll see a few different types of I/O throughout the documentation in this module: Files, TcpStreams, and sometimes even Vec<T>s.