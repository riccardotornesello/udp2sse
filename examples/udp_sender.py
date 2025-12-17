#!/usr/bin/env python3
"""
UDP Sender Example for udp2sse

This script sends test messages to udp2sse via UDP.
Run this after starting the udp2sse server.

Usage:
    python3 udp_sender.py
"""

import socket
import time
from datetime import datetime

# Configuration
UDP_HOST = "localhost"
UDP_PORT = 34254

def send_udp_message(sock, message):
    """Send a message via UDP to the udp2sse server."""
    if isinstance(message, str):
        message = message.encode('utf-8')
    
    sock.sendto(message, (UDP_HOST, UDP_PORT))
    print(f"[{datetime.now().strftime('%H:%M:%S')}] Sent: {message.decode('utf-8', errors='replace')}")

def main():
    print("=== UDP Sender for udp2sse ===")
    print(f"Target: {UDP_HOST}:{UDP_PORT}")
    print("Press Ctrl+C to stop\n")
    
    # Create UDP socket
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    
    try:
        counter = 0
        
        while True:
            counter += 1
            
            # Send different types of messages
            if counter % 5 == 0:
                # Every 5th message: JSON-like data
                message = f'{{"counter": {counter}, "timestamp": "{datetime.now().isoformat()}", "type": "sensor_data"}}'
            elif counter % 3 == 0:
                # Every 3rd message: Metric
                message = f"metric.temperature.value={20 + (counter % 10)}"
            else:
                # Regular message
                message = f"Message #{counter} at {datetime.now().strftime('%H:%M:%S')}"
            
            send_udp_message(sock, message)
            
            # Wait 1 second before sending next message
            time.sleep(1)
            
    except KeyboardInterrupt:
        print("\n\nStopped by user")
    except Exception as e:
        print(f"\nError: {e}")
    finally:
        sock.close()
        print("Socket closed")

if __name__ == "__main__":
    main()
