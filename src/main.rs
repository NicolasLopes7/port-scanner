use std::{
    env,
    io::{self, Write},
    net::{IpAddr, TcpStream},
    process,
    str::FromStr,
    sync::mpsc::{channel, Sender},
};

const MAX_PORT: u16 = 65535;

struct Arguments {
    ip_address: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        let args_count = args.len();

        match args_count {
            args_count if args_count < 2 => return Err("Not enough arguments"),
            args_count if args_count > 4 => return Err("Too many arguments"),
            _ => {}
        }

        let flag = &args[1];
        if let Ok(ip_address) = IpAddr::from_str(flag) {
            return Ok(Arguments {
                ip_address,
                threads: 4,
            });
        }

        if flag.contains("-h") || flag.contains("--help") {
            match args_count {
                2 => {
                    println!("Usage: -j to select how many threads you want \r\n -h or --help to show this help message");
                    return Err("help");
                }
                _ => Err("Too many arguments"),
            }
        } else if flag.contains("-j") {
            let threads = match args[2].parse::<u16>() {
                Ok(s) => s,
                Err(_) => return Err("Failed to parse thread number"),
            };

            let ip_address = match IpAddr::from_str(&args[3]) {
                Ok(s) => s,
                Err(_) => return Err("Not a valid IPADDRESS. Must be IPv4 or IPv6"),
            };

            return Ok(Arguments {
                threads,
                ip_address,
            });
        } else {
            return Err("Invalid syntax");
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port = start_port + 1;

    loop {
        if TcpStream::connect((addr, port)).is_ok() {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(port).unwrap();
        }

        if (MAX_PORT - port) <= num_threads {
            break;
        }

        port += num_threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{} problem parsing arguments: {}", program, err);
            process::exit(1);
        }
    });

    let (tx, rx) = channel();

    let num_threads = arguments.threads;
    let ip_address = arguments.ip_address;

    for i in 0..num_threads {
        let tx = tx.clone();
        std::thread::spawn(move || {
            scan(tx, i, ip_address, num_threads);
        });
    }

    drop(tx);

    let mut out: Vec<u16> = rx.into_iter().collect();
    out.sort();

    println!("");
    for port in out {
        println!("{} is open", port)
    }
}
