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
- Average elapsed time: 344.04µs
- Median elapsed time: 254.1µs
- Fastest elapsed time: 140.1µs
- Slowest elapsed time: 8.4681ms

Custom Rust code:
- Average elapsed time: 25.230445ms
- Median elapsed time: 23.193ms
- Fastest elapsed time: 20.9253ms
- Slowest elapsed time: 57.5296ms

Dig command:
- Average elapsed time: 2.058687189s
- Median elapsed time: 2.054287s
- Fastest elapsed time: 2.0452169s
- Slowest elapsed time: 2.3432059s
```

The crate returns IP addresses, while the custom code returns the domain names of the nameservers. These are not directly comparable. For my purposes the domain names are more useful. See #4 for more info.

If anyone else goes on this journey, I'd suggest checking out the following resources:
- [dns_lookup crate](https://docs.rs/dns-lookup/latest/dns_lookup/)
- [The TCP/IP Guide](http://www.tcpipguide.com/free/t_DNSMessageHeaderandQuestionSectionFormat.htm)
- [Cloudflare's Docs on DNS Wireformat](https://developers.cloudflare.com/1.1.1.1/encryption/dns-over-https/make-api-requests/dns-wireformat/)
- [This Blog Post](https://implement-dns.wizardzines.com/book/part_1.html)
- [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035)