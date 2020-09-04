package main

import (
	"context"
	"fmt"
)

type CalculatorHandler struct {
}

func NewCalculatorHandler() *CalculatorHandler {
	return &CalculatorHandler{}
}

func (p *CalculatorHandler) Ping(ctx context.Context) (err error) {
	fmt.Print("ping()\n")
	return nil
}

func (p *CalculatorHandler) Zip(ctx context.Context) (err error) {
	fmt.Print("zip()\n")
	return nil
}
