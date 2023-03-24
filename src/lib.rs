use std::collections::HashMap;

pub fn two_sum(mut arr: Vec<i32>, target: i32) -> Vec<usize>{
    let mut index_map = HashMap::new();
    let mut sol_vec: Vec<usize> = Vec::new();  // specify the type of sol_vec and make it mutable
    for (index, value) in arr.iter().enumerate(){
        index_map.insert(value, index);
    }
    for (index, value) in arr.iter().enumerate(){
        let check_index = target-value; 
        if index_map.contains_key(&check_index){
            if index_map.get(&check_index) != Some(&index){  // use Some() to get a value from Option type
                sol_vec.push(index);
                sol_vec.push(*index_map.get(&check_index).unwrap());  // use * to dereference the Option type
                break;
            }
        }
    }
    sol_vec
}