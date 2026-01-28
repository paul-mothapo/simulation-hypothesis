# Edge Product Guide

If the laws of physics are working against you, you need the right tools to bring your data closer to the user. This guide details the actual products you can use to implement the **Edge Computing Solution** across different platforms.

## 1. AWS (Amazon Web Services)

- **CloudFront:** The primary CDN service for AWS. It caches your content globally at edge locations.
- **Lambda@Edge:** Allows you to run serverless code at edge locations. It's like having a mini-server right next to your user.
- **Global Accelerator:** Optimizes the path to your applications using Amazon's global network, reducing packet loss and jitter.

## 2. Microsoft Azure

- **Azure Front Door:** A modern cloud CDN that provides fast, reliable, and secure access between your users and your applications.
- **Azure Content Delivery Network (CDN):** A global solution for delivering high-bandwidth content by caching it at strategically placed nodes.
- **Azure Functions (Edge):** Provides the ability to execute logic at the periphery, though it is currently more integrated with Front Door.

## 3. Google Cloud Platform (GCP)

- **Cloud CDN:** Leverages Google's global edge network to serve cached content closer to users, significantly reducing latency.
- **Cloud Load Balancing:** Automatically distributes traffic across regions to ensure requests are handled by the closest available healthy server.

## 4. Cloudflare (The Kings of the Edge)

- **Cloudflare Workers:** Executes serverless code across Cloudflare's massive global network. For true performance, Workers are often the gold standard.
- **KV (Key-Value):** A low-latency data store that provides global replication of data for instant access.
- **R2:** Globally distributed object storage with no egress fees, making it easy to store and serve assets anywhere.

## 5. Vercel

- **Edge Functions:** Runs your logic in the region physically closest to the incoming request.
- **Edge Config:** A global, low-latency configuration store for feature flags and dynamic settings.
- **Vercel CDN:** An integrated global infrastructure that automatically optimizes asset delivery without manual configuration.

## 6. Self-Hosted / VPS (The Architect's Way)

If you prefer full control over your infrastructure, you can build your own edge:

- **Nginx / Varnish Cache:** Deploy a VPS in a local hub (like Johannesburg/Teraco) and configure it as a **Reverse Proxy** to cache data from your origin in NYC.
- **Anycast IP:** A more advanced networking technique where one IP address is routed to multiple physical servers, ensuring users hit the nearest node.
- **GeoDNS:** Configure your DNS servers (like BIND or PowerDNS) to detect the user's location and return the IP address of your closest server.

---

### Conclusion

There are many ways to fight the speed of light. Choose the tool that fits your budget and technical stack, but never ignore the geography of your data.

**Physics doesn't care about your clean code, it only cares about how many kilometers the light has to travel.** ;^)
