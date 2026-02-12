mod network_core;
mod theories;
mod moon_scenario;
mod earth_moon_extensions;

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
    let bandwidth = 10_000_000_000.0;
    sim.connect_nodes(100, 1, bandwidth); 
    sim.connect_nodes(1, 5, bandwidth);   
    sim.connect_nodes(5, 1, bandwidth);   
    sim.connect_nodes(1, 100, bandwidth);

    // Run Theories
    TheoryTests::demonstrate_tcp_handshake(&mut sim, 100, 5);
    sim.run_simulation(1.0); 
    
    // Demonstrate Bufferbloat (Queuing)
    TheoryTests::demonstrate_bufferbloat(&mut sim, 100, 1);
    sim.run_simulation(2.0);
    
    // Demonstrate Edge Computing
    TheoryTests::demonstrate_cdn_solution(&mut sim, 100, 5, 1);
    sim.run_simulation(3.0);

    sim.analyze_results();

    println!("\n=== Final Physics Takeaway ===");
    let dist = sim.calculate_distance(100, 5);
    println!("Physical Distance PTA -> NYC: {:.0} km", dist / 1000.0);
    println!("Min Theoretical RTT (Vacuum): {:.2} ms", (dist * 2.0 / SPEED_OF_LIGHT) * 1000.0);
    println!("Actual Simulated RTT (Fiber + Winding + Handshake): Shows why you see 350ms+ in the real world.");
    
    println!("\n=== The Edge Computing Conclusion ===");
    println!("Physics says light has a speed limit, but economics says we can buy servers closer to users.");
    println!("By using the Johannesburg Edge Node, we reduced the physical path from 12,800km to ~54km.");
    println!("Result: Latency dropped from ~160ms (NYC) to ~0.7ms (JHB Edge).");
    println!("Conclusion: Don't just optimize code; optimize the geography of your data.");

    moon_scenario::print_earth_moon_scenario();
    earth_moon_extensions::print_top_three_extensions();
}
