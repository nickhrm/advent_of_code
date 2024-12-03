package main

import (
	"bufio"
	"fmt"
	"os"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {

	file, err := os.Open("./input")
	check(err)

	defer file.Close()

	scanner := bufio.NewScanner(file)

	fullText := ""

	for scanner.Scan() {
		fullText += scanner.Text()

	}

	var leftOperand, rightOperand int
	s, err := fmt.Sscanf(fullText, "mul(%d,%d)", &leftOperand, &rightOperand)

	check(err)
	fmt.Printf("s: %v\n", s)

}
