
struct  Solution;

impl Solution {

    pub fn count_of_pairs(n: i32, x: i32, y: i32) -> Vec<i32> {
        let mut result = vec![0i32; n as usize];
            
            for i in 1..=n  {
                for j in i+1..=n{ 
                    let dist1 = (i-j).abs();
                    let dist2 = (i-x).abs() + 1 + (y-j).abs();
                    let dist3 = (i-y).abs() + 1 + (x-j).abs();


                    let min_dist = dist1.min(dist2).min(dist3) as usize;

                    result[min_dist -1] += 2;
                }
            }
       result 
    }
}
