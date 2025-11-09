pub mod port;
pub mod process;

use clap::Parser;
use port::{OpenPortsConfig, PortInfo};
use process::ProcessInfo;
use std::borrow::Cow;
use std::process::Command;
use tabular::{Row, Table};

/// A command line tool to search for and manage processes (using listened ports and more.)
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Filter to show only the process that is using a given port.
    #[arg(short, long, value_name = "PORT")]
    port: Option<u16>,

    /// Display headers and all columns of process metadata. Output is similar to ps -u.
    #[arg(short, long, conflicts_with = "command")]
    all: bool,

    /// By default, proc requires a filter (currently, only -p) to execute a command,
    /// so that you don't accidentally run a command (like kill) on every active process. 💀
    /// Use --force if you need to override this.
    #[arg(long, requires = "command")]
    force: bool,

    /// Command to execute. The pid is passed either at the end, or replacing {}.
    /// Example: proc --port 5000 -- kill
    ///
    /// will send kill (SIGKILL) to the process listening on port 5000.
    #[arg(last = true, num_args = 1..)]
    command: Vec<String>,
}

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
    let args = Args::parse();

    let mut ports = port::get_open_ports(OpenPortsConfig {
        ipv6: true,
        ipv4: true,
        udp: true,
        tcp: true,
        mine: false,
    });
    if let Some(using_port) = args.port {
        ports = ports
            .into_iter()
            .filter(|port| port.port == using_port)
            .collect::<Vec<_>>();
        if ports.is_empty() {
            eprintln!("No processes found listening on that port.");
            return;
        }
    }

    let processes = get_processes(ports);

    if !args.command.is_empty() {
        let mut command = args
            .command
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>();
        if !command.contains(&"{}") {
            command.push("{}");
        }
        if !args.force && processes.len() > 1 {
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
                    command
                        .iter()
                        .map(|s| if s == &"{}" {
                            Cow::from(&pid)
                        } else {
                            shell_escape::escape(Cow::from(*s))
                        })
                        .collect::<Vec<_>>()
                        .join(" "),
                    status.code().unwrap()
                );
            }
        }
        return;
    }
    let table_header = args.all || atty::is(atty::Stream::Stdout);
    let table_columns = args.all || atty::is(atty::Stream::Stdout);

    let mut table = if table_columns {
        Table::new("{:>}\t{:<}")
    } else {
        Table::new("{:<}")
    };
    if table_header {
        let mut row = Row::new().with_cell("PID");
        if table_columns {
            row = row.with_cell("COMMAND");
        }
        table.add_row(row);
    }
    for proc in processes {
        let mut row = Row::new().with_cell(proc.pid.to_string());
        if table_columns {
            row = row.with_cell(
                &proc
                    .command
                    .iter()
                    .map(|s| shell_escape::escape(Cow::from(s)))
                    .collect::<Vec<_>>()
                    .join(" "),
            );
        }
        table.add_row(row);
    }
    print!("{}", table);
}
