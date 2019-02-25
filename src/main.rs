extern crate hyper;
extern crate docopt;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use std::io::Read;
use hyper::Client;

static USAGE: &'static str = "
Usage:
    ip-power-bar [-p PORT] [-h HOST] (get | (set <port> (on|off) ))
    ip-power-bar --help

Options:
    -p PORT     The port to listen on [default: 80].
    -h HOST     The host to connect to [default: 192.168.10.100].
    --help      Show this help
";

#[derive(Deserialize, Debug)]
struct Args {
    flag_p: u16,
    flag_h: String,

    arg_port: Option<usize>,
    cmd_on: bool,
    cmd_get: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    println!("Connecting to {}:{}", args.flag_h, args.flag_p);

    let con_string = format!("http://{}:{}/cgi/control.cgi?", args.flag_h, args.flag_p);
    let client = Client::new();

    if args.cmd_get {
        get_port_list(&con_string, &client);
    } else {
        set_port(&con_string, &client, args.arg_port.unwrap(), args.cmd_on);
    }
}

fn get_port_list(con_string: &str, client: &Client) -> () {
    let mut res = client.get(&(con_string.to_string() + "login=p:admin:admin&p=l"))
        .send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    println!("Response: {:?}, {}", res.status, body);
}

fn set_port(con_string: &str, client: &Client, port: usize, state: bool) {
    // create a port list "xxxx" where 'u' means leave as is, '1' enable and '0' disable
    let mut port_list = vec!{'u', 'u', 'u', 'u'};
    let value = if state { '1' } else {'0'};
    match port {
        p @ 1usize ... 4usize => port_list[p-1] = value,
        _ => panic!("port is out of range"),
    };
    let port_list: String = port_list.into_iter().collect();
    let command = format!("{}l=p:admin:admin&p={}", con_string, port_list);
    println!("{}", command);
    let mut res = client.get(&command).send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    println!("Response: {:?}, {}", res.status, body);
}

