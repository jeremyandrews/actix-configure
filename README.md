In to duplicate:

```
cargo run
```

Then in another window from this directory:

```
curl -v POST --data @test.base64 http://127.0.0.1:8080/upload
```

The output should look like:
```
* Could not resolve host: POST
curl: (6) Could not resolve host: POST
*   Trying 127.0.0.1:8080...
* TCP_NODELAY set
* Connected to 127.0.0.1 (127.0.0.1) port 8080 (#1)
> POST /upload HTTP/1.1
> Host: 127.0.0.1:8080
> User-Agent: curl/7.65.3
> Accept: */*
> Content-Length: 45632
> Content-Type: application/x-www-form-urlencoded
> Expect: 100-continue
> 
* Mark bundle as not supporting multiuse
< HTTP/1.1 100 Continue
* Mark bundle as not supporting multiuse
< HTTP/1.1 500 Internal Server Error
< content-length: 56
< content-type: text/plain; charset=utf-8
< date: Mon, 13 Jan 2020 10:45:18 GMT
* HTTP error before end of send, stop sending
< 
* Closing connection 1
App data is not configured, to configure use App::data()
```

