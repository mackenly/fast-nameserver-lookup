# fast-nameserver-lookup
Experiments and benchmarking to compare the speed of a Rust-based nameserver lookup to dig

Three things are tested:
- The speed of the [`dns_lookup` crate](https://docs.rs/dns-lookup/latest/dns_lookup/) in Rust. Nothing magic or custom here.
- The speed of a custom Rust implementation of a nameserver lookup. This was made by me and might not follow spec (didn't really try) and doesn't seem to be as fast as the `dns_lookup` crate, which uses C bindings. The old adage of "don't reinvent the wheel" rings true.
- The speed of the `dig` command on a Linux machine. This is a common tool for looking up nameservers. If run on Windows, it uses `nslookup` instead. `nslookup` seems to be much slower than `dig`.

> [!NOTE]
> Results are highly dependent on the machine and network conditions. Below, you'll find the results a GitHub Actions runner produced.

Average time over 100 iterations of looking up NS records for mackenly.com:
```
Rust lib code:
- Average elapsed time: 597.546µs
- Median elapsed time: 194.492µs
- Fastest elapsed time: 190.364µs
- Slowest elapsed time: 40.14657ms

Custom Rust code:
- Average elapsed time: 6.484227ms
- Median elapsed time: 6.000502ms
- Fastest elapsed time: 5.64434ms
- Slowest elapsed time: 42.626151ms

Dig command:
- Average elapsed time: 20.758443ms
- Median elapsed time: 18.886611ms
- Fastest elapsed time: 8.504378ms
- Slowest elapsed time: 285.612794ms
```
If anyone else goes on this journey, I'd suggest checking out the following resources:
- [dns_lookup crate](https://docs.rs/dns-lookup/latest/dns_lookup/)
- [The TCP/IP Guide](http://www.tcpipguide.com/free/t_DNSMessageHeaderandQuestionSectionFormat.htm)
- [Cloudflare's Docs on DNS Wireformat](https://developers.cloudflare.com/1.1.1.1/encryption/dns-over-https/make-api-requests/dns-wireformat/)
- [This Blog Post](https://implement-dns.wizardzines.com/book/part_1.html)
- [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035)