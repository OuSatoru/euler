package main

import (
	"fmt"
	"math"
)

func main() {
	factor := []int{}
	for k, v := range primes(int(math.Sqrt(600851475143))) {
		if v == 0 && 600851475143%k == 0 {
			factor = append(factor, k)
		}
	}
	fmt.Println(factor[len(factor)-1])
}

func primes(max int) []int {
	nums := make([]int, max+1)
	nums[0] = -1
	nums[1] = -1
	for i := 0; i < max+1; i++ {
		if nums[i] != -1 {
			for j := i + i; j < max+1; j += i {
				nums[j] = -1
			}
		}
	}
	return nums
}
