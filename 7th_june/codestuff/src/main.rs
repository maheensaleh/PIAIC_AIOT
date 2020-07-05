fn make_cors()->Cors{

    let allowed_origins = Allowed_origins::some_exact(&[
        "http://best.ball.surge.sh",

    ]),
    .to_cors()
    .expect("Error while building cors")

}

#[get("/")]


type MessageMap = Mutux<HashMap>

