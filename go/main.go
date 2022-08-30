package main

import (
	"io"
	"net/http"
	"time"
)

func hello(w http.ResponseWriter, req *http.Request) {
	println("go:", req.Method, req.URL.String())
	io.WriteString(w, "hello world")
}

func main() {
	srv := &http.Server{
		Addr:              "127.0.0.1:42069",
		ReadHeaderTimeout: 1 * time.Second,
	}
	srv.Handler = http.HandlerFunc(hello)

	println("go: serving on", srv.Addr)
	if err := srv.ListenAndServe(); err != nil {
		println("go:", err)
	}
}
