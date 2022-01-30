use std::net;
use netstat2::{AddressFamilyFlags, get_sockets_info, ProtocolFlags, ProtocolSocketInfo, TcpState};
use crate::process::ProcessInfo;

#[derive(Debug)]
pub struct PortInfo {
    pub address: net::IpAddr,
    pub port: u16,
    pub protocol: String,
    pub process_info: Vec<ProcessInfo>,
}


impl From<netstat2::SocketInfo> for PortInfo {
    fn from(socket_info: netstat2::SocketInfo) -> Self {
        let protocol = match socket_info.protocol_socket_info {
            ProtocolSocketInfo::Tcp(_) => "TCP",
            ProtocolSocketInfo::Udp(_) => "UDP",
        };
        Self {
            address: socket_info.local_addr(),
            port: socket_info.local_port(),
            protocol: protocol.to_string(),
            process_info: crate::process::get_process_info(socket_info.associated_pids),
        }
    }
}


pub struct OpenPortsConfig {
    pub ipv6: bool,
    pub ipv4: bool,
    pub udp: bool,
    pub tcp: bool,
    pub mine: bool,
}


pub fn get_open_ports(args: OpenPortsConfig) -> Vec<PortInfo> {
    let mut af_flags: AddressFamilyFlags = AddressFamilyFlags::from_bits(0).unwrap();
    if args.ipv6 {
        af_flags |= AddressFamilyFlags::IPV6;
    }
    if args.ipv4 {
        af_flags |= AddressFamilyFlags::IPV4;
    }

    let mut proto_flags: ProtocolFlags = ProtocolFlags::from_bits(0).unwrap();
    if args.udp {
        proto_flags |= ProtocolFlags::UDP;
    }
    if args.tcp {
        proto_flags |= ProtocolFlags::TCP;
    }

    let sockets = get_sockets_info(af_flags, proto_flags).unwrap_or_default();

    let ports = sockets
        .into_iter()
        .filter(|socket_info| match &socket_info.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp) => tcp.state == TcpState::Listen,
            ProtocolSocketInfo::Udp(_) => true,
        })
        .map(|socket_info| PortInfo::from(socket_info))
        ;
    if args.mine {
        ports
            .filter(|port_info| port_info.process_info.iter()
                .any(|process| process.is_current_user)
            )
            .collect()
    } else {
        ports.collect()
    }
}
