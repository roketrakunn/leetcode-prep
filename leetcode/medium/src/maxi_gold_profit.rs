struct Solution; 

use std::collections::HashMap;

impl Solution {
    pub fn maximize_the_profit(n: i32, offers: Vec<Vec<i32>>) -> i32 {
        let mut  group_by_ends = HashMap::new();

        for offer in &offers {
            group_by_ends.entry(offer[1])
                .or_insert(vec![])
                .push(offer);
        }

        let mut dp = vec![0i32; (n+1) as usize]; 
        for i in 1..=n { 
            dp[i as usize] = dp[(i -1) as usize];

            if let Some(ending_offers) = group_by_ends.get(&(i-1)) { 
                for offer in ending_offers { 
                    dp[i as usize] = dp[i as usize].max(dp[offer[0] as usize ] 
                        + offer[2]);
                }
            }

        }
       dp[n as usize] 
    }
}
