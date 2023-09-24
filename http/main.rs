extern crate reqwest;
extern crate serde;
extern crate serde_json;

use reqwest::Error;
use serde::Deserialize;

// Define a struct that represents the JSON data structure
#[derive(Debug, Deserialize)]
struct Post {
    userId: i32,
    id: i32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Define the URL you want to make a GET request to
    let url = "https://jsonplaceholder.typicode.com/posts/1";

    // Create an async function to send the GET request
    let response = reqwest::get(url).await?;

    // Check if the request was successful (status code 200 OK)
    if response.status().is_success() {
        // Read the response body as JSON and deserialize it into the Post struct
        let post: Post = response.json().await?;

        // Access and print fields from the JSON data
        println!("UserID: {}", post.userId);
        println!("Post ID: {}", post.id);
        println!("Title: {}", post.title);
        println!("Body: {}", post.body);
    } else {
        println!("Request failed with status code: {:?}", response.status());
    }

    Ok(())
}
