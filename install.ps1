Push-Location $PSScriptRoot # cd into current dir
echo "installing rustup toolchain for linux-gnu"
rustup toolchain install stable-x86_64-unknown-linux-gnu
cargo build --release
cp .\target\release\*.exe $HOME\.rustup\toolchains\stable-x86_64-unknown-linux-gnu\bin\
Pop-Location