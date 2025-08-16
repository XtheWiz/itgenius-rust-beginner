trait Summary {
    fn summarize(&self) -> String;

    // Default implementation
    fn default_summary(&self) -> String {
        String::from("(Read more...)")
    }
}

// Implement trait สำหรับ struct
struct Article {
    headline: String,
    content: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking News!!! {}", item.summarize());
}

fn main() {
    let article = Article {
        headline: String::from("Rust is amazing!"),
        author: String::from("Jane Doe"),
        content: String::from("..."),
    };

    let tweet = Tweet {
        content: String::from("Elon Musk leave Tesla !!!"),
        username: String::from("Elon Monk")
    };

    // เรียกใช้ default implementation
    println!("Default Summary: {}", article.default_summary());

    notify(&article);
    notify(&tweet);
}
