use aggregator::{Summary, Tweet, NewsArticle};

struct Point<T, U> {
    // values could be of the same or different types
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<T2, U2>(self, other: Point<T2, U2>) -> Point<T, U2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f32, f32> {
    fn _distance_from_origin(&self) -> f32 {
        (&self.x.powi(2) + &self.y.powi(2)).sqrt()
    }
}

fn main() {
    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 2.0, y: 8.0 };
    let mixed = Point { x: 3, y: 9.0 };
    let mixed2 = Point {x: "hello", y: 'c'};

    println!("mixed.x = {}", mixed.x());

    let mixed3 = mixed.mixup(mixed2);
    println!("mixed3.x = {}, mixed3.y = {}", mixed3.x, mixed3.y);

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['w', 'a', 'd', 'r', 'x'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    /////////////////////////
    // Implementing Traits //
    /////////////////////////
    
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL",
        ),
    };

    println!("New article available! {}", article.summarize());
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    // for (index, item) in list.iter().enumerate() {
    //     if item > largest {
    //         largest = &list[index];
    //     }
    // }

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
