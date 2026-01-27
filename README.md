# Simulation Hypothesis

This is actually based of a QUESTION, I had during testing, on why after so many optimizations we still hit an average of 350ms to get data from a server in US-CENTRAL.

This project is a physics-based proof of a fundamental theory in **distributed systems**: that `server location is directly proportional to network performance`. In modern computing, we often treat the internet as an abstract, magical cloud, but this simulation strips away the marketing fluff to reveal the cold, hard physical reality. Information propagation is not instantaneous; it is strictly governed by the speed of light, the refractive index of glass, and the fact that the Earth is a big rock that we can't just drill through to save a few milliseconds.

The engine uses the **Haversine formula** to `calculate true Great Circle distances between coordinates and applies a 1.47 refractive index to account for how light slows down as it crawls through silica fiber optic cables`. To keep things realistic, we’ve factored in a 30% "winding" modifier. This accounts for the fact that real-world cables don't follow a perfect straight line; they have to deal with annoying things like sea-beds, mountains, and bureaucrats who won't let us lay fiber through their backyards.

The data generated here for a user based in **Pretoria (PTA)** proves the theory beyond doubt. When fetching data from a local hub in Johannesburg, the response is basically an "instant" high-five because the physical distance is tiny. But the moment that same request tries to visit **London, New York, or Tokyo**, the latency spikes faster than your heart rate during a production outage. It turns out, photons have a speed limit, and they don't care how much you're paying for your 1Gbps connection.

Looking at the results, you can see the absolute floor of performance. Even if you had infinite bandwidth and NASA-grade routing, a user in Pretoria will always be waiting hundreds of milliseconds to pull memes from **Tokyo or San Francisco**. Why? Because the universe says so. This confirms that "physical distance" is the ultimate bottleneck. It isn't just a design choice for your cloud architect; it’s a law of physics. If you want faster pings, you don't need better code—you need to move your house closer to the server. Or move the planet.

+++++
To Run this you need to have RUST installed on your machine. Then, you can run the following command:

```bash
from the root directory run
cargo run
```

Look at the terminal for the results.

THANK YOU FOR YOUR ATTENTION ON THIS,

PAUL MOTHAPO ;^)