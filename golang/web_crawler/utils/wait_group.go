package utils

import (
	"sync"
	"sync/atomic"
)

type WaitGroupCounter struct {
	wg    sync.WaitGroup
	count int64
}

func (w *WaitGroupCounter) Add(delta int) {
	w.wg.Add(delta)
	atomic.AddInt64(&w.count, int64(delta))
}

func (w *WaitGroupCounter) Done() {
	w.wg.Done()
	atomic.AddInt64(&w.count, -1)
}

func (w *WaitGroupCounter) Wait() {
	w.wg.Wait()
}

func (w *WaitGroupCounter) Count() int {
	return int(atomic.LoadInt64(&w.count))
}
