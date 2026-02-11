use crate::network_core::SPEED_OF_LIGHT;

// Mean radii and distances based on NASA fact sheets and NASA Science references.
const EARTH_RADIUS_KM: f64 = 6_371.0;
const MOON_RADIUS_KM: f64 = 1_737.4;
const EARTH_MOON_AVG_KM: f64 = 384_400.0;
const EARTH_MOON_PERIGEE_KM: f64 = 363_300.0;
const EARTH_MOON_APOGEE_KM: f64 = 405_500.0;

fn surface_distance_km(center_distance_km: f64) -> f64 {
    center_distance_km - EARTH_RADIUS_KM - MOON_RADIUS_KM
}

fn one_way_ms(surface_distance_km: f64) -> f64 {
    (surface_distance_km * 1_000.0 / SPEED_OF_LIGHT) * 1_000.0
}

pub fn print_earth_moon_scenario() {
    println!("\n=== Earth -> Moon Cloud Scenario (Dysporium) ===");
    println!("Dysporium Lunar Center: Moon-side compute/cache.");
    println!("Dysporium Lunar Communicator: relay for continuous link coverage.");
    println!("Assumptions: near-side line-of-sight, free-space propagation, no routing detours.");
    println!("Distance varies with lunar perigee/apogee.");

    let min_km = surface_distance_km(EARTH_MOON_PERIGEE_KM);
    let avg_km = surface_distance_km(EARTH_MOON_AVG_KM);
    let max_km = surface_distance_km(EARTH_MOON_APOGEE_KM);

    let min_ow_ms = one_way_ms(min_km);
    let avg_ow_ms = one_way_ms(avg_km);
    let max_ow_ms = one_way_ms(max_km);

    let min_rtt_ms = min_ow_ms * 2.0;
    let avg_rtt_ms = avg_ow_ms * 2.0;
    let max_rtt_ms = max_ow_ms * 2.0;

    // Handshake timing:
    // - Client sees SYN-ACK after 1 RTT
    // - Server considers connection established after 1.5 RTT (SYN + SYN-ACK + ACK)
    let client_ready_min_ms = min_rtt_ms;
    let client_ready_max_ms = max_rtt_ms;
    let server_ready_min_ms = min_rtt_ms * 1.5;
    let server_ready_max_ms = max_rtt_ms * 1.5;

    println!(
        "Surface distance (min/avg/max): {:.0} / {:.0} / {:.0} km",
        min_km, avg_km, max_km
    );
    println!(
        "One-way light time (min/avg/max): {:.0} / {:.0} / {:.0} ms",
        min_ow_ms, avg_ow_ms, max_ow_ms
    );
    println!(
        "RTT (min/avg/max): {:.0} / {:.0} / {:.0} ms",
        min_rtt_ms, avg_rtt_ms, max_rtt_ms
    );
    println!(
        "TCP handshake window: client ready {:.0}-{:.0} ms, server ready {:.0}-{:.0} ms",
        client_ready_min_ms,
        client_ready_max_ms,
        server_ready_min_ms,
        server_ready_max_ms
    );
    println!("Takeaway: even in perfect vacuum, Earth↔Moon latency is measured in seconds.");

    println!("\n--- Mitigation Strategy (Dysporium) ---");
    println!("1) Push compute to the Moon: run storage, caching, aggregation, and AI at Dysporium Lunar Center.");
    println!("2) Keep links continuous: Dysporium Lunar Communicator relays traffic for far-side coverage.");
    println!("3) Minimize round-trips: batch, prefetch, and avoid chatty request/response patterns.");
    println!("4) Use long-delay protocols: DTN/LTP for bulk transfer; 0-RTT where possible on IP links.");
    println!("Result: we cannot beat light-speed, but we can reduce interactive waits by reducing cross‑link trips.");
}
