#![allow(warnings)]

trait Summary {
    // fn summarize(&self) -> String;
    // fn summarize(&self) -> String {
    //     "Just Do It! by Nike".to_string()
    // }
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Just Do It! by {}", self.summarize_author())
    }
}

struct Tweet {
    content: String,
    username: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} by {}", self.content, self.username)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

struct NewsArticle {
    headline: String,
    author: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
    // fn summarize(&self) -> String {
    //     format!("{} by {}", self.headline, self.author)
    // }
}

// Ch 10 - part3
// Traits
fn main() {
    let tweet = Tweet {
        content: "There is nothing important except how well you do your work!".to_string(),
        username: "profulsadangi".to_string(),
    };

    // dbg!(tweet.summarize());
    notify(&tweet);

    let news = NewsArticle {
        headline: "Removing my site from Google search".to_string(),
        author: "Proful".to_string(),
    };

    // dbg!(news.summarize());
    notify(&news);

    dbg!(returns_summarizable().summarize());
}

// fn notify(item: &impl Summary) {
//     // why not &item?
//     dbg!(item.summarize());
// }

fn notify<T: Summary>(item: &T) {
    // why not &item?
    dbg!(item.summarize());
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        content: "Something better than nothing!".to_string(),
        username: "Kenny".to_string(),
    }
}

//^ Traits
// Tells rustc about functionality a particular type has &
// can share with  other types
//
//
