use std::collections::{HashMap, BinaryHeap, VecDeque, HashSet};
use std::cmp::Ordering;

// The laws of the universe (no breaking these!)
const SPEED_OF_LIGHT: f64 = 299_792_458.0; // Speed of light in vacuum
const FIBER_REFRACTIVE_INDEX: f64 = 1.47; // Light is slower in glass
const SPEED_IN_FIBER: f64 = SPEED_OF_LIGHT / FIBER_REFRACTIVE_INDEX;
const PATH_INEFFICIENCY_FACTOR: f64 = 1.3; // Cables follow roads/sea-beds, not straight lines (30% longer)

#[derive(Debug, Clone)]
struct GeoLocation {
    latitude: f64,
    longitude: f64,
    name: String,
}

impl GeoLocation {
    // Math for measuring how far things are on a giant blue ball (Haversine formula)
    fn distance_to(&self, other: &GeoLocation) -> f64 {
        const EARTH_RADIUS: f64 = 6_371_000.0; // Big rock radius
        
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
struct Server {
    id: usize,
    location: GeoLocation,
    processing_delay: f64, // Nap time for the CPU
    bandwidth: f64, // Data firehose width
}

#[derive(Debug, Clone)]
struct Client {
    id: usize,
    location: GeoLocation,
}

#[derive(Debug, Clone)]
struct DataPacket {
    id: usize,
    source_id: usize,
    destination_id: usize,
    size_bytes: usize,
    created_at: f64,
}

#[derive(Debug, Clone)]
struct NetworkLink {
    from: usize,
    to: usize,
    distance: f64, // How many steps to get there
    latency: f64, // Photon travel time
    bandwidth: f64, // bits per second
}

impl NetworkLink {
    fn new(from: usize, to: usize, distance: f64, bandwidth: f64) -> Self {
        // Here's the realism: Great Circle Distance * Inefficiency Factor
        let real_world_distance = distance * PATH_INEFFICIENCY_FACTOR;
        let latency = real_world_distance / SPEED_IN_FIBER;
        Self {
            from,
            to,
            distance: real_world_distance,
            latency,
            bandwidth,
        }
    }
    
    fn transmission_time(&self, packet_size_bytes: usize) -> f64 {
        let bits = packet_size_bytes as f64 * 8.0;
        bits / self.bandwidth
    }
}

#[derive(Debug, Clone)]
struct Event {
    time: f64,
    packet: DataPacket,
    event_type: EventType,
}

#[derive(Debug, Clone, PartialEq)]
enum EventType {
    PacketArrival(usize), // arrives at node id
    PacketTransmissionComplete(usize), // finished transmitting to node id
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
        // We want the earliest events first, so we're flipping the priority!
        other.time.partial_cmp(&self.time).unwrap_or(Ordering::Equal)
    }
}

struct NetworkSimulation {
    servers: HashMap<usize, Server>,
    clients: HashMap<usize, Client>,
    links: Vec<NetworkLink>,
    event_queue: BinaryHeap<Event>,
    current_time: f64,
    completed_packets: Vec<(DataPacket, f64)>, // Tracking our data's world tour
}

impl NetworkSimulation {
    fn new() -> Self {
        Self {
            servers: HashMap::new(),
            clients: HashMap::new(),
            links: Vec::new(),
            event_queue: BinaryHeap::new(),
            current_time: 0.0,
            completed_packets: Vec::new(),
        }
    }
    
    fn add_server(&mut self, server: Server) {
        self.servers.insert(server.id, server);
    }
    
    fn add_client(&mut self, client: Client) {
        self.clients.insert(client.id, client);
    }
    
