package main

import (
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	// input, err := os.ReadFile("example.txt")
	input, err := os.ReadFile("input.txt")
	if err != err {
		log.Fatal(err)
		os.Exit(1)
	}

	firstSolution(string(input))

	secondSolution(string(input))

}

func firstSolution(content string) {
	reports := generateReports(content)
	levels := calculateLevel(reports)

	safe := int32(0)
	for _, isSafe := range levels {
		if isSafe {
			safe++
		}
	}

	fmt.Printf("[SOL-1] Safe %d\n", safe)
}

func secondSolution(content string) {
	reports := generateReports(content)
	levels := calculateLevel(reports)

	safe := int32(0)
	for reportIndx, isSafe := range levels {
		if isSafe {
			safe++
		} else {
			report := reports[reportIndx]
			yes := isFalsePositive(report)
			if yes {
				safe++
			}
		}
	}

	fmt.Printf("[SOL-2] Safe %d\n", safe)
}

func isFalsePositive(report []int32) bool {
	size := len(report)
	for i := 0; i < size; i++ {
		newReport := make([]int32, size)
		copy(newReport[:], report)
		newReport = append(newReport[:i], newReport[i+1:]...)
		result := calculateLevel([][]int32{newReport})
		if result[0] {
			return true
		}
	}
	return false
}

func generateReports(content string) [][]int32 {
	reports := [][]int32{}

	lines := strings.Split(content, "\n")
	for _, line := range lines {
		if line == "" {
			break
		}

		tokens := strings.Split(line, " ")
		report := []int32{}
		for _, token := range tokens {
			num, err := strconv.ParseInt(token, 10, 32)
			if err != nil {
				log.Fatal(err)
				os.Exit(1)
			}
			report = append(report, int32(num))
		}
		reports = append(reports, report)
	}

	return reports
}

func calculateLevel(reports [][]int32) []bool {
	safeReports := []bool{}

	for _, report := range reports {

		levels := []int32{}
		for i := 1; i < len(report); i++ {
			diff := report[i] - report[i-1]
			levels = append(levels, int32(math.Abs(float64(diff))))
		}

		if !changesDirection(report) && toleranceOk(levels) {
			safeReports = append(safeReports, true)
		} else {
			safeReports = append(safeReports, false)
		}

	}

	return safeReports
}

func toleranceOk(levels []int32) bool {
	ok := false
	for _, level := range levels {
		if level >= 1 && level <= 3 {
			ok = true
		} else {
			ok = false
			break
		}
	}
	return ok
}

// true = direction change detected, false = no direction change detected
func changesDirection(levels []int32) bool {
	if len(levels) < 2 {
		return false
	}

	// 0 = no direction, 1 = ascending, -1 = descending
	var dir int32
	for i := 1; i < len(levels); i++ {
		if levels[i] > levels[i-1] {
			if dir == -1 {
				return true
			}
			dir = 1 // set direction to ascending
		} else if levels[i] < levels[i-1] {
			if dir == 1 {
				return true
			}
			dir = -1 // set direction to descending
		}
		// if levels[i] == levels[i-1], direction remains unchanged
	}
	return false
}
