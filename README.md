# udp2sse

[![Release](https://img.shields.io/github/v/release/riccardotornesello/udp2sse)](https://github.com/riccardotornesello/udp2sse/releases)
[![License](https://img.shields.io/github/license/riccardotornesello/udp2sse)](LICENSE)

A lightweight, high-performance bridge between UDP streams and Server-Sent Events (SSE), written in Rust. Perfect for real-time data streaming applications where you need to forward UDP packets to web clients via HTTP.

## Features

- 🚀 **High Performance**: Built with Rust and Actix-web for maximum throughput and minimal latency
- 🔄 **Real-time Streaming**: Instantly forwards UDP packets to all connected SSE clients
- 📦 **Base64 Encoding**: Automatic base64 encoding of binary data for safe transmission over HTTP
- 🔌 **Simple Integration**: Drop-in solution requiring no configuration files
- 💪 **Production Ready**: Automatic client cleanup, connection management, and error handling
- 🌐 **Cross-platform**: Pre-built binaries for Linux (x86_64, ARM64) and Windows

## Use Cases

- IoT sensor data streaming to web dashboards
- Real-time telemetry and monitoring systems
- Game server state broadcasting to web clients
- Network packet forwarding to browser-based tools
- Live data visualization and analytics

## Quick Start

### Download Pre-built Binary

Download the latest release for your platform from the [releases page](https://github.com/riccardotornesello/udp2sse/releases):

- **Linux x86_64**: `udp2sse-x86_64-unknown-linux-gnu.tar.gz`
- **Linux ARM64**: `udp2sse-aarch64-unknown-linux-gnu.tar.gz`
- **Windows x86_64**: `udp2sse-x86_64-pc-windows-gnu.tar.gz`

Extract and run:

```bash
tar -xzf udp2sse-*.tar.gz
./udp2sse
```

### Build from Source

Requirements: Rust 1.70 or newer

```bash
git clone https://github.com/riccardotornesello/udp2sse.git
cd udp2sse
cargo build --release
./target/release/udp2sse
```

## Usage

Simply run the binary:

```bash
./udp2sse
```

The application will start two services:

- **UDP Listener**: `0.0.0.0:34254` - Receives UDP packets
- **SSE Server**: `http://0.0.0.0:8000/events` - Streams data to HTTP clients

### Sending UDP Data

Send UDP packets to port 34254. Example using netcat:

```bash
echo "Hello World" | nc -u localhost 34254
```

Or using Python:

```python
import socket

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.sendto(b"Hello World", ("localhost", 34254))
```

### Receiving SSE Data

Connect to the SSE endpoint at `http://localhost:8000/events`:

**Using curl:**

```bash
curl -N http://localhost:8000/events
```

**Using JavaScript:**

```javascript
const eventSource = new EventSource('http://localhost:8000/events');

eventSource.addEventListener('connected', (e) => {
  console.log('Connected to stream');
});

eventSource.addEventListener('data', (e) => {
  // Data is base64-encoded
  const decoded = atob(e.data);
  console.log('Received:', decoded);
});

eventSource.addEventListener('ping', (e) => {
  console.log('Ping received (keepalive)');
});
```

## SSE Event Types

The server emits three types of events:

| Event | Description |
|-------|-------------|
| `connected` | Sent immediately when a client connects |
| `data` | Contains base64-encoded UDP packet data |
| `ping` | Keepalive event sent every 20 seconds |

## Configuration

Currently, the application uses hardcoded configuration:

- **UDP Address**: `0.0.0.0:34254`
- **SSE Address**: `0.0.0.0:8000`
- **Ping Interval**: 20 seconds

To modify these values, edit `src/main.rs` and rebuild:

```rust
const PING_INTERVAL: u64 = 20;
const SSE_ADDRESS: &str = "0.0.0.0:8000";
const LISTENER_ADDRESS: &str = "0.0.0.0:34254";
```

## Architecture

```
UDP Packets (port 34254)
         ↓
    UDP Listener
         ↓
    Broadcaster
         ↓
    ┌────┴────┬────────┬────────┐
    ↓         ↓        ↓        ↓
  Client1  Client2  Client3  Client4
    (SSE connections on port 8000)
```

- **Listener**: Receives UDP packets and forwards to broadcaster
- **Broadcaster**: Manages SSE clients and broadcasts data to all connected clients
- **Automatic Cleanup**: Stale clients are removed every 20 seconds via ping/pong

## Examples

See the [examples](./examples) directory for complete working examples:

- **Python UDP Sender**: Send test data via UDP
- **HTML SSE Client**: Browser-based real-time display
- **Shell Scripts**: Simple curl-based examples

## Troubleshooting

### Port Already in Use

If you get a "port already in use" error, another process is using port 8000 or 34254. Find and stop the process:

```bash
# Linux/Mac
sudo lsof -i :8000
sudo lsof -i :34254

# Windows
netstat -ano | findstr :8000
netstat -ano | findstr :34254
```

### No Data Received

1. Verify UDP packets are being sent to the correct port (34254)
2. Check firewall settings allow UDP traffic on port 34254
3. Ensure SSE client is connected to the correct endpoint (`/events`)

### CORS Issues

If accessing from a web browser on a different origin, you may need to add CORS headers. This is not currently supported but can be added by modifying `src/main.rs`.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Credits

Built with:
- [Actix Web](https://actix.rs/) - Powerful, pragmatic, and extremely fast web framework for Rust
- [Actix Web Lab](https://github.com/robbert-vdh/actix-web-lab) - SSE support for Actix Web

## Support

If you find this project helpful, please consider giving it a ⭐ on GitHub!
