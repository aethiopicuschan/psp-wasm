package main

import "unsafe"

//go:wasmimport debug println
func println(ptr unsafe.Pointer, len int32)

func main() {
	message := "Hello, WebAssembly, from Golang!"
	println(unsafe.Pointer(unsafe.StringData(message)), int32(len(message)))
}
