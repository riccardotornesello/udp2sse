# udp2sse

This code allows you to create a bridge between a UDP stream and a Sever-Sent-Events stream.

The software remains listening on `0.0.0.0:34254` for incoming UDP packets while serving real time data received on `http://0.0.0.0:8000/events`.

SSE data is base64-encoded and corresponds to the "data" event.

Feel free to contribute. If you just want to download the software, find the compiled binary in the releases section.
