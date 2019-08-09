#![allow(non_snake_case)]

use crate::{
    dht::{dht_protocol::*, dht_trait::Dht},
    engine::p2p_protocol::P2pProtocol,
    gateway::{Gateway, P2pGateway, SendReliableCheckAddressLoopData, TrackType},
    transport::{
        error::{TransportError, TransportResult},
        protocol::{
            FailureResultData, SendData, SuccessResultData, TransportCommand, TransportEvent,
        },
        transport_trait::Transport,
        ConnectionId, ConnectionIdRef,
    },
};
use lib3h_protocol::DidWork;
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use url::Url;

/// Compose Transport
impl<'gateway, D: Dht> Transport for P2pGateway<'gateway, D> {
    // TODO #176 - Return a higher-level uri instead?
    fn connect(&mut self, uri: &Url) -> TransportResult<ConnectionId> {
        trace!("({}).connect() {}", self.identifier, uri);
        // Connect
        let connection_id = self.inner_transport.as_mut().connect(&uri)?;
        // Store result in connection map
        self.connection_map
            .insert(uri.clone(), connection_id.clone());
        // Done
        Ok(connection_id)
    }

    // TODO #176 - remove conn id conn_map??
    fn close(&mut self, id: &ConnectionIdRef) -> TransportResult<()> {
        self.inner_transport.as_mut().close(id)
    }

    // TODO #176
    fn close_all(&mut self) -> TransportResult<()> {
        self.inner_transport.as_mut().close_all()
    }

    /// id_list =
    ///   - Network : transportId
    ///   - space   : agentId
    /*
    fn send(&mut self, id_list: &[&ConnectionIdRef], payload: &[u8]) -> TransportResult<()> {
        Transport::post(self, TransportCommand::SendReliable(SendData {
            id_list: id_list.iter().map(|x| x.to_string()).collect(),
            payload: payload.to_vec(),
            request_id: "".to_string(),
        }))?;
        Ok(())
    }
    */

    ///
    fn send_all(&mut self, payload: &[u8]) -> TransportResult<()> {
        let connection_list = self.connection_id_list()?;
        let id_list: Vec<&str> = connection_list.iter().map(|v| &**v).collect();
        trace!("({}) send_all() {:?}", &self.identifier, &id_list);
        //self.send(&dht_id_list, payload)
        Transport::post(
            self,
            TransportCommand::SendReliable(SendData {
                id_list: id_list.iter().map(|x| x.to_string()).collect(),
                payload: payload.to_vec(),
                request_id: None,
            }),
        )
    }

    ///
    fn bind(&mut self, url: &Url) -> TransportResult<Url> {
        trace!("({}) bind() {}", self.identifier, url);
        self.inner_transport.as_mut().bind(url)
    }

    ///
    fn post(&mut self, command: TransportCommand) -> TransportResult<()> {
        self.transport_inbox.push_back(command);
        Ok(())
    }

    /// Handle TransportEvents directly
    fn process(&mut self) -> TransportResult<(DidWork, Vec<TransportEvent>)> {
        let mut did_work = false;

        for item in self.workflow.drain(..).collect::<Vec<_>>() {
            if self.serve_try_send_reliable_check_address_loop_data(item)? {
                did_work = true;
            }
        }

        // Process TransportCommand inbox
        loop {
            let cmd = match self.transport_inbox.pop_front() {
                None => break,
                Some(msg) => msg,
            };
            did_work = true;
            self.serve_TransportCommand(&cmd)?;
        }
        trace!(
            "({}).Transport.process() - output: {} {}",
            self.identifier,
            did_work,
            self.transport_outbox.len(),
        );
        // Process inner transport
        // Its okay to process inner transport as long as NetworkEngine only calls
        // Transport::process() on the network gateway,
        // otherwise remove this code and have RealEngine explicitly call the process of the
        // Network transport.
        let (inner_did_work, mut event_list) = self.inner_transport.as_mut().process()?;
        trace!(
            "({}).Transport.inner_process() - output: {} {}",
            self.identifier,
            inner_did_work,
            event_list.len()
        );

        if inner_did_work {
            did_work = true;
        }

        if !event_list.is_empty() {
            self.transport_outbox.append(&mut event_list);
        }

        // TODO XXX - I don't think we want to process ALL these in BOTH places
        //            we should iterate locally and selectively forward to
        //            our outbox

        // Handle TransportEvents
        for evt in self.transport_outbox.clone().iter() {
            self.handle_TransportEvent(&evt)?;
        }

        Ok((did_work, self.transport_outbox.drain(..).collect()))
    }

    /// A Gateway uses its inner_dht's peerData.peer_address as connectionId
    fn connection_id_list(&self) -> TransportResult<Vec<ConnectionId>> {
        let peer_data_list = self.inner_dht.get_peer_list();
        let mut id_list = Vec::new();
        for peer_data in peer_data_list {
            id_list.push(peer_data.peer_address);
        }
        Ok(id_list)
    }

