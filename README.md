# fast-nameserver-lookup
Rust-based nameserver lookup that's faster and more efficient than dig

Average time over 100 iterations of looking up NS records for mackenly.com:
```
Rust code:
- Elapsed time: 201.803µs

dig command:
- Elapsed time: 141.988579ms
``

If ran on Windows, uses nslookup instead of dig for the comparison.