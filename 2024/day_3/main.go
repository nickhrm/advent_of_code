package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
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

	findAnyRegex, _ := regexp.Compile(`(mul\(\d+,\d+\))|(do\(\))|(don't\(\)`)

	findMulRegex, _ := regexp.Compile(`mul\(\d+,\d+\)`)

	findDoRegex, _ := regexp.Compile(`do()`)
	findDontRegex, _ := regexp.Compile(`don't()`)

	muls := findMulRegex.FindAllString(fullText, -1)

	totalVal := 0

	for _, val := range muls {
		var leftOperand, rightOperand int
		_, err := fmt.Sscanf(val, "mul(%d,%d)", &leftOperand, &rightOperand)
		check(err)
		totalVal += (leftOperand * rightOperand)
	}

	fmt.Println(totalVal)

}
