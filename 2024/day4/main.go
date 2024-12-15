package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

type Direction struct {
	DX, DY int
}

var directions = []Direction{
	{DX: 0, DY: 1},   // horizontal right
	{DX: 0, DY: -1},  // horizonatal left
	{DX: 1, DY: 0},   // vertial down
	{DX: -1, DY: 0},  // vertical up
	{DX: 1, DY: 1},   // diag top-left to bottom-right
	{DX: -1, DY: -1}, // diag bottom-right to top-left
	{DX: 1, DY: -1},  // diag top-right to bottom-left
	{DX: -1, DY: 1},  // diag bottom-left to top-rightt
}

type Grid [][]rune

type Match struct {
	X, Y, DX, DY int
}

func Day1(word string, input string) {
	grid := mapToGrid(input)
	rows, cols := len(grid), len(grid[0])
	reverseWord := reverseString(word)

	positions := []Match{}

	for x := 0; x < rows; x++ {
		for y := 0; y < cols; y++ {
			for _, dir := range directions {
				dx, dy := dir.DX, dir.DY
				if searchFrom(word, grid, x, y, dx, dy, rows, cols) || searchFrom(reverseWord, grid, x, y, dir.DX, dir.DY, rows, cols) {
					positions = append(positions, Match{X: x, Y: y, DX: dir.DX, DY: dir.DY})
				}
			}
		}
	}

	if 0 < len(positions) {
		totalCountsWithoutDuplicates := len(positions) / 2
		// fmt.Printf("Word '%s' total: %d, found at: %v\n", word, totalCountsWithoutDuplicates, positions) // Dived by two because we len(positions) counts also duplicates
		fmt.Printf("Word '%s' total: %d\n", word, totalCountsWithoutDuplicates) // Dived by two because we len(positions) counts also duplicates
	} else {
		fmt.Printf("Word '%s' not found.\n", word)
	}
}

func mapToGrid(input string) Grid {
	lines := strings.Split(input, "\n")

	grid := Grid{}

	for _, line := range lines {
		grid = append(grid, []rune(line))
	}
	return grid
}

func notInBound(x, y, rows, cols int) bool {
	return (0 <= x) && (x < rows) && (0 <= y) && (y < cols)
}

func searchFrom(word string, grid Grid, x, y, dx, dy, rows, cols int) bool {

	for i := 0; i < len(word); i++ {
		nx, ny := x+i*dx, y+i*dy
		if !notInBound(nx, ny, rows, cols) || grid[nx][ny] != rune(word[i]) {
			return false
		}
	}
	return true
}

func reverseString(s string) string {
	runes := []rune(s)
	for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
		runes[i], runes[j] = runes[j], runes[i]
	}
	return string(runes)
}

func readInput(filename string) string {
	input, err := os.ReadFile(filename)
	if err != err {
		log.Fatal(err)
		os.Exit(1)
	}

	return string(input)
}

func main() {
	input := `MMMSXXMASM
	MSAMXMSMSA
	AMXSXMAAMM
	MSAMASMSMX
	XMASAMXAMM
	XXAMMXXAMA
	SMSMSASXSS
	SAXAMASAAA
	MAMMMXMMMM
	MXMXAXMASX`

	// input := strings.TrimSpace(readInput("input.txt"))

	Day1("XMAS", input)
}
