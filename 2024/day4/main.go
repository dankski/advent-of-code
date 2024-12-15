package main

import (
	"fmt"
	"strings"
)

type Direction struct {
	DX, DY int
}

var directions = []Direction{
	{DX: 0, DY: 1},   // right
	{DX: 0, DY: -1},  // left
	{DX: 1, DY: 0},   // down
	{DX: -1, DY: 1},  // up
	{DX: 1, DY: 1},   // diag down-right
	{DX: -1, DY: -1}, // diag up-left
	{DX: 1, DY: -1},  // diag down-left
	{DX: -1, DY: 1},  // diag up-right
}

type Grid [][]rune

type Match struct {
	X, Y, DX, DY int
}

func Day1(word string, input string) {
	grid := mapToGrid(input)
	rows := len(grid)
	cols := len(grid[0])

	positions := []Match{}

	for x := range rows {
		for y := range cols {
			for _, dir := range directions {
				if searchFrom(word, grid, x, y, dir.DX, dir.DY, rows, cols) {
					positions = append(positions, Match{X: x, Y: y, DX: dir.DX, DY: dir.DY})
				}
			}
		}
	}

	if 0 < len(positions) {
		fmt.Printf("Word '%s' total: %d, found at: %v\n", word, len(positions), positions)
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

func isValid(x, y, rows, cols int) bool {
	return (0 <= x) && (x < rows) && (0 <= y) && (y < cols)
}

func searchFrom(word string, grid Grid, x, y, dx, dy, rows, cols int) bool {

	for i, c := range word {
		nx, ny := x+i*dx, y+i*dy
		if !isValid(nx, ny, rows, cols) || grid[nx][ny] != c {
			return false
		}
	}
	return true
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

	Day1("XMAS", input)
}
