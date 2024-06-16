# fast-nameserver-lookup
Rust-based nameserver lookup that's faster and more efficient than dig

Average time over 100 iterations of looking up NS records for mackenly.com:
```
Rust code:
- Average elapsed time: 1.066939ms

dig command:
- Average elapsed time: 19.637505ms
```
Rust code:
- Average elapsed time: 1.22474ms

dig command:
- Average elapsed time: 20.330499ms
``

If run on Windows, uses nslookup instead of dig for the comparison.