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

fn repl(conn: &mut std::net::TcpStream) {
    let mut rl = Editor::<()>::new();

    if rl.load_history("/tmp/history.txt").is_err() {
        println!("No previous history.");
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
    rl.save_history("/tmp/history.txt").unwrap();
}

fn main() {
    let matches = App::new("netrat")
        .version("0.0.1")
        .author("Cameron Dart <cdart@anduril.com>")
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
        .get_matches();
    let host = matches.value_of("host").unwrap();
    let port = matches.value_of("port").unwrap();
    let hostname = host.to_string() + ":" + &port.to_string();
    let mut conn = TcpStream::connect(hostname).expect("{} {}");
    conn.set_read_timeout(Some(Duration::new(1, 0)));
    repl(&mut conn);
}
