#![allow(warnings)]

fn main() {
    let favorite_color: Option<&str> = None;
    let is_employed = false;
    let age: Result<u8, _> = "24".parse();

    if let Some(color) = favorite_color {
        println!("Fav color = {}", color);
    } else if is_employed {
        println!("Great u'hv a job.");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("U r oldie!");
        } else {
            println!("young dude!");
        }
    } else {
        println!("Good for nothing");
    }
}
