#![allow(warnings)] // NOT RECOMMENDED

fn main() {
    let mut alphabets = vec!['a', 'b', 'c'];
    alphabets.push('d');

    dbg!(&alphabets);
    dbg!(&alphabets[2]);
    dbg!(&alphabets.len());
    dbg!(&alphabets.capacity());
    dbg!(&alphabets.is_empty());
    dbg!(&alphabets.first());
    dbg!(&alphabets.last());
    dbg!(&alphabets.len());
    dbg!(&alphabets.pop());
    dbg!(&alphabets.len());
    // dbg!(&alphabets[5]); // ERROR: Index out of bounds

    for alphabet in alphabets.iter() {
        dbg!(alphabet);
    }

    for (index, alphabet) in alphabets.iter().enumerate() {
        dbg!(index);
        dbg!(alphabet);
    }

    for alphabet in alphabets.iter_mut() {
        *alphabet = 'x';
    }
    dbg!(&alphabets);
}
