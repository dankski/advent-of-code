package utils

import (
	"log"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

func ReadFile(fname string) string {
	content, err := os.ReadFile(fname)
	if err != nil {
		log.Fatal(err)
		os.Exit(1)
	}

	return string(content)
}

func TwoLists(content string) ([]int64, []int64) {
	arr1 := []int64{}
	arr2 := []int64{}

	lines := strings.Split(content, "\n")
	for _, line := range lines {
		if line == "" {
			break
		}
		tokens := strings.Split(line, "   ")
		num_right, err := strconv.ParseInt(tokens[0], 10, 64)
		if err != nil {
			log.Fatal(err)
			os.Exit(1)
		}

		arr1 = append(arr1, num_right)

		num_left, err := strconv.ParseInt(tokens[1], 10, 64)
		if err != nil {
			log.Fatal(err)
			os.Exit(1)
		}
		arr2 = append(arr2, num_left)
	}

	return arr1, arr2
}

func Diff(list1 []int64, list2 []int64) []int64 {
	sort.Slice(list1, func(i, j int) bool {
		return list1[i] < list1[j]
	})

	sort.Slice(list2, func(i, j int) bool {
		return list2[i] < list2[j]
	})

	diff := []int64{}

	for i, num := range list1 {
		diff = append(diff, int64(math.Abs(float64(num-list2[i]))))
	}

	return diff
}

func Similarity(list1 []int64, list2 []int64) int64 {
	total_similarities := []int64{}

	for _, num := range list1 {
		total_similarities = append(total_similarities, num*occurrences(num, list2))
	}

	var score int64
	for _, num := range total_similarities {
		score += num
	}

	return score
}

func occurrences(target int64, list []int64) int64 {
	count := 0

	for _, num := range list {
		if num == target {
			count++
		}
	}
	return int64(count)
}
