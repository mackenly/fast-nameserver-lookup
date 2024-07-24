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
- Average elapsed time: 779.611µs
- Median elapsed time: 196.077µs
- Fastest elapsed time: 190.618µs
- Slowest elapsed time: 58.193155ms

Custom Rust code:
- Average elapsed time: 4.058976ms
- Median elapsed time: 3.513463ms
- Fastest elapsed time: 3.060153ms
- Slowest elapsed time: 40.41275ms

Dig command:
- Average elapsed time: 20.278525ms
- Median elapsed time: 18.819193ms
- Fastest elapsed time: 18.401163ms
- Slowest elapsed time: 159.959453ms
```
If anyone else goes on this journey, I'd suggest checking out the following resources:
- [dns_lookup crate](https://docs.rs/dns-lookup/latest/dns_lookup/)
- [The TCP/IP Guide](http://www.tcpipguide.com/free/t_DNSMessageHeaderandQuestionSectionFormat.htm)
- [Cloudflare's Docs on DNS Wireformat](https://developers.cloudflare.com/1.1.1.1/encryption/dns-over-https/make-api-requests/dns-wireformat/)
- [This Blog Post](https://implement-dns.wizardzines.com/book/part_1.html)
- [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035)