use std::collections::{HashMap, BinaryHeap, VecDeque, HashSet};
use std::cmp::Ordering;

pub const SPEED_OF_LIGHT: f64 = 299_792_458.0;
pub const FIBER_REFRACTIVE_INDEX: f64 = 1.47;
pub const SPEED_IN_FIBER: f64 = SPEED_OF_LIGHT / FIBER_REFRACTIVE_INDEX;
pub const PATH_INEFFICIENCY_FACTOR: f64 = 1.3;

#[derive(Debug, Clone)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub name: String,
}

impl GeoLocation {
    pub fn distance_to(&self, other: &GeoLocation) -> f64 {
        const EARTH_RADIUS: f64 = 6_371_000.0;
        let lat1 = self.latitude.to_radians();
        let lat2 = other.latitude.to_radians();
        let dlat = (other.latitude - self.latitude).to_radians();
        let dlon = (other.longitude - self.longitude).to_radians();
        let a = (dlat / 2.0).sin().powi(2) 
            + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        EARTH_RADIUS * c
    }
}

#[derive(Debug, Clone)]
pub struct Server {
    pub id: usize,
    pub location: GeoLocation,
    pub processing_delay: f64,
    pub bandwidth: f64,
}

#[derive(Debug, Clone)]
pub struct Client {
    pub id: usize,
    pub location: GeoLocation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketType {
    Standard,
    TcpSyn,
    TcpSynAck,
    TcpAck,
    CdnRequest,
    CdnResponse,
}

#[derive(Debug, Clone)]
pub struct DataPacket {
    pub id: usize,
    pub source_id: usize,
    pub destination_id: usize,
    pub size_bytes: usize,
    pub created_at: f64,
    pub packet_type: PacketType,
}

#[derive(Debug, Clone)]
pub struct NetworkLink {
    pub from: usize,
    pub to: usize,
    pub distance: f64,
    pub latency: f64,
    pub bandwidth: f64,
    pub queue_end_time: f64,
}

impl NetworkLink {
    pub fn new(from_id: usize, to_id: usize, dist: f64, bw: f64) -> Self {
        let real_world_distance = dist * PATH_INEFFICIENCY_FACTOR;
        let lat = real_world_distance / SPEED_IN_FIBER;
        Self {
            from: from_id,
            to: to_id,
            distance: real_world_distance,
            latency: lat,
            bandwidth: bw,
            queue_end_time: 0.0,
        }
    }
    
    pub fn transmission_time(&self, size: usize) -> f64 {
        (size as f64 * 8.0) / self.bandwidth
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    pub time: f64,
    pub packet: DataPacket,
    pub event_type: EventType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    PacketArrival(usize),
    PacketTransmissionComplete(usize),
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.partial_cmp(&self.time).unwrap_or(Ordering::Equal)
    }
}

pub struct NetworkSimulation {
    pub servers: HashMap<usize, Server>,
    pub clients: HashMap<usize, Client>,
    pub links: Vec<NetworkLink>,
    pub event_queue: BinaryHeap<Event>,
    pub current_time: f64,
    pub completed_packets: Vec<(DataPacket, f64)>,
}

impl NetworkSimulation {
    pub fn new() -> Self {
        Self {
            servers: HashMap::new(),
            clients: HashMap::new(),
            links: Vec::new(),
            event_queue: BinaryHeap::new(),
            current_time: 0.0,
            completed_packets: Vec::new(),
        }
    }

    pub fn add_server(&mut self, server: Server) {
        self.servers.insert(server.id, server);
    }
    
    pub fn add_client(&mut self, client: Client) {
        self.clients.insert(client.id, client);
    }
    
    pub fn connect_nodes(&mut self, from_id: usize, to_id: usize, bandwidth: f64) {
        let distance = self.calculate_distance(from_id, to_id);
        let link = NetworkLink::new(from_id, to_id, distance, bandwidth);
        
        println!(
            "Linking {} ↔ {} | Physical Gap: {:.0} km | Actual Fiber: {:.0} km | Min RTT: {:.2} ms",
            self.get_node_name(from_id),
            self.get_node_name(to_id),
            distance / 1000.0,
            link.distance / 1000.0,
            (link.latency * 2.0) * 1000.0
        );
        
        self.links.push(link);
    }

    pub fn get_node_name(&self, id: usize) -> String {
        if let Some(s) = self.servers.get(&id) {
            s.location.name.clone()
        } else if let Some(c) = self.clients.get(&id) {
            c.location.name.clone()
        } else {
            format!("Node {}", id)
        }
    }

    pub fn find_next_hop(&self, from: usize, to: usize) -> Option<usize> {
        let mut queue = VecDeque::new();
        queue.push_back((from, None));
        let mut visited = HashSet::new();
        visited.insert(from);

        while let Some((current, first_hop)) = queue.pop_front() {
            if current == to {
                return first_hop;
            }

            for link in self.links.iter().filter(|l| l.from == current) {
                if !visited.contains(&link.to) {
                    visited.insert(link.to);
                    let next_hop = if first_hop.is_none() { Some(link.to) } else { first_hop };
                    queue.push_back((link.to, next_hop));
                }
            }
        }
        None
    }