    fn connect_nodes(&mut self, from_id: usize, to_id: usize, bandwidth: f64) {
        let distance = self.calculate_distance(from_id, to_id);
        let link = NetworkLink::new(from_id, to_id, distance, bandwidth);
        
        println!(
            "Linking {} ↔ {} | Physical Gap: {:.0} km | Actual Fiber: {:.0} km | Min RTT: {:.2} ms",
            self.get_node_name(from_id),
            self.get_node_name(to_id),
            distance / 1000.0,
            link.distance / 1000.0,
            (link.latency * 2.0) * 1000.0 // RTT is double the one-way latency
        );
        
        self.links.push(link);
    }
    
    fn calculate_distance(&self, from_id: usize, to_id: usize) -> f64 {
        let from_loc = self.get_node_location(from_id);
        let to_loc = self.get_node_location(to_id);
        from_loc.distance_to(to_loc)
    }

    fn get_node_location(&self, id: usize) -> &GeoLocation {
        if let Some(server) = self.servers.get(&id) {
            &server.location
        } else if let Some(client) = self.clients.get(&id) {
            &client.location
        } else {
            panic!("Node {} not found", id);
        }
    }

    fn get_node_name(&self, id: usize) -> String {
        if let Some(s) = self.servers.get(&id) {
            s.location.name.clone()
        } else if let Some(c) = self.clients.get(&id) {
            c.location.name.clone()
        } else {
            format!("Node {}", id)
        }
    }

