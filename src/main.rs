extern crate clap;

use clap::{Arg, App};
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {

    // app arguments
    let args: Vec<Arg> = vec![

        // --in
        Arg::with_name("in")
            .short("i")
            .long("in")
            .value_name("Input command")
            .help("The input command. Its output will be redirected to the --out command's stdin.")
            .required(true)
            .takes_value(true),

        // --out
        Arg::with_name("out")
            .short("o")
            .long("out")
            .value_name("Output command")
            .help("The command that will execute and read the redirected output.")
            .required(true)
            .takes_value(true)
    ];

    // clap App object
    let app: App = App::new("myPipe")
        .version("1.0")
        .author("N'zaou Renaud (nzaou.renaud@live.fr)")
        .about("Pipe --in command to --out command (no argumentss support)")
        .args(&args);

    // parse command line arguments
    let matches = app
        .get_matches_safe()
        .unwrap_or_else(|e| e.exit());

    let in_command = matches
        .value_of("in")
        .unwrap();

    let out_command = matches
        .value_of("out")
        .unwrap();

    // get output from first process.
    let in_process = Command::new(in_command)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .expect("Failed to get IN process output");

    // spawn second process
    let mut out_process = Command::new(out_command)
        .stdin(Stdio::piped())
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to spawn OUT process");

    // write first process output into second process's standard input.
    let stdin = out_process.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all(&in_process.stdout).expect("Failed to write to stdin");
    
    stdin.flush().expect("Failed to flush stdin");

    out_process.wait().expect("Failed to wait for process");
}

