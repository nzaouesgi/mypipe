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

    // get output from first process.
    let in_process = Command::new(in_command)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn IN process.");

    // spawn second process
    let out_process = Command::new(out_command)
        .stdin(in_process.stdout.unwrap())
        .output()
        .expect("Failed to get OUT process's output.");

    println!("{}", String::from_utf8_lossy(&out_process.stdout));
}

