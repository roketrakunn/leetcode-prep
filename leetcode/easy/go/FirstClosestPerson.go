package go_test
func findClosest(x int, y int, z int) int {
    
	if  abs(x -z) > abs(y-z) { 
		return 2
	} else if abs(x-z) < abs(y-z) { 
		return 1
	}
	return 0
}

func abs(x int) int { 
	if x< 0 { 
		return  -x
	}
	return x 
}
