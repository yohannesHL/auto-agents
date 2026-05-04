// Package main is a minimal Go stub service included to make the monorepo
// genuinely polyglot (TypeScript, Python, Go, Rust).
// Replace this with a real Go agent or gateway integration as needed.
package main

import (
	"fmt"
	"net/http"
	"os"
)

func main() {
	port := os.Getenv("GO_SERVICE_PORT")
	if port == "" {
		port = "8080"
	}

	http.HandleFunc("/healthz", func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		fmt.Fprintln(w, "ok")
	})

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintln(w, "go-service stub — replace with your Go agent")
	})

	addr := ":" + port
	fmt.Printf("go-service listening on %s\n", addr)
	if err := http.ListenAndServe(addr, nil); err != nil {
		fmt.Fprintf(os.Stderr, "go-service: %v\n", err)
		os.Exit(1)
	}
}