    /// TODO: return a higher-level uri instead
    fn get_uri(&self, id: &ConnectionIdRef) -> Option<Url> {
        self.inner_transport.as_ref().get_uri(id)
        //let maybe_peer_data = self.inner_dht.get_peer(id);
        //maybe_peer_data.map(|pd| pd.peer_address)
    }
}

/// Private internals
impl<'gateway, D: Dht> P2pGateway<'gateway, D> {
    /// Get Uris from DHT peer_address'
    pub(crate) fn dht_address_to_uri_list(
        &self,
        address_list: &[String],
    ) -> TransportResult<Vec<Url>> {
        let mut uri_list = Vec::with_capacity(address_list.len());
        for address in address_list {
            let maybe_peer = self.inner_dht.get_peer(address);
            match maybe_peer {
                None => {
                    return Err(TransportError::new(format!(
                        "Unknown peerAddress: {}",
                        address
                    )));
                }
                Some(peer) => uri_list.push(peer.peer_uri),
            }
        }
        Ok(uri_list)
    }

    fn handle_new_connection(&mut self, id: &ConnectionIdRef) -> TransportResult<()> {
        let maybe_uri = self.get_uri(id);
        if maybe_uri.is_none() {
            return Ok(());
        }
        let uri = maybe_uri.unwrap();
        trace!("({}) new_connection: {} -> {}", self.identifier, uri, id,);
        // TODO #176 - Maybe we shouldn't have different code paths for populating
        // the connection_map between space and network gateways.
        let maybe_previous = self.connection_map.insert(uri.clone(), id.to_string());
        if let Some(previous_cId) = maybe_previous {
            debug!("Replaced connectionId for {} ; was: {}", uri, previous_cId,);
        }

        // Send to other node our PeerAddress
        let this_peer = self.this_peer().clone();
        let our_peer_address = P2pProtocol::PeerAddress(
            self.identifier().to_string(),
            this_peer.peer_address,
            this_peer.timestamp,
        );
        let mut buf = Vec::new();
        our_peer_address
            .serialize(&mut Serializer::new(&mut buf))
            .unwrap();
        trace!(
            "({}) sending P2pProtocol::PeerAddress: {:?} to {:?}",
            self.identifier,
            our_peer_address,
            id,
        );
        let request_id = self.register_track(TrackType::TransportSendFireAndForget);
        self.inner_transport
            .as_mut()
            .post(TransportCommand::SendReliable(SendData {
                id_list: vec![id.to_string()],
                payload: buf,
                request_id: Some(request_id),
            }))
        //return self.inner_transport.as_mut().send(&[&id], &buf);
    }

    /// Process a transportEvent received from our internal connection.
    pub(crate) fn handle_TransportEvent(&mut self, evt: &TransportEvent) -> TransportResult<()> {
        debug!(
            "<<< '({})' recv transport event: {:?}",
            self.identifier, evt
        );
        // Note: use same order as the enum
        match evt {
            TransportEvent::ErrorOccured(id, e) => {
                error!(
                    "({}) Connection Error for {}: {}\n Closing connection.",
                    self.identifier, id, e,
                );
                self.inner_transport.as_mut().close(id)?;
            }
            TransportEvent::ConnectResult(id, _) => {
                info!("({}) Outgoing connection opened: {}", self.identifier, id);
                self.handle_new_connection(id)?;
            }
            TransportEvent::IncomingConnectionEstablished(id) => {
                info!("({}) Incoming connection opened: {}", self.identifier, id);
                self.handle_new_connection(id)?;
            }
            TransportEvent::ConnectionClosed(_id) => {
                // TODO #176
            }
            TransportEvent::ReceivedData(connection_id, payload) => {
                debug!("Received message from: {}", connection_id);
                // trace!("Deserialize msg: {:?}", payload);
                let mut de = Deserializer::new(&payload[..]);
                let maybe_p2p_msg: Result<P2pProtocol, rmp_serde::decode::Error> =
                    Deserialize::deserialize(&mut de);
                if let Ok(p2p_msg) = maybe_p2p_msg {
                    if let P2pProtocol::PeerAddress(gateway_id, peer_address, peer_timestamp) =
                        p2p_msg
                    {
                        debug!(
                            "Received PeerAddress: {} | {} ({})",
                            peer_address, gateway_id, self.identifier
                        );
                        let peer_uri = self
                            .inner_transport
                            .as_mut()
                            .get_uri(connection_id)
                            .expect("FIXME"); // TODO #58
                        debug!("peer_uri of: {} = {}", connection_id, peer_uri);
                        if self.identifier == gateway_id {
                            let peer = PeerData {
                                peer_address: peer_address.clone(),
                                peer_uri,
                                timestamp: peer_timestamp,
                            };
                            Dht::post(self, DhtCommand::HoldPeer(peer)).expect("FIXME"); // TODO #58
                                                                                         // TODO #150 - Should not call process manually
                            Dht::process(self).expect("HACK");
                        }
                    }
                }
            }
            TransportEvent::SuccessResult(msg) => {
                if let Some(track_type) = self.request_track.remove(&msg.request_id) {
                    match track_type {
                        TrackType::TransportSendFireAndForget => {
                            trace!("SUCCESS RESULT: {:?}", msg);
                            // all good :+1:
                        }
                        TrackType::TransportSendDelegateLower { gateway_request_id } => {
                            self.transport_outbox.push(TransportEvent::SuccessResult(
                                SuccessResultData {
                                    request_id: gateway_request_id,
                                },
                            ));
                        }
                    }
                }
            }
            TransportEvent::FailureResult(msg) => {
                if let Some(track_type) = self.request_track.remove(&msg.request_id) {
                    match track_type {
                        TrackType::TransportSendFireAndForget => {
                            error!("FAILURE RESULT: {:?}", msg);
                        }
                        TrackType::TransportSendDelegateLower { gateway_request_id } => {
                            self.transport_outbox.push(TransportEvent::FailureResult(
                                FailureResultData {
                                    request_id: gateway_request_id,
                                    error: msg.error.clone(),
                                },
                            ));
                        }
                    }
                }
            }
        };
        Ok(())
    }

