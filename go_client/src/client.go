package main

import (
	"context"
	"crypto/tls"
	"fmt"
	"tutorial"

	"github.com/apache/thrift/lib/go/thrift"
)

var defaultCtx = context.Background()

func handleClient(client *tutorial.CalculatorClient) (err error) {
	for i := 0; i < LOOP_NUM; i++ {
		err = client.Ping(defaultCtx)
		//fmt.Println("ping()")
	}

	return err
}

func runClient(transportFactory thrift.TTransportFactory, protocolFactory thrift.TProtocolFactory, addr string, secure bool) error {
	var transport thrift.TTransport
	var err error
	if secure {
		cfg := new(tls.Config)
		cfg.InsecureSkipVerify = true
		transport, err = thrift.NewTSSLSocket(addr, cfg)
	} else {
		transport, err = thrift.NewTSocket(addr)
	}
	if err != nil {
		fmt.Println("Error opening socket:", err)
		return err
	}
	transport, err = transportFactory.GetTransport(transport)
	if err != nil {
		return err
	}
	defer transport.Close()
	if err := transport.Open(); err != nil {
		return err
	}
	iprot := protocolFactory.GetProtocol(transport)
	oprot := protocolFactory.GetProtocol(transport)
	return handleClient(tutorial.NewCalculatorClient(thrift.NewTStandardClient(iprot, oprot)))
}
