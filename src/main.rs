use std::str::FromStr;

const MULTICAST_ADDR: std::net::Ipv4Addr = std::net::Ipv4Addr::new(224, 0, 0, 251);

fn main() {
    // Initialize UDP socket on port specified by argv
    let listen_port = std::env::args()
        .nth(1)
        .expect("Please specify binding port")
        .parse::<u16>()
        .expect("Please specify valid port");

    let mdns_from_ip = std::env::args()
        .nth(2)
        .expect("Please provide the IP from which you want to originate mDNS queries from");
    let mdns_iface =
        std::net::Ipv4Addr::from_str(&mdns_from_ip).expect("Please provide a valid IPv4");
    let mdns_socket =
        std::net::UdpSocket::bind((mdns_iface, 0)).expect("Unable to bind to multicast DNS");
    /*mdns_socket
    .join_multicast_v4(&MULTICAST_ADDR, &mdns_iface)
    .expect("Unable to join mDNS");*/
    let dns_socket = std::net::UdpSocket::bind((std::net::Ipv4Addr::new(0, 0, 0, 0), listen_port))
        .expect("Unable to bind to requested port");
    let mut buf: [u8; 512] = [0; 512];
    loop {
        if let Ok((msg_size, sender)) = dns_socket.recv_from(&mut buf) {
            mdns_socket
                .send_to(&buf[0..msg_size], (MULTICAST_ADDR, 5353))
                .unwrap();
            println!("{:?}", dns_parser::Packet::parse(&buf).unwrap());
            if let Ok(mdns_len) = mdns_socket.recv(&mut buf) {
                dns_socket.send_to(&buf[0..mdns_len], sender).unwrap();
            }
        }
    }
}
