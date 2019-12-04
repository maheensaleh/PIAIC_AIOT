#[derive(Debug)]
struct Post{
    author :String,
    about:String,
    text:String,

}
//creating trait
trait Newnotification{
    fn alert(&self)->String;
}

//creating trait
trait Summarize{
    fn summary(&self)->String;
}

// implementing trait
impl Newnotification for Post{
    fn alert(&self)->String{
        format!("hey you have got a new notification")
    }
}

//implementing trait
impl Summarize for Post{
    fn summary(&self)->String{
        format!("Post made by {} about {} context {}",self.author,self.about,self.text)
    }
}

//trait bound function
fn notify<T: Newnotification +Summarize>(mess :T){
    println!("new message! \n{:?}\n{:?}",mess.alert(),mess.summary());
}

fn main(){
    let mypost = Post{
        author: String::from("sara"),
        about:String::from("weather"),
        text:String::from("what a great day!"),
    };
    // println!("{}",mypost.alert());
    // println!("{}",mypost.summary());
    notify(mypost);
}