    pub fn calculate_distance(&self, from_id: usize, to_id: usize) -> f64 {
        let get_loc = |id| {
            if let Some(s) = self.servers.get(&id) { Some(&s.location) }
            else { self.clients.get(&id).map(|c| &c.location) }
        };

        let from_loc = get_loc(from_id).expect("Source node not found");
        let to_loc = get_loc(to_id).expect("Destination node not found");
        
        from_loc.distance_to(to_loc)
    }

    pub fn send_packet_ex(&mut self, from: usize, to: usize, size_bytes: usize, p_type: PacketType) {
        let packet = DataPacket {
            id: self.completed_packets.len() + self.event_queue.len(),
            source_id: from,
            destination_id: to,
            size_bytes,
            created_at: self.current_time,
            packet_type: p_type,
        };
        
        if let Some(next_hop) = self.find_next_hop(from, to) {
            let current_time = self.current_time;
            if let Some(link) = self.links.iter_mut().find(|l| l.from == from && l.to == next_hop) {
                let trans_time = link.transmission_time(size_bytes);
                let start_time = current_time.max(link.queue_end_time);
                let arrival_time = start_time + link.latency + trans_time;
                link.queue_end_time = start_time + trans_time;
                
                self.event_queue.push(Event {
                    time: arrival_time,
                    packet,
                    event_type: EventType::PacketArrival(next_hop),
                });
            }
        }
    }

    pub fn run_simulation(&mut self, duration: f64) {
        while let Some(event) = self.event_queue.pop() {
            if event.time > duration { break; }
            self.current_time = event.time;
            
            match event.event_type {
                EventType::PacketArrival(node_id) => {
                    if node_id == event.packet.destination_id {
                        let latency = self.current_time - event.packet.created_at;
                        println!("[{:.4}s] {:?} packet (ID {}) arrived at {} | Latency: {:.2} ms", 
                                 self.current_time, event.packet.packet_type, event.packet.id, self.get_node_name(node_id), latency * 1000.0);
                        
                        match event.packet.packet_type {
                            PacketType::TcpSyn => {
                                self.send_packet_ex(node_id, event.packet.source_id, 64, PacketType::TcpSynAck);
                            }
                            PacketType::TcpSynAck => {
                                self.send_packet_ex(node_id, event.packet.source_id, 64, PacketType::TcpAck);
                            }
                            PacketType::CdnRequest => {
                                // [Sepedi]: Ge resepi entle ya CDN e fihla, re araba kapee-pee
                                // CDN server responds immediately with the cached data (1KB for demo)
                                self.send_packet_ex(node_id, event.packet.source_id, 1024, PacketType::CdnResponse);
                            }
                            _ => {}
                        }
                        
                        self.completed_packets.push((event.packet, latency));
                    } else {
                        let delay = self.servers.get(&node_id).map(|s| s.processing_delay).unwrap_or(0.0);
                        self.event_queue.push(Event {
                            time: self.current_time + delay,
                            packet: event.packet,
                            event_type: EventType::PacketTransmissionComplete(node_id),
                        });
                    }
                }
                EventType::PacketTransmissionComplete(node_id) => {
                    if let Some(next_hop) = self.find_next_hop(node_id, event.packet.destination_id) {
                        let current_time = self.current_time;
                        let size = event.packet.size_bytes;
                        if let Some(link) = self.links.iter_mut().find(|l| l.from == node_id && l.to == next_hop) {
                            let trans_time = link.transmission_time(size);
                            let start_time = current_time.max(link.queue_end_time);
                            let arrival_time = start_time + link.latency + trans_time;
                            link.queue_end_time = start_time + trans_time;
                            
                            self.event_queue.push(Event {
                                time: arrival_time,
                                packet: event.packet,
                                event_type: EventType::PacketArrival(next_hop),
                            });
                        }
                    }
                }
            }
        }
    }

    pub fn analyze_results(&self) {
        println!("\n=== Simulation Results ===");
        if self.completed_packets.is_empty() { return; }
        
        let avg_latency = self.completed_packets.iter().map(|(_, l)| *l).sum::<f64>() / self.completed_packets.len() as f64;
        let max_lat = self.completed_packets.iter().map(|(_, l)| *l).fold(0.0f64, |a, b| a.max(b));
        
        let total_capacity: f64 = self.servers.values().map(|s| s.bandwidth).sum();
        
        println!("Total delivered: {}", self.completed_packets.len());
        println!("Total Capacity: {:.2} Gbps", total_capacity / 1_000_000_000.0);
        println!("Avg Latency: {:.2} ms", avg_latency * 1000.0);
        println!("Max Latency: {:.2} ms", max_lat * 1000.0);
    }
}