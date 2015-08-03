# ip-power-bar-rs


Simple command line tool for the [IP Power
Bar](http://www.netio-products.com/en/product/netio-230c/)

## Examples

```
$ cargo.exe run -- -p 80 -h 192.168.10.100 get
     Running `target\debug\ip-power-bar.exe -p 80 -h 192.168.10.100 get`
Connecting to 192.168.10.100:80
Response: Ok, <html>0 0 0 0 </html>


$ cargo.exe run -- -p 80 -h 192.168.10.100 set 1 on
     Running `target\debug\ip-power-bar.exe -p 80 -h 192.168.10.100 set 1 on`
Connecting to 192.168.10.100:80
http://192.168.10.100:80/cgi/control.cgi?l=p:admin:admin&p=1uuu
Response: Ok, <html>250 OK</html>

$ cargo.exe run -- -p 80 -h 192.168.10.100 get
     Running `target\debug\ip-power-bar.exe -p 80 -h 192.168.10.100 get`
Connecting to 192.168.10.100:80
Response: Ok, <html>1 0 0 0 </html>
```

