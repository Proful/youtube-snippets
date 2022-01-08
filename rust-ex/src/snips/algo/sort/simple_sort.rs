#![allow(warnings)] // NOT RECOMMENDED

use std::fmt::Debug;

pub fn bubble_sort<T>(arr: &mut [T])
 where T : Ord + Debug {
    println!("Original: {:?}", arr);
    for i in 0..arr.len() {
        dbg!(arr.len() - i - 1);
        println!("arr.len() - i - 1: {}", arr.len() - i - 1);
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
            println!("{:?}", arr);
        }
        println!("--------------------");
    }
}

pub fn insertion_sort<T>(arr: &mut [T])
 where T : Ord + Debug {
    println!("Original: {:?}", arr);
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
        println!("{:?}", arr);
        println!("--------------------");
    }
}

pub fn selection_sort<T>(arr: &mut [T])
 where T : Ord + Debug {
    println!("Original: {:?}", arr);
    for i in 0..arr.len() - 1 {
        let mut min_idx  = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        arr.swap(i, min_idx);
        println!("{:?}", arr);
        println!("--------------------");
    }
}

fn main() {
    let mut nums = [50, 4, 30, 5, 20];
    // bubble_sort(&mut nums);
    // insertion_sort(&mut nums);
    selection_sort(&mut nums);
    println!("Sorted: {:?}", nums);
}