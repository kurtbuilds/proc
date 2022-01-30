pub mod port;
pub mod process;

use std::borrow::Cow;
use sysinfo::{Process, ProcessExt, System, SystemExt};
use users::{get_current_uid, get_user_by_uid, User};
use std::net;
use std::process::{Command, ExitStatus};
use clap::Arg;
use libproc::libproc::proc_pid::{listpids, ProcType};
use netstat2::{AddressFamilyFlags, get_sockets_info, ProtocolFlags, ProtocolSocketInfo, TcpState};
use tabular::{Row, Table};
use port::{OpenPortsConfig, PortInfo};
use process::ProcessInfo;


const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");


fn get_processes(ports: Vec<PortInfo>) -> Vec<ProcessInfo> {
    let mut processes: Vec<ProcessInfo> = Vec::new();
    for port in ports.into_iter() {
        let proc = port.process_info.into_iter().next().unwrap();
        if !processes.iter().any(|p| p.pid == proc.pid) {
            processes.push(proc);
        }
    }
    processes
}


fn main() {
    let args = clap::App::new(NAME)
        .version(VERSION)
        .arg(Arg::new("port")
            .long("port")
            .short('p')
            .value_name("PORT")
            .help("Filter to show only the process that is using a given port.")
            .takes_value(true)
        )
        .arg(Arg::new("all")
            .long("all")
            .short('a')
            .conflicts_with("command")
            .help("Display headers and all columns of process metadata. Output is similar to ps -u.")
            .takes_value(false)
        )
        .arg(Arg::new("force")
            .long("force")
            .requires("command")
            .help("By default, proc requires a filter (currently, only -u) to execute a command, \
            so that you don't accidentally run a command (like kill) on every active process. 💀 \
            Use --force if you need to override this.")
            .takes_value(false)
        )
        .arg(Arg::new("command")
            .last(true)
            .help("Command to execute. The pid is passed either at the end, or replacing {}. Example:
            proc --port 5000 -- kill

            will send kill (SIGKILL) to the process listening on port 5000.
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
    if args.is_present("port") {
        let using_port = args.value_of("port").unwrap().parse::<u16>().unwrap();
        ports = ports.into_iter().filter(|port| port.port == using_port).collect::<Vec<_>>();
        if ports.is_empty() {
            eprintln!("No processes found listening on that port.");
            return;
        }
    }

    let processes = get_processes(ports);

    if args.is_present("command") {
        let mut command = args.values_of("command").unwrap().collect::<Vec<&str>>();
        if !command.contains(&"{}") {
            command.push("{}");
        }
        if !args.is_present("force") && processes.len() > 1 {
            eprintln!("This would run the provided command on multiple processes. If you are absolutely sure you want to do this, use the --force option.");
            return;
        }
        for proc in processes {
            let mut args = command.iter();
            let pid = proc.pid.to_string();
            let status = Command::new(args.next().unwrap())
                .args(args.map(|&s| if s == "{}" { &pid } else { s }))
                .status()
                .unwrap();
            if !status.success() {
                eprintln!(
                    "{}: Failed with exit code {}",
                    command.iter().map(|s| if s == &"{}" {
                        Cow::from(&pid)
                    } else {
                        shell_escape::escape(Cow::from(*s))
                    }).collect::<Vec<_>>().join(" "),
                    status.code().unwrap()
                );
            }
        }
        return;
    }
    let table_header = args.is_present("all");
    let table_columns = args.is_present("all");

    let mut table =
        if table_columns {
            Table::new("{:>}\t{:<}")
        } else {
            Table::new("{:<}")
        };
    if table_header {
        let mut row = Row::new()
            .with_cell("PID");
        if table_columns {
            row = row.with_cell("COMMAND");
        }
        table.add_row(row);
    }
    for proc in processes {
        let mut row = Row::new()
            .with_cell(proc.pid.to_string());
        if table_columns {
            row = row.with_cell(&proc.command.iter().map(|s| shell_escape::escape(Cow::from(s))).collect::<Vec<_>>().join(" "));
        }
        table.add_row(row);
    }
    print!("{}", table);
}