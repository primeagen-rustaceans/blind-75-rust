use std::collections::HashSet;


fn main(){
    println!("hello there");
    // println!("{:?}",contains_duplicate(vec![7,1,2,3,40,10]));
    println!("{:?}",contains_duplicate_loop(vec![7,1,2,3,40,10]));
}

// using sets
fn contains_duplicate(numbers: Vec<i32>) -> bool {
    let mut results = HashSet::new();

    for item in numbers {
        if results.contains(&item) {
            return true;
        }
        results.insert(item);
    }
    false
}

// using rust collection exposed method
fn contains_duplicate_method(numbers: Vec<i32>) -> bool {
    let resultant_set:HashSet<&i32> = HashSet::from_iter(numbers.iter()); 

    !(&resultant_set.len() == &numbers.len())
}

// if vec size is too large and space constrain is there. Might give TLE in lc
fn contains_duplicate_loop(numbers: Vec<i32>) -> bool {
    for i in 0..(numbers.len()) {
        for j in 0..(numbers.len()) {
            if i != j {
                if numbers[i] == numbers[j] {
                    return true;
                }
            }
        }
    }
    false
}

// sorting the vec first and comparing adjacent values.
fn contains_duplicate_sort(numbers: Vec<i32>) -> bool {
    let mut sorted = numbers.clone();
    sorted.sort();

    for i in 0..(sorted.len() - 1) {
        if sorted[i] == sorted[i+1] {
            return true
        }
    }
    false
}