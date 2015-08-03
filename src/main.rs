extern crate hyper;
extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;
use std::io::Read;
use hyper::Client;

static USAGE: &'static str = "
Usage: ip_power_bar [-p PORT] [-h HOST] (get | (set <port> (on|off) ))

Options:
    -p PORT    The port to listen on [default: 80].
    -h HOST    The host to connect to [default: 192.168.10.100].
";

#[derive(RustcDecodable, Debug)]
struct Args {
    flag_p: u16,
    flag_h: String,

    arg_port: Option<usize>,
    cmd_on: bool,
    cmd_get: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    println!("Connecting to {}:{}", args.flag_h, args.flag_p);

    let con_string = format!("http://{}:{}/cgi/control.cgi?", args.flag_h, args.flag_p);
    let client = Client::new();

    if args.cmd_get {
        let mut res = client.get(&(con_string + "login=p:admin:admin&p=l"))
            .send().unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        println!("Response: {:?}, {}", res.status, body);
    } else {
        // create a port list "xxxx" where 'u' means leave as is, '1' enable and '0' disable
        let mut port_list = vec!{'u', 'u', 'u', 'u'};
        let value = if args.cmd_on { '1' } else {'0'};
        match args.arg_port.unwrap() {
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
}

