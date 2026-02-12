# Simulation Hypothesis

This is actually based of a **QUESTION**, I had during testing, on why after so many optimizations we still hit an average of 350ms to get data from a server in US-CENTRAL. Why does it feel so slow even if the code is clean?

This project is a physics-based proof of a fundamental theory in **distributed systems**: that `server location is directly proportional to network performance`. In modern computing, we often treat the internet as an abstract, magical cloud, but this simulation strips away the marketing fluff to reveal the cold, hard physical reality. Information propagation is not instantaneous; it is strictly governed by the speed of light, the refractive index of glass, and the fact that the Earth is a big rock that we can't just drill through to save a few milliseconds.

The engine uses the **Haversine formula** to `calculate true Great Circle distances between coordinates and applies a 1.47 refractive index to account for how light slows down as it crawls through silica fiber optic cables`. To keep things realistic, we’ve factored in a 30% "winding" modifier. This accounts for the fact that real-world cables don't follow a perfect straight line; they have to deal with annoying things like sea-beds, mountains, and bureaucrats who won't let us lay fiber through their backyards.

We've taken it further to prove the **TCP Handshake tax**. Even if you only want 1 byte of data, the universe makes you pay a "round-trip" fee. You have to send a SYN, wait for a SYN-ACK, and send an ACK before the server even starts sending your actual data. For a user in Pretoria hitting a server in New York, that signal has to cross the Atlantic three times just to say hello. This is why no amount of code optimization can fix that initial lag, it's a physical law.

Another legit law we've added is **Queuing Theory** or the "Bufferbloat" effect. It turns out that having a fat 1Gbps pipe doesn't matter if there's a traffic jam in the fiber. If you send a burst of data, the first packet gets there at light-speed, but the 10th one is stuck waiting for the others to clear. This simulation proves that "bandwidth" and "latency" are two different beasts, and one person downloading a big file can spike everyone else's pings to the moon.

The data generated here for a user based in **Pretoria (PTA)** proves the theory beyond doubt. When fetching data from a local hub in Johannesburg, the response is basically an "instant" high-five because the physical distance is tiny. But the moment that same request tries to visit London, New York, or Tokyo, the latency spikes faster than your heart rate during a production outage. It turns out, photons have a speed limit, and they don't care how much you're paying for your internet.

Looking at the results, you can see the absolute floor of performance. Even if you had infinite bandwidth and NASA-grade routing, a user in Pretoria will always be waiting hundreds of milliseconds to pull memes from **Tokyo or San Francisco**. Why? Because the universe says so. This confirms that "physical distance" is the ultimate bottleneck. It isn't just a design choice for your cloud architect; it’s a law of physics. If you want faster pings, don't write better code—move your house closer to the server. Or move the planet.

## The Edge Computing

Since we can't move the planet (yet), we use **Edge Computing**. This simulation proves that the only way to beat the physics of long-distance networking is to not go the distance at all.

By adding an **Edge Node (CDN)** in Johannesburg, we cache the data from the New York origin server. When a user in Pretoria requests that data, they only travel **54km** instead of **12,800km**.

**The result?** Latency drops from a painful **~160ms** round-trip to New York down to a lightning-fast **~0.7ms** from the local edge. This is why tools like Cloudflare, Akamai, and AWS CloudFront are not just "nice to have" they are physical necessities for a fast internet.

## Earth ↔ Moon Cloud (Dysporium)

If we put a data center on the Moon, we are *not* escaping physics, we are moving our compute to a place where physics has new rules. The speed of light still applies, and Earth ↔ Moon is far enough that even perfect vacuum links take **seconds**.

### Reality Check (Physics Bound)
- The Moon's distance varies from **perigee** to **apogee**, so latency varies with the orbit.
- Even with perfect free-space propagation, Earth ↔ Moon one-way light time is roughly **1.18 to 1.33 seconds**, and round-trip time is **~2.36 to 2.66 seconds**.
- Translation: "very low latency" can only mean low *interactive* delay by avoiding round-trips, not beating light-speed.

### The Mitigation Approach (What Actually Helps)
- **Dysporium Lunar Center** (Moon-side compute): run storage, caching, compression, aggregation, and AI processing locally so only results travel back to Earth.
- **Dysporium Lunar Communicator** (relay network): use a lunar relay to keep line-of-sight coverage even when the Moon's far side faces away from Earth. This is aligned with real-world relay programs such as NASA's LCRNS and the LunaNet framework, and ESA's Moonlight/Lunar Pathfinder communications relay.
- **Delay-Tolerant Networking (DTN)**: use protocols built for long RTTs and intermittent connectivity (Bundle Protocol v7 + LTP). This avoids the cost of repeated handshakes and lets the network store-and-forward reliably.
- **Reduce handshake round-trips** on IP links when possible (e.g., QUIC 0-RTT or TCP Fast Open).
- **Batch, prefetch, and schedule**: move bulk data in planned windows; avoid chatty request/response patterns.

### Extended Theory (Now Simulated In Code)
The project now extends the Earth ↔ Moon scenario with 3 extra models:

1. **Orbital Dynamics Over Time**
- Instead of fixed min/avg/max values only, the simulation sweeps a lunar month and prints how latency changes as the Moon moves from perigee to apogee.
- This shows the RTT swing over time, not just one static number.

2. **Line-of-Sight Outages + Relay Tradeoff**
- The simulation models a lunar site near the far-side limb and checks visibility over time.
- It compares:
  - **Direct Earth link**: lower one-way delay but partial coverage.
  - **Relay-assisted link**: higher uptime, with extra path delay.
- This captures the real engineering tradeoff: availability vs pure latency.

3. **Protocol Startup Comparison**
- The simulation compares startup delay to first response byte for:
  - TCP + TLS 1.2
  - TCP + TLS 1.3
  - QUIC 1-RTT
  - QUIC 0-RTT
  - DTN/LTP scheduled-delivery behavior
- This demonstrates why round-trip reduction matters more than raw bandwidth on deep-space links.

These outputs are printed under:
- `Extension 1: Orbital Dynamics Over Time`
- `Extension 2: Line-of-Sight Outages and Relay Impact`
- `Extension 3: Protocol Startup Comparison`

### Bottom Line
The only way to make Earth ↔ Moon feel "fast" is to **minimize interactive trips** and **push work to the Moon**. Physics sets the floor; architecture is how you live with it.

+++++
To Run this you need to have RUST installed on your machine. Then, you can run the following command:

```bash
from the root directory run
cargo run
```

Look at the terminal for the results.

THANK YOU FOR YOUR ATTENTION ON THIS,

PAUL MOTHAPO ;^)
