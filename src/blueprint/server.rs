use std::collections::BTreeMap;
use std::net::{AddrParseError, IpAddr};

use derive_setters::Setters;
use hyper::HeaderMap;

use crate::config;
use crate::valid::{Valid, ValidationError, ValidStructCompatibility};

#[derive(Clone, Debug, Setters)]
pub struct Server {
    pub enable_apollo_tracing: bool,
    pub enable_cache_control_header: bool,
    pub enable_graphiql: bool,
    pub enable_introspection: bool,
    pub enable_query_validation: bool,
    pub enable_response_validation: bool,
    pub global_response_timeout: i64,
    pub port: u16,
    pub hostname: IpAddr,
    pub vars: BTreeMap<String, String>,
    pub response_headers: HeaderMap,
}

impl Default for Server {
    fn default() -> Self {
        // NOTE: Using unwrap because try_from default will never fail
        Server::try_from(config::Server::default()).unwrap()
    }
}

impl Server {
    pub fn get_enable_http_validation(&self) -> bool {
        self.enable_response_validation
    }
    pub fn get_enable_cache_control(&self) -> bool {
        self.enable_cache_control_header
    }

    pub fn get_enable_introspection(&self) -> bool {
        self.enable_introspection
    }

    pub fn get_enable_query_validation(&self) -> bool {
        self.enable_query_validation
    }
}

fn validate_hostname(hostname: String) -> Valid<IpAddr, String> {
    if hostname == "localhost" {
        Valid::succeed(IpAddr::from([127, 0, 0, 1]))
    } else {
        Valid::from(
            hostname
                .parse()
                .map_err(|e: AddrParseError| ValidationError::new(format!("Parsing failed because of {}", e))),
        )
            .trace("hostname")
            .trace("@server")
            .trace("schema")
    }
}
