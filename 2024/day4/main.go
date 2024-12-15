package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

var directions = [][2]int{
	{0, 1},   // Horizontal right
	{0, -1},  // Horizontal left
	{1, 0},   // Vertical down
	{-1, 0},  // Vertical up
	{1, 1},   // Diagonal top-left to bottom-right
	{-1, -1}, // Diagonal bottom-right to top-left
	{1, -1},  // Diagonal top-right to bottom-left
	{-1, 1},  // Diagonal bottom-left to top-right
}

// Function to check if the position is within bounds
func isInBounds(x, y, rows, cols int) bool {
	return x >= 0 && x < rows && y >= 0 && y < cols
}

// Function to search in a specific direction
func search(grid [][]rune, word string, x, y, dx, dy int) bool {
	rows, cols := len(grid), len(grid[0])
	for i := 0; i < len(word); i++ {
		nx, ny := x+i*dx, y+i*dy
		if !isInBounds(nx, ny, rows, cols) || grid[nx][ny] != rune(word[i]) {
			return false
		}
	}
	return true
}

// Main function to find all occurrences of a word and its reverse
func findWord(grid [][]rune, word string) [][2]int {
	reverseWord := reverseString(word)
	rows, cols := len(grid), len(grid[0])
	var occurrences [][2]int

	for x := 0; x < rows; x++ {
		for y := 0; y < cols; y++ {
			// Check all directions from this starting point
			for _, dir := range directions {
				dx, dy := dir[0], dir[1]
				if search(grid, word, x, y, dx, dy) || search(grid, reverseWord, x, y, dx, dy) {
					occurrences = append(occurrences, [2]int{x, y})
				}
			}
		}
	}

	return occurrences
}

// Helper function to reverse a string
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
	// Input grid as a string
	// 	input := `MMMSXXMASM
	// MSAMXMSMSA
	// AMXSXMAAMM
	// MSAMASMSMX
	// XMASAMXAMM
	// XXAMMXXAMA
	// SMSMSASXSS
	// SAXAMASAAA
	// MAMMMXMMMM
	// MXMXAXMASX`

	input := strings.TrimSpace(readInput("input.txt"))

	// Convert input to 2D rune slice
	lines := strings.Split(input, "\n")
	grid := make([][]rune, len(lines))
	for i, line := range lines {
		grid[i] = []rune(line)
	}

	// Word to find
	word := "XMAS"

	// Find occurrences
	occurrences := findWord(grid, word)

	// Print occurrences
	fmt.Println("Occurrences of XMAS (or SAMX):")
	fmt.Printf("Total: %d\n", len(occurrences)/2)
	// for _, occ := range occurrences {
	// 	fmt.Printf("Start at row %d, col %d\n", occ[0]+1, occ[1]+1)
	// }
}
