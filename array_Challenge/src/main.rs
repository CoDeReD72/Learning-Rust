/* Array Challenge */
// 1 - Sum of all elements
// 2 - Find the largest element
// 3 - Find the smallest element
// 4 - Reverse the array
// 5 - Pop an element from the array
// 6 - Push an element to the array


fn main() {

    let arr = [1,2,3,4,5,6,7,8,9,10];

    println!("Sum of all elements: {}", sum_of_all_elements(&arr)); // = 55
    println!("Largest element: {}", largest_element(&arr)); // = 10
    println!("Smallest element: {}", smallest_element(&arr)); // = 1
    println!("Reversed array: {:?}", reverse_array(&arr)); // = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
    println!("Popped array: {:?}", pop_element(&arr)); // = [2, 3, 4, 5, 6, 7, 8, 9, 10]
    println!("Pushed array: {:?}", push_element(&arr, 11)); // = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
    
}

fn sum_of_all_elements(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for i in arr {
        sum += i;
    }
    sum
}

fn largest_element(arr : &[i32]) -> i32{
    let mut largest = arr[0];
    for i in arr {
        if i > &largest {
            largest = *i;
        }
    }
    largest
}

fn smallest_element(arr : &[i32]) -> i32{
    let mut smallest = arr[0];
    for i in arr {
        if i < &smallest {
            smallest = *i;
        }
    }
    smallest
}

fn reverse_array(arr : &[i32]) -> Vec<i32> {
    let mut reversed_array = Vec::new();
    for i in arr.iter().rev() {
        // iter() - returns an iterator over the slice
        // rev() - reverses the iterator 
        // Meaning it will iterate from the end to the start
        reversed_array.push(*i);
    }
    reversed_array
}

fn pop_element(arr : &[i32]) -> Vec<i32> {
    let mut popped_array = Vec::new();
    for i in arr.iter().skip(1) { // skip(1) - skips the first element
        popped_array.push(*i);
    }
    popped_array
}

fn push_element(arr : &[i32] , element : i32) -> Vec<i32> {
    let mut pushed_array = Vec::new();
    for i in arr.iter() {
        pushed_array.push(*i);
    }
    pushed_array.push(element);
    pushed_array
}