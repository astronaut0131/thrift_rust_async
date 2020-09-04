package main

import (
	"flag"
	"fmt"
	"github.com/apache/thrift/lib/go/thrift"
	"os"
	"sync"
	"time"
)

const (
	THREAD_NUM = 1024
	LOOP_NUM = 10000
)

func Usage() {
	fmt.Fprint(os.Stderr, "Usage of ", os.Args[0], ":\n")
	flag.PrintDefaults()
	fmt.Fprint(os.Stderr, "\n")
}

func main() {
	flag.Usage = Usage
	protocol := flag.String("P", "binary", "Specify the protocol (binary, compact, json, simplejson)")
	//framed := flag.Bool("framed", false, "Use framed transport")
	//buffered := flag.Bool("buffered", false, "Use buffered transport")
	addr := flag.String("addr", "localhost:9090", "Address to listen to")
	secure := flag.Bool("secure", false, "Use tls secure transport")

	flag.Parse()

	var protocolFactory thrift.TProtocolFactory
	switch *protocol {
	case "compact":
		protocolFactory = thrift.NewTCompactProtocolFactory()
	case "simplejson":
		protocolFactory = thrift.NewTSimpleJSONProtocolFactory()
	case "json":
		protocolFactory = thrift.NewTJSONProtocolFactory()
	case "binary", "":
		protocolFactory = thrift.NewTBinaryProtocolFactoryDefault()
	default:
		fmt.Fprint(os.Stderr, "Invalid protocol specified", protocol, "\n")
		Usage()
		os.Exit(1)
	}

	var transportFactory thrift.TTransportFactory
	//if *buffered {
		transportFactory = thrift.NewTBufferedTransportFactory(8192)
	//} else {
	//	transportFactory = thrift.NewTTransportFactory()
	//}

	//if *framed {
	//	transportFactory = thrift.NewTFramedTransportFactory(transportFactory)
	//}

	wg := sync.WaitGroup{}
	wg.Add(THREAD_NUM)
	before := time.Now()
	for i := 0; i < THREAD_NUM; i++ {
		go func() {
			defer wg.Done()
			if err := runClient(transportFactory, protocolFactory, *addr, *secure); err != nil {
				fmt.Println("error running client:", err)
			}
		}()
	}
	wg.Wait()
	total := time.Now().UnixNano() - before.UnixNano()
	fmt.Printf("thread: %d, loop: %d, thread * loop: %d\n", THREAD_NUM, LOOP_NUM, THREAD_NUM * LOOP_NUM)
	fmt.Printf("total time: %d us\n", total / 1000)
	fmt.Printf("qps: %f\n", float64(THREAD_NUM * LOOP_NUM * 1000 * 1000 * 1000) / float64(total))
}
