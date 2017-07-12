The missing link to use rust from WSL (Linux Susbsystem for Windows) from IntelliJ (and potentially also other IDEs) to achieve a cross compiling environment on Windows without the hassle of setting up an actual cross compiler (because the latter is depending on C cross toolchains which are currently all outdated for Windows->Linux and generally not recommended for real usage).

# Installation

```
git clone git@github.com:philippkeller/rustup-toolchain-proxy.git
cd rustup-toolchain-proxy
./install.ps1
```

This 

- installs the `stable-x86_64-unknown-linux-gnu` toolchain (or updates if it is already installed)
- installs cargo.exe, rustc.exe and rustup.exe into `$HOME\.rustup\toolchains\stable-x86_64-unknown-linux-gnu\bin\`

# Usage

Set your project as linux-gnu, pointing to the 

```
cd my_rust_project
rustup rustup override set stable-x86_64-unknown-linux-gnu
```

Currently the following works in IntelliJ (tested on my laptop only, please file PRs/bug requests):

- first time loading a project in IntelliJ discovers the correct toolchain
- `refresh Cargo` makes IntelliJ understand the code structure and e.g. displays play icons next to `main()` or test functions
- the line numbers in compiler errors are clickable

What does not work yet:

- colors (probably needs some "tty" support or whatever windows calls this)
- environment variables which IntelliJ sets, e.g. RUST_BACKTRACE are not set within bash

# How it works

This takes into account the proxy characteristics of rustup itself. Basically that's the chain of commands:

- IntelliJ runs cargo.exe (or rustc.exe)
- rustup (which is a proxy really) checks active toolchain (`rustup show`) and runs cargo.exe lying in ` $HOME\.rustup\toolchains\stable-x86_64-unknown-linux-gnu\bin\`
- the `cargo.exe` in this toolchain is again a proxy which executes `bash.exe -c "cargo ..."` and outputs stdout/stderr, plus does translate the linux paths from e.g. "/home/linux_user/..." to: "C:\\Users\\windows_user\\AppData\\Local\lxss\\home\\linux_user\.."