package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
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

	goodLines := 0

	for scanner.Scan() {
		line := scanner.Text()
		numbersAsStrings := strings.Split(line, " ")
		numbers := []int{}
		for _, val := range numbersAsStrings {
			v, err := strconv.Atoi(val)
			check(err)
			numbers = append(numbers, v)
		}

		if checkWithRemovingOneByOne(numbers) {
			goodLines++
		}

	}

	fmt.Printf("End: %d", goodLines)

}

func checkWithRemovingOneByOne(numbers []int) bool {
	valid := false
	for i := range numbers {
		newNumbers := CopySlice(numbers) // Eine Kopie von numbers erstellen
		newNumbers = RemoveIndex(newNumbers, i)

		if checkSliceOfNumbers(newNumbers) {
			valid = true
			break
		}
	}
	return valid
}

// returns true if line is good
func checkSliceOfNumbers(nums []int) bool {
	lineIsGood := true

	lastVal := nums[0]

	var lineIsIncreasing bool = lastVal < nums[1]

	for index, val := range nums {
		if index == 0 {
			continue
		}
		fmt.Println(string(val))

		distance := val - lastVal

		if distance == 0 || math.Abs(float64(distance)) > 3 {
			lineIsGood = false
			break
		}

		valIsIncreasing := distance > 0

		if valIsIncreasing != lineIsIncreasing {
			lineIsGood = false
			break
		}

		lastVal = val
	}

	return lineIsGood

}

func RemoveIndex(s []int, index int) []int {
	return append(s[:index], s[index+1:]...)
}

func CopySlice(slice []int) []int {
	newSlice := make([]int, len(slice))
	copy(newSlice, slice)
	return newSlice
}
