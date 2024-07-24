use std::borrow::Cow;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const DNS_SERVER: &str = "1.1.1.1:53";
const TIMEOUT: Duration = Duration::from_secs(10);

struct SocketPool {
    sockets: Mutex<Vec<UdpSocket>>,
}

impl SocketPool {
    fn new() -> Self {
        SocketPool {
            sockets: Mutex::new(Vec::new()),
        }
    }

    fn get(&self) -> UdpSocket {
        let mut pool = self.sockets.lock().unwrap();
        pool.pop()
            .unwrap_or_else(|| UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket"))
    }

    fn return_socket(&self, socket: UdpSocket) {
        let mut pool = self.sockets.lock().unwrap();
        pool.push(socket);
    }
}

lazy_static::lazy_static! {
    static ref SOCKET_POOL: Arc<SocketPool> = Arc::new(SocketPool::new());
}

#[derive(Debug)]
struct DnsHeader {
    id: u16,
    flags: u16,
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DnsAnswer {
    name: String,
    record_type: u16,
    class: u16,
    ttl: u32,
    rd_length: u16,
    rdata: Vec<u8>,
    rdata_offset: usize,
}

impl DnsAnswer {
    pub fn new(
        name: String,
        record_type: u16,
        class: u16,
        ttl: u32,
        rd_length: u16,
        rdata: Vec<u8>,
        rdata_offset: usize,
    ) -> Self {
        Self {
            name,
            record_type,
            class,
            ttl,
            rd_length,
            rdata,
            rdata_offset,
        }
    }

    pub fn name_server<'a>(&self, response: &'a [u8]) -> Cow<'a, str> {
        if self.record_type == 2 {
            // NS record
            Cow::Owned(parse_compressed_name(response, self.rdata_offset))
        } else {
            Cow::Borrowed("Not a name server record")
        }
    }
}

fn build_dns_query(domain: &str) -> Vec<u8> {
    let mut query = Vec::with_capacity(512);

    let header = DnsHeader {
        id: 0x1234,
        flags: 0x0100,
        qdcount: 1,
        ancount: 0,
        nscount: 0,
        arcount: 0,
    };

    query.extend_from_slice(&header.id.to_be_bytes());
    query.extend_from_slice(&header.flags.to_be_bytes());
    query.extend_from_slice(&header.qdcount.to_be_bytes());
    query.extend_from_slice(&header.ancount.to_be_bytes());
    query.extend_from_slice(&header.nscount.to_be_bytes());
    query.extend_from_slice(&header.arcount.to_be_bytes());

    for label in domain.split('.') {
        query.push(label.len() as u8);
        query.extend_from_slice(label.as_bytes());
    }
    query.push(0);

    query.extend_from_slice(&2u16.to_be_bytes());
    query.extend_from_slice(&1u16.to_be_bytes());

    query
}

fn parse_domain_name(response: &[u8], offset: &mut usize) -> String {
    let mut domain = String::new();
    let mut jump_performed = false;
    let mut local_offset = *offset;

    loop {
        let length = response[local_offset] as usize;
        if length == 0 {
            if !jump_performed {
                *offset = local_offset + 1;
            }
            break;
        }

        if length & 0xC0 == 0xC0 {
            if !jump_performed {
                *offset = local_offset + 2;
            }
            local_offset = ((length & 0x3F) as usize) << 8 | response[local_offset + 1] as usize;
            jump_performed = true;
            continue;
        }

        local_offset += 1;
        if !domain.is_empty() {
            domain.push('.');
        }
        domain
            .push_str(std::str::from_utf8(&response[local_offset..local_offset + length]).unwrap());
        local_offset += length;
    }

    domain
}

