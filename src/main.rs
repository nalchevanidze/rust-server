use warp::Filter;
//use std::fs;

// fn get_file() -> String {
//     let x = fs::read_to_string("hello.html").unwrap();
//     x.to_string()
// }

#[tokio::main]
async fn main() {
    let port = 3000;
    let host = [127, 0, 0, 1];
    
    let hello = warp::path!("test" / String)
        .map(|name| format!("test, {}!", name));

    let any = warp::any()
        .and(warp::fs::file("./assets/index.html"));

    let routes = warp::get().and(hello.or(any));
    
    println!("started server with Port: {}", port);

    warp::serve(routes)
        .run((host, port))
        .await;
}