

#[allow(dead_code)]
pub fn use_connection(conn: &Connection) {
    println!("Using enum connection: {}", conn.describe());
    conn.connect();
    match conn {   //we can match on this for specific behavior.
        Connection::Tcp { port: 443, encryption: false, .. } => {
            // Warning case: HTTPS port without encryption
            println!("WARNING: Port 443 detected but encryption is OFF—fallback to insecure mode");
        }
        Connection::Tcp { .. } => {}
        Connection::Udp { .. } => {}
        Connection::LocalHost { .. } => {}
    }
    println!("\n");
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Connection {
    /// TCP connection with address, port, and encryption toggle
    Tcp {
        address: String,
        port: u16,
        encryption: bool,
    },

    /// UDP connection with address and port (no encryption concept)
    Udp {
        address: String,
        port: u16,
    },

    /// Localhost / in-process connection – no address or encryption
    LocalHost {
        port: u16,
    },
}

#[allow(dead_code)]
impl Connection {
    pub fn connect(&self) {
        match self {
            Connection::Tcp {
                address,
                port,
                encryption,
            } => {
                println!(
                    "Connecting via TCP to {}:{} (encryption: {})",
                    address, port, encryption
                );
                // Real code: TcpStream::connect((address.as_str(), *port))?;
            }

            Connection::Udp { address, port } => {
                println!("Connecting via UDP to {}:{}", address, port);
                // Real code: UdpSocket::bind(...)?
            }

            Connection::LocalHost { port } => {
                println!("Connecting to local service on port {}", port);
                // Real code: connect to local in-process channel.
            }
        }
    }

    pub fn describe(&self) -> String {
        match self {
            Connection::Tcp {
                address,
                port,
                encryption,
            } => format!(
                "tcp://{}:{} (encrypted: {})",
                address, port, encryption
            ),
            Connection::Udp { address, port } => {
                format!("udp://{}:{}", address, port)
            }
            Connection::LocalHost { port } => format!("local://{}", port),
        }
    }
}

