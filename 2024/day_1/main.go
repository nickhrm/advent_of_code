package main

import (
	"bufio"
	"fmt"
	"log"
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

	leftSide := []int{}
	rightSide := []int{}

	for scanner.Scan() {
		var left, right int
		_, err := fmt.Sscanf(scanner.Text(), "%d %d", &left, &right)
		check(err)

		leftSide = append(leftSide, left)
		rightSide = append(rightSide, right)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	// sort.Ints(leftSide)
	// sort.Ints(rightSide)

	// totalDistance := 0
	// fmt.Println(totalDistance)

	// for index := range leftSide {
	// 	distance := rightSide[index] - leftSide[index]
	// 	absDistance := int(math.Abs((float64(distance))))

	// 	totalDistance += absDistance
	// }

	similarityScore := 0

	for _, leftSideVal := range leftSide {
		occurences := 0
		for _, rightSideVal := range rightSide {
			if rightSideVal == leftSideVal {
				occurences++
			}
		}
		similarityScore += occurences * leftSideVal
	}

	fmt.Println("Total")
	fmt.Println(similarityScore)

}
