package utils

import (
	"fmt"
	"runtime"
	"time"
)

// PrintMemUsage outputs the current, total and OS memory being used. As well as the number
// of garage collection cycles completed.
func PrintMemUsage() {
	var m runtime.MemStats
	runtime.ReadMemStats(&m)
	// For info on each, see: https://golang.org/pkg/runtime/#MemStats
	fmt.Printf("Alloc = %v MiB", formatByteToMB(m.Alloc))
	fmt.Printf("\nTotalAlloc = %v MiB", formatByteToMB(m.TotalAlloc))
	fmt.Printf("\nSys = %v MiB", formatByteToMB(m.Sys))
	fmt.Printf("\nNumGC = %v\n", m.NumGC)
}

func formatByteToMB(b uint64) uint64 {
	return b / 1024 / 1024
}

func Measure(fn func() interface{}) interface{} {
	start := time.Now()
	result := fn()
	elapsed := time.Since(start)

	fmt.Printf("\033[34mFinished in: %f seconds\n", elapsed.Seconds())
	PrintMemUsage()

	return result
}
