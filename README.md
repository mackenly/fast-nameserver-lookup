# fast-nameserver-lookup
Rust-based nameserver lookup that's faster and more efficient than dig

Nothing magic or custom here. Testing the speed of `dns-lookup` in Rust compared to `dig` on a Linux machine. Results are highly dependent on the machine and network conditions. 

Average time over 100 iterations of looking up NS records for mackenly.com:
```
Rust code:
- Average elapsed time: 1.511143ms

dig command:
- Average elapsed time: 78.585925ms
```

If run on Windows, uses nslookup instead of dig for the comparison.