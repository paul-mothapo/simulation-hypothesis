use crate::network_core::{NetworkSimulation, PacketType};

pub struct TheoryTests;

impl TheoryTests {
    // 1. The "Traffic Jam" proof (Kgopolo ya mola wa dikoloi)
    // Proving that fat pipes don't matter if there's a line at the toll booth
    // [Sepedi]: Go bontšha gore diphaepe tše dikgolo ga di thuse selo ge go na le mola wo motelele wa go letela
    pub fn demonstrate_bufferbloat(sim: &mut NetworkSimulation, source: usize, destination: usize) {
        println!("\n--- [THEORY] Queuing Theory & Bufferbloat ---");
        println!("Scenario: Sending a burst of 10 packets at once. Watch the last one cry.");
        
        // Send 10 chunky packets in the same microsecond
        // [Sepedi]: Re romela diphakete tše lesome ka nako e tee. Ela hloko ya mafelelo ge e diega kudu.
        for _ in 0..10 {
            // 10MB packets to really clog the drain
            sim.send_packet_ex(source, destination, 10_000_000, PacketType::Standard);
        }
    }

    // 2. The "Round-Trip Tax" proof (Tefelo ya leeto la go ya le go boa)
    // Proving that photons hitting a speed limit makes 'saying hello' expensive
    // [Sepedi]: Go bontšha gore go romelana melaetša khomphutheng go tšea nako ka lebaka la maeto a go ya le go boa
    pub fn demonstrate_tcp_handshake(sim: &mut NetworkSimulation, client_id: usize, server_id: usize) {
        println!("\n--- [THEORY] TCP Handshake Overhead ---");
        println!("Scenario: Establishing a TCP connection to a server across the world.");
        
        // We start the SYN dance here.
        // [Sepedi]: Mo re thoma motšhene wa kgokaganyo ya SYN.
        println!(
            "Starting the SYN dance: {} -> {}",
            sim.get_node_name(client_id),
            sim.get_node_name(server_id)
        );
        sim.send_packet_ex(client_id, server_id, 64, PacketType::TcpSyn);
    }
}
