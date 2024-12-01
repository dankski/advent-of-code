package main

import (
	"day1/pkg/utils"
	"fmt"
)

func main() {
	content := utils.ReadFile("input-day1.txt")
	// content := utils.ReadFile("day1-test-input.txt")

	arr1, arr2 := utils.TwoLists(content)

	diff := utils.Diff(arr1, arr2)

	var sum_total_dist int64
	for _, num := range diff {
		sum_total_dist += num
	}

	fmt.Println("Sum of total distances:")
	fmt.Println(sum_total_dist)

	similarity_score := utils.Similarity(arr1, arr2)
	fmt.Println("Similarity score:")
	fmt.Println(similarity_score)
}
