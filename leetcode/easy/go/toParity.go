package go_test
import "slices"

func transformArray(nums []int) []int {
	
	for i ,val := range nums { 
		if val % 2 != 0 { 
			nums[i] = 1
		}else { 
			nums[i] = 0
		}
	}
	slices.Sort(nums)
	return nums
}
