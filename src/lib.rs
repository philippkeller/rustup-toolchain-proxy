use std::env;
use std::process;
use std::thread;
use std::io::{Read, Write};
use std::{io, str};

extern crate regex;

use regex::{Regex, Captures};

fn pass_through<T: Read, U:Write>(mut reader:T, mut writer:U) {
    let mut buffer = [0u8; 128];
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => {
                // eof
                break
            }
            Ok(n) if n > 0 => {
                // linux has \n as line ending, windows \r\n -> needs some copy/replace action
                let s:String = str::from_utf8(&buffer[0..n]).expect("utf-8 error").to_string();
                writer.write(s.replace("\n", "\r\n").as_bytes()).expect("cannot write line");
            },
            _ => {}
        }
    }
}

// TODO: there's an error "cannot find stdlib in sysroot" which is probably caused by
// ` rustc --print sysroot` printing a linux path, currently
// `/home/philipp/.rustup/toolchains/stable-x86_64-unknown-linux-gnu`

fn replace_paths<T: Read, U:Write>(mut reader:T, mut writer:U) {
    let mut buffer = [0u8; 128];
    let mut res = String::with_capacity(8192);
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => {
                // eof
                break
            }
            Ok(n) if n > 0 => {
                // linux has \n as line ending, windows \r\n -> needs some copy/replace action
                let s:&str = str::from_utf8(&buffer[0..n]).expect("utf-8 error");
                res.push_str(&s);
            },
            _ => {}
        }
    }
    // TODO: find out username from environment
    // TODO: doesn't that need to point to the WSL path instead?
    // from: "/home/philipp/..."
    // to:   "C:\\Users\\lcl40026\\.."
    let re = Regex::new("\"/home/[^/]+/.cargo/([^\"]+)\"").unwrap();
    res = re.replace_all(&res, |caps: &Captures| {
        let path = "\"C:/Users/lcl40026/AppData/Local/lxss/home/philipp/.cargo/".to_string() + &caps[1] + "\"";
        path.replace("/", "\\\\")
    }).to_string();
    // from: "/mnt/c/..."
    // to:   "C:\\.."
    let re = Regex::new("\"/mnt/([^\"]+)\"").unwrap();
    res = re.replace_all(&res, |caps: &Captures| {
        format!("\"{}:\\\\{}\"", caps[1][0..1].to_uppercase(), &caps[1][1..].replace("/", "\\\\"))
    }).to_string();
    // from: path+file:///mnt/c/ProgramData/oss/rexpect)
    // to:   path+file:///C:/ProgramData..
    let re = Regex::new("path\\+file:///mnt/([^\"]+)").unwrap();
    res = re.replace_all(&res, |caps: &Captures| {
        format!("path+file:///{}:{}", caps[1][0..1].to_uppercase(), &caps[1][1..])
    }).to_string();


    writer.write_all(res.as_bytes()).expect("cannot write line");
}

pub fn proxy(command: &str) {
    let mut arguments:Vec<String> = env::args().collect::<Vec<String>>();
    arguments[0] = "~/.cargo/bin/".to_string() + command;
    let args = arguments.join(" ");

    let stdout_fn = if command == "cargo" && arguments.len() >= 2 && arguments[1] == "metadata" {
        replace_paths
    } else {
        pass_through
    };
    let mut process = process::Command::new("bash.exe").arg("-ci").arg(&args)
        .stdout(process::Stdio::piped()).stderr(process::Stdio::piped()).spawn().expect(&format!("could not execute {} {}", &command, &args));
    let stdout = process.stdout.take().unwrap();
    let stderr = process.stderr.take().unwrap();
    let t_out = thread::spawn(move || {
        stdout_fn(stdout, io::stdout());
    });
    let t_err = thread::spawn(move || {
        pass_through(stderr, io::stderr());
    });

    let exit = process.wait().expect("could not wait until finished..");
    t_out.join().expect("could not join thread reading/writing from stdout");
    t_err.join().expect("could not join thread reading/writing from stderr");
    process::exit(exit.code().expect("no exit code"));
}