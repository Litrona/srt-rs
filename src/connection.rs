use std::{
    net::SocketAddr,
    time::{Duration, Instant},
};

use crate::{Packet, SeqNumber, SocketID};

pub struct Connection {
    pub settings: ConnectionSettings,
    pub hs_returner: HandshakeReturner,
}

#[derive(Debug, Clone, Copy)]
pub struct ConnectionSettings {
    /// The remote socket to send & receive to
    pub remote: SocketAddr,

    /// The socket id of the UDT entity on the other side
    pub remote_sockid: SocketID,

    /// The local UDT socket id
    pub local_sockid: SocketID,

    /// The time that this socket started at, used to develop timestamps
    pub socket_start_time: Instant,

    /// The first sequence number
    pub init_seq_num: SeqNumber,

    /// The maximum packet size
    pub max_packet_size: u32,

    /// The maxiumum flow size
    pub max_flow_size: u32,

    /// The TSBPD latency configured by the user.
    /// Not necessarily the actual decided on latency, which
    /// is the max of both side's respective latencies.
    pub tsbpd_latency: Duration,
}

/// handshake returner
/// A handshake packet to return
/// Because this depends on the connection initilization method
/// the sender/receiver needs to have this information.
pub type HandshakeReturner = Box<dyn Fn(&Packet) -> Option<Packet> + Send>;

impl ConnectionSettings {
    /// Timestamp in us
    pub fn get_timestamp(&self, at: Instant) -> i32 {
        let elapsed = at - self.socket_start_time;

        elapsed.as_micros() as i32 // TODO: handle overflow here
    }

    /// Timestamp in us
    pub fn get_timestamp_now(&self) -> i32 {
        self.get_timestamp(Instant::now())
    }
}
