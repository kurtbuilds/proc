pub mod port;
pub mod process;
use sysinfo::{ProcessExt, System, SystemExt};
use users::{get_current_uid, get_user_by_uid, User};
use std::net;
use clap::Arg;
use libproc::libproc::proc_pid::{listpids, ProcType};
use netstat2::{AddressFamilyFlags, get_sockets_info, ProtocolFlags, ProtocolSocketInfo, TcpState};
use port::{OpenPortsConfig, PortInfo};
use process::ProcessInfo;


const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    let args = clap::App::new(NAME)
        .version(VERSION)
        .arg(Arg::new("using")
            .long("using")
            .short('u')
            .takes_value(true)
        )
        .arg(Arg::new("command")
            .last(true)
            .help("Command to execute. The pid is passed either at the end, or replacing {}. Example:
            procs -u 5000 -- kill

            will send kill (SIGKILL) to the process using port 5000.
            ")
            .multiple_values(true)
        )
        .get_matches();

    let mut ports = port::get_open_ports(OpenPortsConfig {
        ipv6: true,
        ipv4: true,
        udp: true,
        tcp: true,
        mine: false,
    });
    if args.is_present("using") {
        let using_port = args.value_of("using").unwrap().parse::<u16>().unwrap();
        ports = ports.into_iter().filter(|port| port.port == using_port).collect::<Vec<_>>();
    }
    if ports.is_empty() {
        eprintln!("No processes found listening on that port.");
    } else {
        for port in ports {
            println!("{:?}", port);
        }
    }
}
