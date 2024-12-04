package main

import (
	"fmt"
	"regexp"
)

func main() {
	program_1 := "mul(2,3)"
	program_2 := "mul ( 2 , 4 )"
	program_3 := "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"

	matches := parseProgramMemory(program_1)
	fmt.Printf("PRG: %s, Match: %v\n", program_1, matches)

	matches = parseProgramMemory(program_2)
	fmt.Printf("PRG: %s, Match: %v\n", program_2, matches)

	matches = parseProgramMemory(program_3)
	fmt.Printf("PRG: %s, Match: %v\n", program_3, matches)
}

func parseProgramMemory(p string) []string {
	re := regexp.MustCompile(`mul\(\d+,\d+\)`)
	return re.FindAllString(p, -1)
}
