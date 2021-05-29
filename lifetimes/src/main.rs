fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// ライフタイム省略規則

// 最初の規則は、参照である各引数は、独自のライフタイム引数を得るというものです。
// 換言すれば、 1引数の関数は、1つのライフタイム引数を得るということです: fn foo<'a>(x: &'a i32); 
// 2つ引数のある関数は、2つの個別のライフタイム引数を得ます: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); 以下同様。

// 2番目の規則は、1つだけ入力ライフタイム引数があるなら、
// そのライフタイムが全ての出力ライフタイム引数に代入されるというものです: fn foo<'a>(x: &'a i32) -> &'a i32。

// 3番目の規則は、複数の入力ライフタイム引数があるけれども、
// メソッドなのでそのうちの一つが&selfや&mut selfだったら、 selfのライフタイムが全出力ライフタイム引数に代入されるというものです。 
// この3番目の規則により、必要なシンボルの数が減るので、メソッドが遥かに読み書きしやすくなります。

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        // お知らせします
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: std::fmt::Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
