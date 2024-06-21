use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_table: HashMap<i32, usize> = HashMap::new();
        
        // First pass: build the hash table
        for (i, &num) in nums.iter().enumerate() {
            hash_table.insert(num, i);
        }
        
        // Second pass: find the solution
        for (i, &num) in nums.iter().enumerate() {
            let diff = target - num;
            if let Some(&j) = hash_table.get(&diff) {
                if i != j {
                    return vec![i as i32, j as i32];
                }
            }
        }
        
        // If no solution is found, which is guaranteed not to happen by problem constraints
        vec![]
    }
}