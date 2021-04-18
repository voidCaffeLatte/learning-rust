use std::fmt::Debug;
use std::fmt::Display;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        let article = crate::NewsArticle {
            // ペンギンチームがスタンレーカップチャンピオンシップを勝ち取る！
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            // アメリカ、ペンシルベニア州、ピッツバーグ
            location: String::from("Pittsburgh, PA, USA"),
            // アイスバーグ
            author: String::from("Iceburgh"),
            // ピッツバーグ・ペンギンが再度NHL(National Hockey League)で最強のホッケーチームになった
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        };
        println!("New article available! {}", article.summarize());
    }
}

pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String
    // {
    //     format!("{}, by {}, ({})", self.headline, self.author, self.location)
    // }

    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// pub fn notify(item: &impl Summary)
// {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify<T: Summary>(item: &T)
{
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2(item1: &impl Summary, item2: &impl Summary)
{
    // do something
}

pub fn notify3<T: Summary>(item1: &T, item2: &T)
{
    // do something
}

pub fn notify4(item: &(impl Summary + Display))
{
    // do something
}

pub fn notify5<T: Summary + Display>(item: &T)
{
    // do something
}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32
{
    1
}

fn some_function2<T, U>(t: &T, u: &U) -> i32 
    where T: Display + Clone, U: Clone + Debug    
{
    1
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// error! must return same type value
// fn returns_summarizable2(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

struct Pair<T>
{
    x: T,
    y: T,
}

impl<T> Pair<T>
{
    fn new(x: T, y: T) -> Self
    {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T>
{
    fn cmp_display(&self)
    {
        if self.x >= self.y
        {
            println!("The largest number is x = {}", self.x);
        }
        else
        {
            pritnln!("The largest number is y = {}", self.y);
        }
    }
}