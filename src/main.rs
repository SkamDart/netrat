extern crate clap;
extern crate rustyline;

use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use std::time::Duration;

use clap::{App, Arg};
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn execute(conn: &mut std::net::TcpStream, cmd: String) {
    if let Ok(_) = conn.write(cmd.as_bytes()) {
        let mut res = [0 as u8; 1024];
        if let Ok(_) = conn.read(&mut res) {
            println!("{}", from_utf8(&res).unwrap());
        }
    } else {
        println!("failed to write {}", cmd);
    }
}

fn repl(conn: &mut std::net::TcpStream, history_path: &str) {
    let mut rl = Editor::<()>::new();
    if rl.load_history(history_path).is_err() {
        println!("Unable to use history path {}", history_path);
    }

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                rl.add_history_entry(line.to_string());
                execute(conn, line.to_string() + "\r\n");
            }
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
    rl.save_history(history_path).unwrap();
}

fn main() {
    let matches = App::new("netrat")
        .version("0.0.1")
        .author("Cameron Dart <camerondart13@gmail.com>")
        .about("Netrat is a netcat clone written in rust.")
        .arg(
            Arg::with_name("host")
                .help("hostname to connect to")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("port")
                .help("port to connect to")
                .index(2)
                .required(true),
        )
        .arg(
            Arg::with_name("history_file")
                .help("file where history commands are stored")
                .required(false)
                .takes_value(true)
        )
        .get_matches();

    let host = matches.value_of("host").unwrap();
    let port = matches.value_of("port").unwrap();
    let hostname = host.to_string() + ":" + &port.to_string();
    let history_path =
        matches.value_of("history_file").unwrap_or("/tmp/history.txt");
    let mut conn = TcpStream::connect(hostname).expect("unable to connect");
    conn.set_read_timeout(Some(Duration::new(1, 0))).expect("unable to set duration");
    repl(&mut conn, history_path);
}
