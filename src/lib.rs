use std::{collections::HashMap, hash::Hash};

// Two sum
pub fn two_sum(mut arr: Vec<i32>, target: i32) -> Vec<usize> {
    let mut index_map = HashMap::new();
    let mut sol_vec: Vec<usize> = Vec::new(); // specify the type of sol_vec and make it mutable
    for (index, value) in arr.iter().enumerate() {
        index_map.insert(value, index);
    }
    for (index, value) in arr.iter().enumerate() {
        let check_index = target - value;
        if index_map.contains_key(&check_index) {
            if index_map.get(&check_index) != Some(&index) {
                // use Some() to get a value from Option type
                sol_vec.push(index);
                sol_vec.push(*index_map.get(&check_index).unwrap()); // use * to dereference the Option type
                break;
            }
        }
    }
    sol_vec
}

// Best time to buy and sell stock
pub fn btt_buy_and_sell(arr: Vec<i32>) -> i32 {
    let mut buy_price = arr[0];
    let mut max_profit = 0;
    for i in 1..=arr.len() - 1 {
        if arr[i] - buy_price > max_profit {
            max_profit = arr[i] - buy_price;
        }
        if arr[i] < buy_price {
            buy_price = arr[i];
        }
    }
    max_profit
}

pub fn contains_duplicate(arr:Vec<i32>) -> bool{
    let mut ret_value = false;
    let mut check = HashMap::new();
    for item in arr {
        if check.contains_key(&item) {
            ret_value = true; 
            break;
        }else{
            check.insert(item, 1);
        }
    }
    ret_value
}