This repo was created to duplicate a problem I was having:
https://github.com/actix/actix-web/issues/1272

To duplicate, clone this repo, and build:

```
cargo run
```

In another window, POST to this little server as follows (running this command from the top level of the repo):

```
curl -v --data @test.base64 http://127.0.0.1:8080/upload
```

The resulting error:

```
*   Trying 127.0.0.1:8080...
* TCP_NODELAY set
* Connected to 127.0.0.1 (127.0.0.1) port 8080 (#0)
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
< date: Mon, 13 Jan 2020 12:49:21 GMT
* HTTP error before end of send, stop sending
< 
* Closing connection 0
App data is not configured, to configure use App::data()
```

The error was fixed as follows:
https://github.com/jeremyandrews/actix-configure/pull/1

And the resulting code now works as expected:

```
 curl -v --data @test.base64 http://127.0.0.1:8080/upload
*   Trying 127.0.0.1:8080...
* TCP_NODELAY set
* Connected to 127.0.0.1 (127.0.0.1) port 8080 (#0)
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
* We are completely uploaded and fine
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-length: 17
< content-type: plain/text
< date: Mon, 13 Jan 2020 12:47:29 GMT
< 
* Connection #0 to host 127.0.0.1 left intact
upload successful
```
