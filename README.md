# fast-nameserver-lookup
Experiments and benchmarking to compare the speed of a Rust-based nameserver lookup to dig

Three things are tested:
- The speed of the [`dns_lookup` crate](https://docs.rs/dns-lookup/latest/dns_lookup/) in Rust. Nothing magic or custom here.
- The speed of a custom Rust implementation of a nameserver lookup. This was made by me and might not follow spec and doesn't seem to be as fast as the `dns_lookup` crate, which uses C bindings. The old adage of "don't reinvent the wheel" rings true.
- The speed of the `dig` command on a Linux machine. This is a common tool for looking up nameservers. If run on Windows, uses `nslookup` instead.

> [!NOTE]
> Results are highly dependent on the machine and network conditions. Below, you'll find the results a GitHub Actions runner produced.

Average time over 100 iterations of looking up NS records for mackenly.com:
```
Rust lib code:
- Average elapsed time: 8.561265ms
- Median elapsed time: 233.028µs
- Fastest elapsed time: 192.491µs
- Slowest elapsed time: 83.594805ms

Custom Rust code:
- Average elapsed time: 19.971246ms
- Median elapsed time: 19.44533ms
- Fastest elapsed time: 18.708635ms
- Slowest elapsed time: 25.55777ms

Dig command:
- Average elapsed time: 39.872143ms
- Median elapsed time: 20.466018ms
- Fastest elapsed time: 19.989352ms
- Slowest elapsed time: 215.045065ms
```
