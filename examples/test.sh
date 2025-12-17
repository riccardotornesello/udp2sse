#!/bin/bash
#
# Simple test script for udp2sse
#
# This script demonstrates basic usage of udp2sse using common command-line tools.
#
# Requirements: netcat (nc), curl
#
# Usage:
#   1. Start udp2sse in another terminal: ./udp2sse
#   2. Run this script: ./test.sh
#

set -e

UDP_HOST="localhost"
UDP_PORT="34254"
SSE_URL="http://localhost:8000/events"

echo "==================================="
echo "udp2sse Test Script"
echo "==================================="
echo ""

# Check if nc is available
if ! command -v nc &> /dev/null; then
    echo "Error: netcat (nc) is not installed"
    echo "Install it with: sudo apt-get install netcat  (Debian/Ubuntu)"
    echo "                 brew install netcat           (macOS)"
    exit 1
fi

# Check if curl is available
if ! command -v curl &> /dev/null; then
    echo "Error: curl is not installed"
    exit 1
fi

echo "Test 1: Send a simple message via UDP"
echo "---------------------------------------"
echo "Sending: 'Hello from test script'"
echo "Hello from test script" | nc -u -w1 $UDP_HOST $UDP_PORT
echo "✓ Message sent"
echo ""

echo "Test 2: Send multiple messages"
echo "---------------------------------------"
for i in {1..5}; do
    echo "Message #$i" | nc -u -w1 $UDP_HOST $UDP_PORT
    echo "Sent: Message #$i"
    sleep 0.5
done
echo "✓ All messages sent"
echo ""

echo "Test 3: Listen to SSE stream (Ctrl+C to stop)"
echo "---------------------------------------"
echo "Connecting to $SSE_URL"
echo "You should see events in real-time. Try sending more UDP data from another terminal!"
echo ""
echo "Try: echo 'Test message' | nc -u $UDP_HOST $UDP_PORT"
echo ""
curl -N $SSE_URL
