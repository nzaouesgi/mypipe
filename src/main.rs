extern crate clap;

use clap::{Arg, App};
use std::process::{Command, Stdio};

fn main() {

    // app arguments
    let args: Vec<Arg> = vec![

        // --in
        Arg::with_name("in")
            .short("i")
            .long("in")
            .value_name("Input command")
            .help("The input command.")
            .required(true)
            .takes_value(true),

        // --out
        Arg::with_name("out")
            .short("o")
            .long("out")
            .value_name("Output command")
            .help("The output command. Takes --out command stdout as stdin.")
            .required(true)
            .takes_value(true)
    ];

    // clap App object
    let app: App = App::new("myPipe")
        .version("1.0")
        .author("N'zaou Renaud (nzaou.renaud@live.fr)")
        .about("Pipe --in command to --out command (no arguments support)")
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

    // spawn first process	
    let in_process = Command::new(in_command)
	.stdin(Stdio::null())
        .stdout(Stdio::piped())
	.stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to spawn IN process.");

    // spawn second process
    // stdout and stderr are inherited explicitly from main parent process
    let mut out_process = Command::new(out_command)
        .stdin(in_process.stdout.unwrap())
	.stdout(Stdio::inherit())
	.stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to get OUT process's output.");

    // wait until output process terminates
    out_process.wait().expect("Failed to get process's status.");
}

