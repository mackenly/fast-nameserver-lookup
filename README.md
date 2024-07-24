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
- Average elapsed time: 1.18287ms
- Median elapsed time: 360.9µs
- Fastest elapsed time: 274.8µs
- Slowest elapsed time: 8.757ms

Custom Rust code:
- Average elapsed time: 24.05256ms
- Median elapsed time: 23.5126ms
- Fastest elapsed time: 21.5567ms
- Slowest elapsed time: 27.2018ms

Dig command:
- Average elapsed time: 2.05370638s
- Median elapsed time: 2.0551955s
- Fastest elapsed time: 2.0470218s
- Slowest elapsed time: 2.0593575s
```
