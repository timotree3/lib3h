mod network_layer;
pub mod p2p_protocol;
pub mod real_engine;
mod space_layer;

use std::collections::{HashMap, HashSet, VecDeque};

use crate::{
    dht::dht_trait::{Dht, DhtFactory},
    gateway::P2pGateway,
    transport::{transport_trait::Transport, TransportId},
};

use lib3h_crypto_api::{Buffer, CryptoSystem};
use lib3h_protocol::{protocol_client::Lib3hClientProtocol, Address};
use std::{cell::RefCell, rc::Rc};
use url::Url;

/// Identifier of a source chain: SpaceAddress+AgentId
pub type ChainId = (Address, Address);

/// Struct holding all config settings for the RealEngine
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RealEngineConfig {
    pub socket_type: String,
    pub bootstrap_nodes: Vec<String>,
    pub work_dir: String,
    pub log_level: char,
    #[serde(with = "url_serde")]
    pub bind_url: Url,
    pub dht_custom_config: Vec<u8>,
}

#[allow(dead_code)]
pub struct TransportKeys<SecBuf: Buffer, Crypto: CryptoSystem> {
    /// Our TransportId (aka MachineId, "HcMyadayada")
    pub transport_id: String,
    /// The TransportId public key
    pub transport_public_key: Vec<u8>,
    /// The TransportId secret key
    pub transport_secret_key: SecBuf,
    /// needed to accept the Crypto trait generic
    pub phantom_crypto: std::marker::PhantomData<Crypto>,
}

/// Lib3h's 'real mode' as a NetworkEngine
pub struct RealEngine<T: Transport, D: Dht, SecBuf: Buffer, Crypto: CryptoSystem> {
    /// Identifier
    name: String,
    /// Config settings
    config: RealEngineConfig,
    /// FIFO of Lib3hClientProtocol messages received from Core
    inbox: VecDeque<Lib3hClientProtocol>,
    /// Factory for building DHT's of type D
    dht_factory: DhtFactory<D>,
    /// Transport used by the network gateway
    #[allow(dead_code)]
    network_transport: Rc<RefCell<T>>,
    /// P2p gateway for the network layer
    network_gateway: Rc<RefCell<P2pGateway<T, D>>>,
    /// Store active connections?
    network_connections: HashSet<TransportId>,
    /// Map of P2p gateway per Space+Agent
    space_gateway_map: HashMap<ChainId, P2pGateway<P2pGateway<T, D>, D>>,
    #[allow(dead_code)]
    /// transport_id data, public/private keys, etc
    transport_id: TransportKeys<SecBuf, Crypto>,
}
