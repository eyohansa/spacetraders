use std::fs;
// use reqwest::{Client, Error};
use serde_json::{Result, Value};

// use spacetraders::Data;

fn main() -> Result<()> {
    // let mut data = HashMap::new();
    // data.insert("symbol", "USER_01");
    // data.insert("faction", "COSMIC");

    // let client = Client::new();
    // let res = client.post("https://api.spacetraders.io/v2/register")
    //     .header("content", "application/json")
    //     .json(&data)
    //     .send()
    //     .await?
    //     .json::<JsonResponse>()
    //     .await?;

    // println!("{:#?}", res);

    let contents = fs::read_to_string("response.example.json")
        .expect("File not found");

    let json: Value = serde_json::from_str(&contents)?;

    println!("{:?}", json);

    Ok(())

}
