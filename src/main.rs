

fn main(){
    println!("hello there");
    println!("{:?}",profit(vec![7,1,2,3,40,10]));
}

fn profit(price: Vec<i32>) -> i32 {
    let mut profit: i32 = 0;
    let mut minimum: i32 = price[0];

    for current_price in price.iter().skip(1) {
        if current_price < &minimum {
            minimum = *current_price 
        }

        if current_price - minimum > profit {
            profit = current_price - minimum;
        }
    }
    profit
}

// if want to know the day of buy and sell : 2 pointers should be used.
fn profit_two_pointers(prices: Vec<i32>) -> i32 {
    let mut max_profit: i32 = 0;
    let mut left_ptr = 0;
    let mut right_ptr = 1;

    while right_ptr < prices.len() - 1 {
        if prices[left_ptr] < prices[right_ptr] {
            let profit = prices[right_ptr] - prices[left_ptr];

            if profit > max_profit {
                max_profit = profit;
            }
        } else {
            left_ptr = right_ptr;
        }
        right_ptr += 1
    }
    max_profit
}