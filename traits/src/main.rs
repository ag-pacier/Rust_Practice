pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
     }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
/*
Above is the simplest way but not the only way to implement a function using a trait.
The "trait bound" way to do it:
fn notify<T: Summary>(item: &T)
If you want to do more variables:
fn notify(item1: &impl Summary, item2: &impl Summary)
Lastly, if they need to be the same type:
fn notify<T: Summary>(item1: &T, item2: &T)

*/
fn main() {
    //Traits tell the compiler what kinds of things a generic can do
    //Other languages may call them interfaces

    //Trait methods are a way to group together like generics by methods

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}
