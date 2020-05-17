// Q1:​ ​ Write a program which​ ​ contains:
// -> two user defined types ​ NewsArticle ​ and​ Tweet​ .
// NewsArticle consist of two fields : {author: String, content: String}
// Tweet consist of two fields : {username:String, content:String}
// -> a trait with the name ​ Summary​ , which contains a method signature.
// method signature like this:​ fn summarize (&self) -> String;
// -> Implement Summary trait on type NewsArticle and Tweet.
// -> In main function make instances of NewsArticle and Tweet and call summarize
// method on it.


fn main() {


pub struct NewsArticle {

    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,

}
pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("author :{}\ncontent  {}",self.author,self.content)
    }
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

let tweet = Tweet {
    username: String::from("momo"),
    content: String::from("what a great day"),

};

println!("1 new tweet:\n{}\n", tweet.summarize());

let article = NewsArticle {

    author: String::from("Diamond Rain"),
    content: String::from("Diamond rain falls on Saturn and Jupiter.\n"),
};

println!("New article available! {}\n", article.summarize());


}