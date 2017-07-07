use std::env;
use std::process;
use std::thread;
use std::io::{Read, Write};
use std::{io, str};

fn pass_through<T: Read, U:Write>(mut reader:T, mut writer:U) {
    let mut buffer = [0u8; 256];
    loop {
        match reader.read(&mut buffer) {
            Ok(n) if n > 0 => {
                // linux has \n as line ending, windows \r\n -> needs some copy/replace action
                let s:String = str::from_utf8(&buffer[0..n]).expect("utf-8 error").to_string();
                writer.write(s.replace("\n", "\r\n").as_bytes()).expect("cannot write line");
            }
            _ => {}
        }
    }
}

pub fn proxy(command: &str) {
    let mut arguments:Vec<String> = env::args().collect::<Vec<String>>();
    arguments[0] = "~/.cargo/bin/".to_string() + command;
    let args = arguments.join(" ");
    let mut process = process::Command::new("bash.exe").arg("-ci").arg(&args)
        .stdout(process::Stdio::piped()).stderr(process::Stdio::piped()).spawn().expect(&format!("could not execute {} {}", &command, &args));
    let stdout = process.stdout.take().unwrap();
    let stderr = process.stderr.take().unwrap();
    thread::spawn(move || {
        pass_through(stdout, io::stdout());
    });
    thread::spawn(move || {
        pass_through(stderr, io::stderr());
    });

    let exit = process.wait().expect("could not wait until finished..");
    process::exit(exit.code().expect("no exit code"));
}