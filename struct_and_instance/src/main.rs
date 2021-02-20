fn main() 
{
    let mut user1 = User 
    {
        email: String::from("someone@example.com"),
        user_name: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail.example.com");

    let user2 = User
    {
        email: String::from("another@example.com"),
        user_name: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count
    };

    let user2 = User
    {
        email: String::from("another@example.com"),
        user_name: String::from("anotherusername567"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct User
{
    user_name: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn build_user(email: String, user_name: String) -> User
{
    User
    {
        email, // Not have to specify field name;
        user_name,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);