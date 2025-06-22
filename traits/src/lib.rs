
use std::fmt::Display;
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) ->String {
        format!("{}, ({})", self.headline, self.location)
    }
}


pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
pub trait Summary {
   fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct Pair<T> {
    x: T,
    y:T
}
impl <T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {x,y}
    }
}

impl <T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


