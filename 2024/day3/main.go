package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
)

var expressionParser = regexp.MustCompile(`mul\(\d+,\d+\)`)
var valueParser = regexp.MustCompile(`mul\((\d+),(\d+)\)`)
var dontAndDoParser = regexp.MustCompile(`mul\((\d+),(\d+)\)|don't\(\)|do\(\)`)

func main() {

	day1(readInput("input.txt"))

	day2(readInput("input.txt"))

}

func day1(program string) {
	matches := parseProgramMemory(program, expressionParser)
	fmt.Printf("PRG: %s, Match: %v\n", program, matches)
	fmt.Printf("Match: %s, \n\nSum: %d\n", matches, evaluateProgram(matches))
}

func day2(program string) {
	matches := parseProgramMemory(program, dontAndDoParser)
	enabled := true
	var mulExpressions []string
	for _, m := range matches {
		if expressionParser.MatchString(m) {
			if enabled {
				mulExpressions = append(mulExpressions, m)
			}
		} else if "don't()" == m {
			enabled = false
		} else if "do()" == m {
			enabled = true
		}
	}

	fmt.Printf("PRG: %s, Match: %v\n", program, mulExpressions)
	fmt.Printf("Match: %V, \n\nSum: %d\n", mulExpressions, evaluateProgram(mulExpressions))
}

func parseProgramMemory(p string, parser *regexp.Regexp) []string {
	return parser.FindAllString(p, -1)
}

func evaluateProgram(expressions []string) int32 {
	var sum int32 = 0
	if len(expressions) > 0 {
		for _, exp := range expressions {
			values := valueParser.FindStringSubmatch(exp)
			a, _ := strconv.ParseInt(values[1], 10, 32)
			b, _ := strconv.ParseInt(values[2], 10, 32)
			sum = sum + (int32(a) * int32(b))
		}
	} else {
		fmt.Println("No expressions to calculate")
	}
	return sum
}

func readInput(filename string) string {
	input, err := os.ReadFile(filename)
	if err != err {
		log.Fatal(err)
		os.Exit(1)
	}

	return string(input)
}
