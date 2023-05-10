use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use reqwest::{Client, Error};

#[derive(Serialize, Deserialize, Debug)]
struct JsonResponse {
    json: HashMap<String, String>
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut data = HashMap::new();
    data.insert("symbol", "ASHYCRE");
    data.insert("faction", "Cosmic");

    let client = Client::new();
    let res = client.post("https://api.spacetraders.io/v2/register")
        .json(&data)
        .send()
        .await?;

    println!("{:#?}", res);

    Ok(())
}
