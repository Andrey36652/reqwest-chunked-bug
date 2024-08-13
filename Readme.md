# Chunked gzip response reading bug in Reqwest

Run this with `RUST_LOG=trace` to see incorrect behaviour: all three HTTP requests being finished immediately and each of them creates new connection.

Expected behaviour: HTTP requests should be executed serially, each should take approximately 4 (see `src/server.rs`) seconds, all requests should reuse same connection.

In order to see expected behavior uncomment line in `src/client.rs`.