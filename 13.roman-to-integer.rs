/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        //Solution 1
        // let mut hash_table: HashMap<char, i32> = HashMap::new();
        // hash_table.insert('I', 1);
        // hash_table.insert('V', 5);
        // hash_table.insert('X', 10);
        // hash_table.insert('L', 50);
        // hash_table.insert('C', 100);
        // hash_table.insert('D', 500);
        // hash_table.insert('M', 1000);

        // let chars: Vec<char> = s.chars().collect();
        // //println!("{:?}", chars);
        // let mut answer = 0;

        // for i in 0..chars.len() {
        //     if let Some(&cur) = hash_table.get(&chars[i]) {
        //         if i > 0 {
        //             if let Some(&prev) = hash_table.get(&chars[i-1]) {
        //                 if prev < cur {
        //                     answer = answer - (2*prev) + cur
        //                 } else {
        //                     answer += cur;
        //                 }
        //             }
        //         } 
        //         else {
        //             answer += cur;
        //         }
        //         //print!("{} {}\n", &chars[i], &answer);
        //     }
        // }
        // answer

        //Solution 2
        let s_t = s
            .replace("IV", "IIII")
            .replace("IX", "VIIII")
            .replace("XL", "XXXX")
            .replace("XC", "LXXXX")
            .replace("CD", "CCCC")
            .replace("CM", "DCCCC");

        s_t.chars().map(|c| {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0
            }
        }).sum()
    }
}

// @lc code=end

