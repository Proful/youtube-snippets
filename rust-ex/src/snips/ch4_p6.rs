#![allow(warnings)]

// chapter 4 -- part 6
// Slices
fn main() {
    let msg = "Hello Word";
    let word = &msg[..5];
    dbg!(word);

    let numbers = [5, 10, 15, 20, 25];
    let num = &numbers[0..3];
    dbg!(num);
}

fn main1() {
    let msg = String::from("This is interesting");

    // word store -> start index, length (end - start)
    // 3.. ..5 ..
    let word = &msg[0..5];

    // below stuff is mutable, use let mut msg
    // can't clear the string here, bcoz word is pointinng to it
    // msg.clear();

    dbg!("[main] word = {}", word);
}