    /// Process a TransportCommand: Call the corresponding method and possibily return some Events.
    /// Return a list of TransportEvents to owner.
    #[allow(non_snake_case)]
    fn serve_TransportCommand(&mut self, cmd: &TransportCommand) -> TransportResult<()> {
        trace!("({}) serving transport cmd: {:?}", self.identifier, cmd);
        // Note: use same order as the enum
        match cmd {
            TransportCommand::Connect(url, request_id) => {
                let id = self.connect(url)?;
                let evt = TransportEvent::ConnectResult(id, request_id.clone());
                self.transport_outbox.push(evt);
            }
            TransportCommand::SendReliable(msg) => {
                self.serve_TransportCommand_SendReliable(msg)?;
            }
            TransportCommand::SendAll(payload) => {
                let _id = self.send_all(payload)?;
            }
            TransportCommand::Close(id) => {
                self.close(id)?;
                let evt = TransportEvent::ConnectionClosed(id.to_string());
                self.transport_outbox.push(evt);
            }
            TransportCommand::CloseAll => {
                self.close_all()?;
            }
            TransportCommand::Bind(url) => {
                self.bind(url)?;
            }
        }
        Ok(())
    }

    #[allow(non_snake_case)]
    fn serve_TransportCommand_SendReliable(&mut self, msg: &SendData) -> TransportResult<()> {
        self.workflow.push(SendReliableCheckAddressLoopData {
            msg: msg.clone(),
            last_tickle_ms: 0,
            expires_ms: crate::time::since_epoch_ms() + 200,
        });
        Ok(())
    }

    fn serve_try_send_reliable_check_address_loop_data(
        &mut self,
        mut item: SendReliableCheckAddressLoopData,
    ) -> TransportResult<bool> {
        let now = crate::time::since_epoch_ms();
        if now - item.last_tickle_ms < 10 {
            self.workflow.push(item);
            return Ok(false);
        }
        item.last_tickle_ms = now;
        match self.serve_try_send_reliable_check_address_loop_data_inner(&item.msg) {
            Ok(_) => Ok(true),
            Err(e) => {
                if item.expires_ms > now {
                    self.workflow.push(item);
                    return Ok(true);
                }
                if let Some(rid) = item.msg.request_id {
                    self.transport_outbox
                        .push(TransportEvent::FailureResult(FailureResultData {
                            request_id: rid.clone(),
                            error: vec![e],
                        }));
                } else {
                    error!("failed to send: {:?}", e);
                }
                Ok(true)
            }
        }
    }

    fn serve_try_send_reliable_check_address_loop_data_inner(
        &mut self,
        msg: &SendData,
    ) -> TransportResult<()> {
        // get connectionId from the inner dht first
        let dht_uri_list = self.dht_address_to_uri_list(msg.id_list.as_slice())?;
        // send
        trace!(
            "({}).send() {:?} -> {:?} | {}",
            self.identifier,
            &msg.id_list,
            dht_uri_list,
            msg.payload.len(),
        );
        // Get connectionIds for the inner Transport.
        let mut conn_list: Vec<String> = Vec::new();
        for dht_uri in dht_uri_list {
            let net_id = self.connection_map.get(&dht_uri).expect("unknown dht_uri");
            conn_list.push(net_id.to_string());
            trace!(
                "({}).send() reversed mapped dht_uri {:?} to net_uri {:?}",
                self.identifier,
                dht_uri,
                net_id,
            )
        }

        let request_id = msg.request_id.clone().map(|rid| {
            self.register_track(TrackType::TransportSendDelegateLower {
                gateway_request_id: rid,
            })
        });
        self.inner_transport
            .as_mut()
            .post(TransportCommand::SendReliable(SendData {
                id_list: conn_list,
                payload: msg.payload.clone(),
                request_id,
            }))

        /*
        // Send on the inner Transport
        self.inner_transport.as_mut().send(&ref_list, &msg.payload)?;

        self.outbox.push(TransportEvent::SuccessResult(SuccessResultData {
            request_id: msg.request_id.clone(),
        }));
        Ok(())
        */
    }
}
