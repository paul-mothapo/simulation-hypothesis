use std::f64::consts::PI;

use crate::network_core::SPEED_OF_LIGHT;

const EARTH_RADIUS_KM: f64 = 6_371.0;
const MOON_RADIUS_KM: f64 = 1_737.4;
const EARTH_MOON_AVG_KM: f64 = 384_400.0;
const EARTH_MOON_PERIGEE_KM: f64 = 363_300.0;
const EARTH_MOON_APOGEE_KM: f64 = 405_500.0;
const ANOMALISTIC_MONTH_DAYS: f64 = 27.55455;

const LIBRATION_LONGITUDE_AMPLITUDE_DEG: f64 = 7.9;
const SIDEREAL_MONTH_DAYS: f64 = 27.321661;

const LUNAR_SITE_LONGITUDE_DEG: f64 = 95.0;

const RELAY_EXTRA_PATH_KM: f64 = 12_000.0;
const RELAY_ASSUMED_UPTIME: f64 = 99.8;

struct ProtocolProfile {
    name: &'static str,
    startup_rtts: f64,
    note: &'static str,
}

fn surface_distance_km(center_distance_km: f64) -> f64 {
    center_distance_km - EARTH_RADIUS_KM - MOON_RADIUS_KM
}

fn one_way_ms(surface_distance_km: f64) -> f64 {
    (surface_distance_km * 1_000.0 / SPEED_OF_LIGHT) * 1_000.0
}

fn orbital_center_distance_km(day: f64) -> f64 {
    let average = (EARTH_MOON_APOGEE_KM + EARTH_MOON_PERIGEE_KM) / 2.0;
    let amplitude = (EARTH_MOON_APOGEE_KM - EARTH_MOON_PERIGEE_KM) / 2.0;
    average - amplitude * ((2.0 * PI * day) / ANOMALISTIC_MONTH_DAYS).cos()
}

fn normalize_degrees(mut degrees: f64) -> f64 {
    while degrees > 180.0 {
        degrees -= 360.0;
    }
    while degrees < -180.0 {
        degrees += 360.0;
    }
    degrees
}

fn sub_earth_longitude_deg(day: f64) -> f64 {
    LIBRATION_LONGITUDE_AMPLITUDE_DEG * ((2.0 * PI * day) / SIDEREAL_MONTH_DAYS).sin()
}

pub fn print_top_three_extensions() {
    print_orbital_dynamics_extension();
    print_line_of_sight_extension();
    print_protocol_comparison_extension();
}

fn print_orbital_dynamics_extension() {
    println!("Orbital Dynamics Over Time");
    println!("Model: monthly Earth-Moon distance variation (perigee <-> apogee).");
    println!("Day | Surface Distance (km) | One-way (ms) | RTT (ms)");

    let mut min_rtt_ms = f64::MAX;
    let mut max_rtt_ms: f64 = 0.0;

    let mut day = 0.0;
    while day <= 27.0 {
        let center_km = orbital_center_distance_km(day);
        let surface_km = surface_distance_km(center_km);
        let one_way = one_way_ms(surface_km);
        let rtt = one_way * 2.0;

        min_rtt_ms = min_rtt_ms.min(rtt);
        max_rtt_ms = max_rtt_ms.max(rtt);

        println!(
            "{:>3.0} | {:>21.0} | {:>11.0} | {:>8.0}",
            day, surface_km, one_way, rtt
        );

        day += 3.0;
    }

    println!(
        "RTT swing over one cycle: {:.0} ms -> {:.0} ms (delta {:.0} ms)",
        min_rtt_ms,
        max_rtt_ms,
        max_rtt_ms - min_rtt_ms
    );
}

fn print_line_of_sight_extension() {
    println!("Line-of-Sight Outages and Relay Impact");
    println!("Lunar site longitude: {:.1} degrees (near far-side limb).", LUNAR_SITE_LONGITUDE_DEG);
    println!("Sim horizon: 28 days sampled hourly.");

    let total_hours = 28 * 24;
    let mut visible_hours = 0usize;
    let mut direct_one_way_sum_ms = 0.0;
    let mut relay_one_way_sum_ms = 0.0;

    for hour in 0..total_hours {
        let day = hour as f64 / 24.0;
        let center_km = orbital_center_distance_km(day);
        let surface_km = surface_distance_km(center_km);
        let one_way_direct_ms = one_way_ms(surface_km);
        let earth_sub_long = sub_earth_longitude_deg(day);
        let separation = normalize_degrees(LUNAR_SITE_LONGITUDE_DEG - earth_sub_long).abs();
        let visible = separation <= 90.0;

        if visible {
            visible_hours += 1;
            direct_one_way_sum_ms += one_way_direct_ms;
        }

        let relay_penalty_ms = one_way_ms(RELAY_EXTRA_PATH_KM);
        relay_one_way_sum_ms += one_way_direct_ms + relay_penalty_ms;
    }

    let direct_uptime = (visible_hours as f64 / total_hours as f64) * 100.0;
    let direct_avg_one_way_ms = if visible_hours > 0 {
        direct_one_way_sum_ms / visible_hours as f64
    } else {
        f64::INFINITY
    };
    let relay_avg_one_way_ms = relay_one_way_sum_ms / total_hours as f64;

    println!(
        "Without relay: uptime {:.1}% | avg one-way when visible: {:.0} ms",
        direct_uptime, direct_avg_one_way_ms
    );
    println!(
        "With relay: uptime {:.1}% | avg one-way: {:.0} ms",
        RELAY_ASSUMED_UPTIME, relay_avg_one_way_ms
    );
    println!(
        "Tradeoff: relay adds ~{:.0} ms one-way but recovers coverage.",
        one_way_ms(RELAY_EXTRA_PATH_KM)
    );
}

fn print_protocol_comparison_extension() {
    println!("Protocol Startup Comparison");
    println!("Baseline distance: average Earth-Moon separation.");

    let surface_km = surface_distance_km(EARTH_MOON_AVG_KM);
    let one_way = one_way_ms(surface_km);
    let rtt = one_way * 2.0;

    let profiles = [
        ProtocolProfile {
            name: "TCP + TLS 1.2",
            startup_rtts: 4.0,
            note: "Most expensive startup path",
        },
        ProtocolProfile {
            name: "TCP + TLS 1.3",
            startup_rtts: 3.0,
            note: "Saves one RTT vs TLS 1.2",
        },
        ProtocolProfile {
            name: "QUIC (1-RTT)",
            startup_rtts: 2.0,
            note: "Transport + crypto combined",
        },
        ProtocolProfile {
            name: "QUIC (0-RTT)",
            startup_rtts: 1.0,
            note: "Fastest interactive startup (replay caveats)",
        },
    ];

    println!("Protocol | Startup RTTs to first response byte | Time (ms)");
    for p in profiles {
        println!(
            "{:<14} | {:>34.1} | {:>8.0}  ({})",
            p.name,
            p.startup_rtts,
            p.startup_rtts * rtt,
            p.note
        );
    }

    let average_contact_wait_ms = 7.5 * 60.0 * 1_000.0;
    let dtn_total_ms = average_contact_wait_ms + one_way;
    println!(
        "DTN/LTP (scheduled contact) | wait + one-way delivery | {:>8.0}  (best for bulk, not chatty RPC)",
        dtn_total_ms
    );
}