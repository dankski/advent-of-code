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

func main() {
	// program_1 := "mul(2,3)"
	// program_2 := "mul ( 2 , 4 )"
	// program_3 := "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"

	// matches := parseProgramMemory(program_1)
	// fmt.Printf("PRG: %s, Match: %v\n", program_1, matches)
	// fmt.Printf("Match: %s, Sum: %d\n", matches, evaluateProgram(matches))

	// matches = parseProgramMemory(program_2)
	// fmt.Printf("PRG: %s, Match: %v\n", program_2, matches)
	// fmt.Printf("Match: %s, Sum: %d\n", matches, evaluateProgram(matches))

	// matches = parseProgramMemory(program_3)
	// fmt.Printf("PRG: %s, Match: %v\n", program_3, matches)
	// fmt.Printf("Match: %s, Sum: %d\n", matches, evaluateProgram(matches))

	day1()
}

func day1() {
	input, err := os.ReadFile("input.txt")
	if err != err {
		log.Fatal(err)
		os.Exit(1)
	}

	program := string(input)
	matches := parseProgramMemory(program)
	fmt.Printf("PRG: %s, Match: %v\n", program, matches)
	fmt.Printf("Match: %s, \n\nSum: %d\n", matches, evaluateProgram(matches))

}

func parseProgramMemory(p string) []string {
	return expressionParser.FindAllString(p, -1)
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
