//! An arboric::config::Builder allows for a fluent interface for
//! building arboric::Configuration

use super::{JwtSigningKeySource, ListenerConfig};
use crate::abac::Policy;
use crate::arboric::influxdb;
use hyper::Uri;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

/// A ListenerBuilder implements the fluent-syntax builder for
/// [arboric::Configuration](arboric::Configuration)
pub struct ListenerBuilder {
    bind_address: IpAddr,
    port: u16,
    proxy_uri: Option<Uri>,
    jwt_signing_key_source: Option<JwtSigningKeySource>,
    policies: Vec<Policy>,
    influx_db_backend: Option<influxdb::Backend>,
}

impl ListenerBuilder {
    // Constructs a new ListenerBuilder with no JWT signing key source,
    // an empty Policy list, and no query logging
    pub fn new() -> Self {
        ListenerBuilder {
            bind_address: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            port: 0,
            proxy_uri: None,
            jwt_signing_key_source: None,
            policies: Vec::new(),
            influx_db_backend: None,
        }
    }

    pub fn bind_addr(mut self, addr: IpAddr) -> Self {
        self.bind_address = addr;
        self
    }

    pub fn bind_addr_v4(mut self, addr_v4: Ipv4Addr) -> Self {
        self.bind_address = IpAddr::V4(addr_v4);
        self
    }

    pub fn localhost(self) -> Self {
        self.bind_addr_v4(Ipv4Addr::LOCALHOST)
    }

    pub fn bind(mut self, a: u8, b: u8, c: u8, d: u8) -> Self {
        self.bind_address = IpAddr::V4(Ipv4Addr::new(a, b, c, d));
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn proxy<I>(mut self, i: I) -> Self
    where
        I: Into<Uri>,
    {
        self.proxy_uri = Some(i.into());
        self
    }

    /// Configure this `Listener` with a hexadecimal JWT signing key from then environment
    pub fn jwt_from_env_hex<S: Into<String>>(&mut self, key: S) -> &mut Self {
        self.jwt_signing_key_source = Some(JwtSigningKeySource::hex_from_env(key.into()));
        self
    }

    /// Configure this `Listener` with a JWT signing key from a file
    pub fn jwt_from_file<S: Into<String>>(&mut self, filename: S) -> &mut Self {
        self.jwt_signing_key_source = Some(JwtSigningKeySource::from_file(filename.into()));
        self
    }

    pub fn add_policy(&mut self, policy: Policy) -> &mut Self {
        self.policies.push(policy);
        self
    }

    pub fn log_to_influx_db(&mut self, uri: &String, database: &String) -> &mut Self {
        self.influx_db_backend = Some(influxdb::Backend {
            config: influxdb::Config::new(uri.clone(), database.clone()),
        });
        self
    }

    pub fn build(self) -> ListenerConfig {
        ListenerConfig {
            listener_address: SocketAddr::new(self.bind_address, self.port),
            listener_path: None,
            api_uri: self.proxy_uri.unwrap(),
            jwt_signing_key_source: self.jwt_signing_key_source,
            pdp: crate::abac::PDP::with_policies(self.policies),
            influx_db_backend: self.influx_db_backend,
        }
    }
}