    fn find_next_hop(&self, from: usize, to: usize) -> Option<usize> {
        // Very basic BFS to find the first step towards the destination
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
    
    fn send_packet(&mut self, from: usize, to: usize, size_bytes: usize) {
        let packet = DataPacket {
            id: self.completed_packets.len() + self.event_queue.len(),
            source_id: from,
            destination_id: to,
            size_bytes,
            created_at: self.current_time,
        };
        
        if let Some(next_hop) = self.find_next_hop(from, to) {
            if let Some(link) = self.links.iter().find(|l| l.from == from && l.to == next_hop) {
                let transmission_time = link.transmission_time(size_bytes);
                let arrival_time = self.current_time + link.latency + transmission_time;
                
                self.event_queue.push(Event {
                    time: arrival_time,
                    packet,
                    event_type: EventType::PacketArrival(next_hop),
                });
            }
        } else {
            println!("No path found from {} to {}!", self.get_node_name(from), self.get_node_name(to));
        }
    }
    
    fn run_simulation(&mut self, duration: f64) {
        println!("\n=== Starting Network Simulation ===");
        println!("Speed of light in vacuum: {:.0} m/s", SPEED_OF_LIGHT);
        println!("Speed in fiber optic: {:.0} m/s ({:.1}% of c)", 
                 SPEED_IN_FIBER, 
                 (SPEED_IN_FIBER / SPEED_OF_LIGHT) * 100.0);
        println!("\n");
        
        while let Some(event) = self.event_queue.pop() {
            if event.time > duration {
                break;
            }
            
            self.current_time = event.time;
            
            match event.event_type {
                EventType::PacketArrival(node_id) => {
                    let node_name = self.get_node_name(node_id);
                    if node_id == event.packet.destination_id {
                        let latency = self.current_time - event.packet.created_at;
                        println!(
                            "[{:.4}s] Packet {} (from {}) arrived at final destination {} ({}) | Travel time: {:.2} ms",
                            self.current_time,
                            event.packet.id,
                            self.get_node_name(event.packet.source_id),
                            node_id,
                            node_name,
                            latency * 1000.0
                        );
                        self.completed_packets.push((event.packet, latency));
                    } else {
                        // Not there yet! Let's see if this node can pass it on.
                        let processing_delay = self.servers.get(&node_id).map(|s| s.processing_delay).unwrap_or(0.0);
                        
                        // Schedule the next hop transmission
                        self.event_queue.push(Event {
                            time: self.current_time + processing_delay,
                            packet: event.packet,
                            event_type: EventType::PacketTransmissionComplete(node_id),
                        });
                    }
                }
                EventType::PacketTransmissionComplete(node_id) => {
                    if let Some(next_hop) = self.find_next_hop(node_id, event.packet.destination_id) {
                        if let Some(link) = self.links.iter().find(|l| l.from == node_id && l.to == next_hop) {
                            let transmission_time = link.transmission_time(event.packet.size_bytes);
                            let arrival_time = self.current_time + link.latency + transmission_time;
                            
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
    
    fn analyze_results(&self) {
        println!("\n=== Simulation Results ===");
        println!("Total packets delivered: {}", self.completed_packets.len());
        
        if !self.completed_packets.is_empty() {
            let latencies: Vec<f64> = self.completed_packets
                .iter()
                .map(|(_, latency)| latency * 1000.0)
                .collect();
            
            let avg_latency = latencies.iter().sum::<f64>() / latencies.len() as f64;
            let min_latency = latencies.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let max_latency = latencies.iter().fold(0.0f64, |a, &b| a.max(b));
            
            let total_capacity: f64 = self.servers.values().map(|s| s.bandwidth).sum();
            let total_fiber_length: f64 = self.links.iter().map(|l| l.distance).sum();

            println!("Network backbone capacity: {:.2} Gbps", total_capacity / 1_000_000_000.0);
            println!("Total fiber optics deployed: {:.0} km", total_fiber_length / 1000.0);
            println!("Average latency: {:.2} ms", avg_latency);
            println!("Min latency: {:.2} ms", min_latency);
            println!("Max latency: {:.2} ms", max_latency);
        }
    }
}

fn main() {
    let mut sim = NetworkSimulation::new();
    
    // --- Servers (The Powerhouses) ---
    
    // Johannesburg (JHB) - The central hub of SA
    sim.add_server(Server {
        id: 1,
        location: GeoLocation {
            latitude: -26.2041,
            longitude: 28.0473,
            name: "Johannesburg (JHB)".to_string(),
        },
        processing_delay: 0.0005,
        bandwidth: 100_000_000_000.0, // 100 Gbps core
    });
    
    // Cape Town (CPT) - The coastal gateway
    sim.add_server(Server {
        id: 2,
        location: GeoLocation {
            latitude: -33.9249,
            longitude: 18.4241,
            name: "Cape Town (CPT)".to_string(),
        },
        processing_delay: 0.0005,
        bandwidth: 100_000_000_000.0,
    });
    
    // London (LDN) - European connection point
    sim.add_server(Server {
        id: 3,
        location: GeoLocation {
            latitude: 51.5074,
            longitude: -0.1278,
            name: "London (LDN)".to_string(),
        },
        processing_delay: 0.0008,
        bandwidth: 200_000_000_000.0,
    });

    // Frankfurt (FRA) - Another major EU hub
    sim.add_server(Server {
        id: 4,
        location: GeoLocation {
            latitude: 50.1109,
            longitude: 8.6821,
            name: "Frankfurt (FRA)".to_string(),
        },
        processing_delay: 0.0008,
        bandwidth: 200_000_000_000.0,
    });

    // New York (NYC) - US East Gateway
    sim.add_server(Server {
        id: 5,
        location: GeoLocation {
            latitude: 40.7128,
            longitude: -74.0060,
            name: "New York (NYC)".to_string(),
        },
        processing_delay: 0.0006,
        bandwidth: 200_000_000_000.0,
    });

    // San Francisco (SFO) - US West Hub
    sim.add_server(Server {
        id: 6,
        location: GeoLocation {
            latitude: 37.7749,
            longitude: -122.4194,
            name: "San Francisco (SFO)".to_string(),
        },
        processing_delay: 0.0006,
        bandwidth: 200_000_000_000.0,
    });

    // Tokyo (NRT) - Asia Hub
    sim.add_server(Server {
        id: 7,
        location: GeoLocation {
            latitude: 35.6762,
            longitude: 139.6503,
            name: "Tokyo (NRT)".to_string(),
        },
        processing_delay: 0.0007,
        bandwidth: 150_000_000_000.0,
    });

    // Singapore (SIN) - SEA Gateway
    sim.add_server(Server {
        id: 8,
        location: GeoLocation {
            latitude: 1.3521,
            longitude: 103.8198,
            name: "Singapore (SIN)".to_string(),
        },
        processing_delay: 0.0007,
        bandwidth: 150_000_000_000.0,
    });
    
    // --- Clients (The People) ---
    
    sim.add_client(Client {
        id: 100,
        location: GeoLocation {
            latitude: -25.7479,
            longitude: 28.2293,
            name: "Pretoria User (PTA)".to_string(),
        },
    });
    
    // --- Network Topology (The Infrastructure) ---
    
    let local_bandwidth = 10_000_000_000.0; // 10 Gbps
    let subsea_bandwidth = 40_000_000_000.0; // 40 Gbps
    
    // Local SA connections
    sim.connect_nodes(100, 1, local_bandwidth); // PTA to JHB
    sim.connect_nodes(1, 2, local_bandwidth);   // JHB to CPT
    
    // International connections (The long haul)
    sim.connect_nodes(2, 3, subsea_bandwidth);  // CPT to LDN (WACS/Equiano)
    sim.connect_nodes(3, 4, subsea_bandwidth);  // LDN to FRA
    sim.connect_nodes(3, 5, subsea_bandwidth);  // LDN to NYC (Atlantic Crossing)
    sim.connect_nodes(5, 6, subsea_bandwidth);  // NYC to SFO (Transcontinental)
    sim.connect_nodes(6, 7, subsea_bandwidth);  // SFO to NRT (Pacific Crossing)
    sim.connect_nodes(7, 8, subsea_bandwidth);  // NRT to SIN
    sim.connect_nodes(3, 8, subsea_bandwidth);  // LDN to SIN (Eurasian route)
    
    // Return paths for global routing
    sim.connect_nodes(8, 7, subsea_bandwidth);
    sim.connect_nodes(7, 6, subsea_bandwidth);
    sim.connect_nodes(6, 5, subsea_bandwidth);
    sim.connect_nodes(5, 3, subsea_bandwidth);
    sim.connect_nodes(8, 3, subsea_bandwidth);
    sim.connect_nodes(4, 3, subsea_bandwidth);
    sim.connect_nodes(3, 2, subsea_bandwidth);
    sim.connect_nodes(2, 1, local_bandwidth);
    sim.connect_nodes(1, 100, local_bandwidth);

    println!("\n--- Sending multi-hop packets ---");

    // Pretoria user fetching data from around the globe
    sim.send_packet(100, 1, 1500); // PTA -> JHB (Local)
    sim.send_packet(100, 3, 1500); // PTA -> LDN (Europe)
    sim.send_packet(100, 5, 1500); // PTA -> NYC (America East)
    sim.send_packet(100, 6, 1500); // PTA -> SFO (America West)
    sim.send_packet(100, 8, 1500); // PTA -> SIN (Asia via Europe)
    sim.send_packet(100, 7, 1500); // PTA -> NRT (Asia via America)
    
    // Hit the big red button!
    sim.run_simulation(1.0); // 1 second of simulation time
    sim.analyze_results();
    
    println!("\n=== Physics Insight ===");
    println!("1. The speed of light in vacuum is a hard limit: 299,792 km/s.");
    println!("2. In fiber (glass), light is slowed down by the refractive index (~1.47) to ~204,000 km/s.");
    println!("3. We applied a {:.0}% 'winding' factor because cables follow sea-beds and land routes.", (PATH_INEFFICIENCY_FACTOR - 1.0) * 100.0);
    println!("\nConclusion: Even with infinite bandwidth, the signal from JHB to Tokyo cannot arrive faster than {:.1}ms due to the physical distance and the speed of light.", (sim.calculate_distance(1, 7) / SPEED_OF_LIGHT) * 1000.0);
}