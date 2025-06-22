use traits::{SocialPost, Summary};
fn main() {
     let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!{"summary, {}", post.summarize()}
    notify(&post);

    let aa = traits::Pair::new(1, 2);
    aa.cmp_display();
}
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news ! {}", item.summarize());
// }
// both means the same 
pub fn notify(item: &impl Summary) {
    println!("Breaking news ! {}", item.summarize());
}

// with impl summary we an use two different types as long both types implement summary
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {

// if we wanna have one type we need to use trait bound
// pub fn notify<T: Summary>(item1: &T, item2: &T) 

//itme needs to implement both
//pub fn notify(item: &(impl Summary + Display)) {
// or
//pub fn notify<T: Summary + Display>(item: &T) {

//fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
// differen syntax sugar for that
//fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {

fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}