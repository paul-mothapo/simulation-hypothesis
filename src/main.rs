mod network_core;
mod theories;

use network_core::{NetworkSimulation, Server, Client, GeoLocation, SPEED_OF_LIGHT};
use theories::TheoryTests;

fn main() {
    let mut sim = NetworkSimulation::new();
    
    // Setup Nodes
    sim.add_server(Server {
        id: 1,
        location: GeoLocation { latitude: -26.2041, longitude: 28.0473, name: "Johannesburg".to_string() },
        processing_delay: 0.0005,
        bandwidth: 100_000_000_000.0,
    });
    
    sim.add_server(Server {
        id: 5,
        location: GeoLocation { latitude: 40.7128, longitude: -74.0060, name: "New York".to_string() },
        processing_delay: 0.0006,
        bandwidth: 200_000_000_000.0,
    });

    sim.add_client(Client {
        id: 100,
        location: GeoLocation { latitude: -25.7479, longitude: 28.2293, name: "Pretoria".to_string() },
    });

    // Setup Links
    // [Sepedi]: Re kopanya mafelo a go fapafapana
    let bandwidth = 10_000_000_000.0;
    sim.connect_nodes(100, 1, bandwidth); // ADD [Sepedi]: Re kopanya Pretoria le Joburg
    sim.connect_nodes(1, 5, bandwidth);   // ADD [Sepedi]: Re kopanya Joburg le New York (Mošola wa lewatle)
    sim.connect_nodes(5, 1, bandwidth);   // ADD [Sepedi]: Tsela ya go bowa go tšwa NYC
    sim.connect_nodes(1, 100, bandwidth); // ADD [Sepedi]: Go boela gae Pretoria

    // Run Theories
    // [Sepedi]: Re thoma diteko tša rena tša thiori
    TheoryTests::demonstrate_tcp_handshake(&mut sim, 100, 5);
    
    // Demonstrate Bufferbloat (Queuing)
    TheoryTests::demonstrate_bufferbloat(&mut sim, 100, 1);

    sim.run_simulation(2.0);
    sim.analyze_results();

    println!("\n=== Final Physics Takeaway ===");
    let dist = sim.calculate_distance(100, 5);
    println!("Physical Distance PTA -> NYC: {:.0} km", dist / 1000.0);
    println!("Min Theoretical RTT (Vacuum): {:.2} ms", (dist * 2.0 / SPEED_OF_LIGHT) * 1000.0);
    println!("Actual Simulated RTT (Fiber + Winding + Handshake): Shows why you see 350ms+ in the real world.");
}