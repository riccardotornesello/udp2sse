# Examples

This directory contains practical examples demonstrating how to use udp2sse.

## Available Examples

### 1. Python UDP Sender (`udp_sender.py`)

A Python script that sends test data to udp2sse via UDP.

**Requirements**: Python 3.6+

**Usage**:
```bash
python3 udp_sender.py
```

**What it does**:
- Sends test messages to localhost:34254 every second
- Demonstrates how to send both text and binary data
- Includes timestamps for verification

### 2. HTML SSE Client (`sse_client.html`)

A complete web page that receives and displays SSE data from udp2sse in real-time.

**Usage**:
1. Start udp2sse: `./udp2sse`
2. Open `sse_client.html` in a web browser
3. Run the UDP sender to see data appear in the browser

**Features**:
- Real-time message display with timestamps
- Automatic base64 decoding
- Connection status indicator
- Event type visualization (connected, data, ping)

### 3. Shell Script Example (`test.sh`)

A simple bash script demonstrating UDP sending and SSE receiving using standard tools.

**Requirements**: netcat (nc), curl

**Usage**:
```bash
chmod +x test.sh
./test.sh
```

## Full Demo

Run a complete demonstration:

**Terminal 1** - Start udp2sse:
```bash
./udp2sse
```

**Terminal 2** - Send test data:
```bash
python3 examples/udp_sender.py
```

**Terminal 3** - View SSE stream:
```bash
curl -N http://localhost:8000/events
```

Or open `examples/sse_client.html` in a web browser to see a visual interface.

## Integration Examples

### Node.js UDP Sender

```javascript
const dgram = require('dgram');
const client = dgram.createSocket('udp4');

const message = Buffer.from('Hello from Node.js');
client.send(message, 34254, 'localhost', (err) => {
  if (err) console.error(err);
  client.close();
});
```

### Go UDP Sender

```go
package main

import (
    "net"
    "log"
)

func main() {
    conn, err := net.Dial("udp", "localhost:34254")
    if err != nil {
        log.Fatal(err)
    }
    defer conn.Close()
    
    _, err = conn.Write([]byte("Hello from Go"))
    if err != nil {
        log.Fatal(err)
    }
}
```

### React SSE Client

```javascript
import { useEffect, useState } from 'react';

function UdpStream() {
  const [messages, setMessages] = useState([]);
  const [connected, setConnected] = useState(false);

  useEffect(() => {
    const eventSource = new EventSource('http://localhost:8000/events');

    eventSource.addEventListener('connected', () => {
      setConnected(true);
    });

    eventSource.addEventListener('data', (e) => {
      const decoded = atob(e.data);
      setMessages(prev => [...prev, decoded]);
    });

    eventSource.onerror = () => {
      setConnected(false);
    };

    return () => eventSource.close();
  }, []);

  return (
    <div>
      <p>Status: {connected ? 'Connected' : 'Disconnected'}</p>
      <ul>
        {messages.map((msg, i) => (
          <li key={i}>{msg}</li>
        ))}
      </ul>
    </div>
  );
}
```