fn parse_compressed_name(response: &[u8], offset: usize) -> String {
    let mut result = String::new();
    let mut jumped = false;
    let max_jumps = 5;
    let mut jumps = 0;
    let mut local_offset = offset;

    loop {
        if jumps > max_jumps {
            return String::from("Error: Too many jumps");
        }

        if local_offset >= response.len() {
            return String::from("Error: Offset out of bounds");
        }

        let length = response[local_offset] as usize;

        if length == 0 {
            break;
        }

        if length & 0xC0 == 0xC0 {
            if local_offset + 1 >= response.len() {
                return String::from("Error: Invalid compression pointer");
            }
            let pointer = ((length & 0x3F) as usize) << 8 | response[local_offset + 1] as usize;
            if !jumped {
                jumped = true;
            }
            local_offset = pointer;
            jumps += 1;
        } else {
            local_offset += 1;
            if local_offset + length > response.len() {
                return String::from("Error: Label length exceeds message bounds");
            }
            if !result.is_empty() {
                result.push('.');
            }
            result.push_str(
                std::str::from_utf8(&response[local_offset..local_offset + length])
                    .unwrap_or("Error: Invalid UTF-8"),
            );
            local_offset += length;
        }
    }

    result
}

fn parse_dns_response(response: &[u8]) -> Option<Vec<DnsAnswer>> {
    let header = DnsHeader {
        id: u16::from_be_bytes([response[0], response[1]]),
        flags: u16::from_be_bytes([response[2], response[3]]),
        qdcount: u16::from_be_bytes([response[4], response[5]]),
        ancount: u16::from_be_bytes([response[6], response[7]]),
        nscount: u16::from_be_bytes([response[8], response[9]]),
        arcount: u16::from_be_bytes([response[10], response[11]]),
    };

    if header.ancount == 0 {
        return None;
    }

    let mut offset = 12;

    let _question_domain = parse_domain_name(response, &mut offset);
    offset += 4;

    let mut answers = Vec::with_capacity(header.ancount as usize);

    for _ in 0..header.ancount {
        let name = parse_domain_name(response, &mut offset);
        let record_type = u16::from_be_bytes([response[offset], response[offset + 1]]);
        offset += 2;
        let class = u16::from_be_bytes([response[offset], response[offset + 1]]);
        offset += 2;
        let ttl = u32::from_be_bytes([
            response[offset],
            response[offset + 1],
            response[offset + 2],
            response[offset + 3],
        ]);
        offset += 4;
        let rd_length = u16::from_be_bytes([response[offset], response[offset + 1]]);
        offset += 2;
        let rdata_offset = offset;
        let rdata = response[offset..offset + rd_length as usize].to_vec();
        offset += rd_length as usize;

        answers.push(DnsAnswer::new(
            name,
            record_type,
            class,
            ttl,
            rd_length,
            rdata,
            rdata_offset,
        ));
    }

    Some(answers)
}

fn lookup(domain: &str) -> Option<(Vec<DnsAnswer>, Vec<u8>)> {
    let dns_server: std::net::SocketAddr = DNS_SERVER.parse().expect("Invalid DNS server address");
    let socket = SOCKET_POOL.get();
    socket
        .set_read_timeout(Some(TIMEOUT))
        .expect("Failed to set read timeout");

    let query = build_dns_query(domain);
    socket
        .send_to(&query, dns_server)
        .expect("Failed to send DNS query");

    let mut response = vec![0; 512];
    match socket.recv_from(&mut response) {
        Ok((num_bytes, _)) => {
            response.truncate(num_bytes);
            let result = parse_dns_response(&response).map(|answers| (answers, response.clone()));
            SOCKET_POOL.return_socket(socket);
            result
        }
        Err(e) => {
            eprintln!("Error receiving DNS response: {:?}", e);
            SOCKET_POOL.return_socket(socket);
            None
        }
    }
}

pub fn get_nameservers(domain: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    match lookup(domain) {
        Some((answers, response)) => {
            let mut name_servers = Vec::new();
            for answer in answers {
                if answer.record_type == 2 {
                    let ns = answer.name_server(&response);
                    if ns.starts_with("Error:") {
                        return Err(ns.into());
                    }
                    name_servers.push(ns.to_string());
                }
            }
            Ok(name_servers)
        }
        None => Err("Failed to retrieve DNS information".into()),
    }
}
