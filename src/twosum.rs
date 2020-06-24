use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_map: HashMap<i32, usize> = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        if let Some(second_index) = num_map.get(&(target.clone() - num.clone())) {
            if *second_index != index {
                return vec![index.clone() as i32, second_index.clone() as i32];
            }
        }
        num_map.insert(num.clone(), index.clone());
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_2() {
        assert_eq!(two_sum(vec![1, 2], 3), vec![1, 0]);
    }

    #[test]
    fn case_3() {
        assert_eq!(two_sum(vec![1, 2, 3], 3), vec![1, 0])
    }

    #[test]
    fn case_3_out_of_order() {
        assert_eq!(two_sum(vec![1, 3, 2], 3), vec![2, 0])
    }
    #[test]
    fn last_2_of_3() {
        assert_eq!(two_sum(vec![1, 2, 3], 5), vec![2, 1])
    }

    #[test]
    fn last_2_of_3_test() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![2, 1])
    }
    #[test]
    fn multiples() {
        assert_eq!(two_sum(vec![3, 3, 2], 6), vec![1, 0])
    }
}
