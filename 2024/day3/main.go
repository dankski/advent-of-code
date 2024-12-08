package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"
)

var expressionParser = regexp.MustCompile(`mul\(\d+,\d+\)`)
var valueParser = regexp.MustCompile(`mul\((\d+),(\d+)\)`)
var dontAndDoParser = regexp.MustCompile(`don't\(\)(.*?)do\(\)`)

func main() {
	// var matches []string
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

	// day1(readInput("input.txt"))

	// program := "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
	// filter_matches := parseProgramMemory(program, dontAndDoParser)

	// filtered_program := strings.Replace(program, filter_matches[0], "", 1)
	// fitered_program_matches := parseProgramMemory(filtered_program, expressionParser)
	// fmt.Printf("Match: %s, Sum: %d\n", fitered_program_matches, evaluateProgram(fitered_program_matches))

	// day2(readInput("input.txt"))
	day2("ixmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))")
}

func day1(program string) {
	matches := parseProgramMemory(program, expressionParser)
	fmt.Printf("PRG: %s, Match: %v\n", program, matches)
	fmt.Printf("Match: %s, \n\nSum: %d\n", matches, evaluateProgram(matches))
}

func day2(program string) {
	dontMarker := "don't()"
	doMarker := "do()"

	var toRemove []string
	var stack []int

	for reader := 0; reader < len(program); {

		beginIndex := strings.Index(program[reader:], dontMarker)
		if beginIndex != -1 {
			stack = append(stack, beginIndex+len(dontMarker))
			reader = beginIndex + len(dontMarker)
			continue
		}

		endIndex := strings.Index(program[reader:], doMarker)
		if endIndex != -1 {
			startIndex := stack[len(stack)-1]
			stack = stack[:len(stack)-1] // pop
			toRemove = append(toRemove, program[startIndex:endIndex])
			reader = endIndex + len(doMarker)
			continue
		}

		reader += 1

	}

	fmt.Println("%v", toRemove)
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
