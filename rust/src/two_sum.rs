pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>{
    for (i, el) in nums.iter().enumerate() {
        let diff = target - *el;
        let v = &nums[i..nums.len()];
        let diff_index = v.iter().rposition(|&r| r == diff );
        match diff_index {
            Some(d_index) => {
                let dd = i + d_index;
                if nums.contains(&diff) && i != dd {
                    return [i as i32, dd as i32].to_vec();
                }
            }
            None => ()
        }
        
    }
    return [-1, -1].to_vec()
}