#![allow(dead_code)]

use std::collections::HashMap;

fn main() {
    let results:Vec<i32>=two_sum(vec!(1,2,3,4,5,6,7,8,9,10),10);
    // let results:Vec<i32>=two_sum_hm(vec!(1,2,3,4,5,6,7,8,9,10),10);
    println!("Hello, world! {:?}",results);
}
fn two_sum(num:Vec<i32>,target:i32)->Vec<i32>{
    let mut results:Vec<i32>=Vec::new();
    for i in 0..num.len() {
        for j in (i+1)..num.len() {
            if num[i]+num[j]==target{
                results.push(i as i32);
                results.push(j as i32);
            }
        }
    }
    results
}

// adding
fn two_sum_hm(num:Vec<i32>,target:i32)->Vec<i32>{
    let mut knowledge_base=HashMap::new();
    let mut results = vec![];
    for i in 0..num.len(){
        match knowledge_base.get(&(&target-&num[i])){
            Some(&index)=>{
                results.push(index as i32);
                results.push(i as i32);
                break;
            },
            None =>knowledge_base.insert(num[i],i as i32).unwrap()
        };
    }
    vec![]
}
