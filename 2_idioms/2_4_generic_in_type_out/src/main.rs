use derive_more::{AsMut, AsRef};
use std::net::{IpAddr, SocketAddr};

fn main() {
    println!("Refactor me!");

    let mut err = Error::new("NO_USER");
    err.status(404).message("User not found");
    println!("{:?}", err);

    let err2 = Error::default();
    println!("{:?}", err2);
}

#[derive(Debug)]
pub struct Error {
    code: String,
    status: u16,
    message: String,
}

impl Default for Error {
    #[inline]
    fn default() -> Self {
        Self {
            code: "UNKNOWN".to_string(),
            status: 500,
            message: "Unknown error has happened.".to_string(),
        }
    }
}

impl Error {
    pub fn new<S: Into<String>>(code: S) -> Self {
        Self {
            code: code.into(),
            ..Default::default()
        }
    }

    pub fn set_status(&mut self, s: u16) -> &mut Self {
        self.status = s;
        self
    }

    pub fn status(&mut self, s: u16) -> &mut Self {
        self.status = s;
        self
    }

    pub fn message<S: Into<String>>(&mut self, m: S) -> &mut Self {
        self.message = m.into();
        self
    }

    pub fn set_message<S: Into<String>>(&mut self, m: S) -> &mut Self {
        self.message = m.into();
        self
    }
}

#[derive(Debug, Default, AsMut, AsRef)]
pub struct Server(Option<SocketAddr>);

impl Server {
    pub fn bind<A: Into<IpAddr>>(&mut self, ip: A, port: u16) -> &mut Self {
        self.0 = Some(SocketAddr::new(ip.into(), port));
        self
    }

    pub fn get_address(&self) -> Option<&SocketAddr> {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod server_spec {
    use super::*;

    mod bind {
        use std::net::Ipv4Addr;

        use super::*;

        #[test]
        fn sets_provided_address_to_server() {
            let mut server = Server::default();

            server.bind(Ipv4Addr::new(127, 0, 0, 1), 8080);
            assert_eq!(
                format!("{}", server.get_address().unwrap()),
                "127.0.0.1:8080"
            );

            server.bind("::1".parse::<IpAddr>().unwrap(), 9911);
            assert_eq!(format!("{}", server.get_address().unwrap()), "[::1]:9911");
        }
    }
}

#[cfg(test)]
mod error_spec {
    use super::*;

    #[test]
    fn creates_error_with_default_values() {
        let err = Error::default();
        assert_eq!(err.code, "UNKNOWN");
        assert_eq!(err.status, 500);
        assert_eq!(err.message, "Unknown error has happened.");
    }

    #[test]
    fn allows_chaining_methods_to_customize_error() {
        let mut err = Error::new("INVALID_INPUT");
        err.set_status(400).set_message("Invalid input provided");

        assert_eq!(err.code, "INVALID_INPUT");
        assert_eq!(err.status, 400);
        assert_eq!(err.message, "Invalid input provided");
    }
}
