#![deny(warnings)]
#[tokio::main]

async fn main() -> Result<(), reqwest::Error> {
    /*
    let body = reqwest::get("https://www.baidu.com")
        .await?
        .text()
        .await?;

    println!("body = {:?}", body);

    let resp = reqwest::get("http://static.baidu.com/gb_js/app/app_NewAsk.json")?;
    println!("{:?}", resp.remote_addr());
    Ok(())
    */
    let url = "http://static.baidu.com/gb_js/app/app_NewAsk.json";
    //let echo_json: serde_json::Value = reqwest::get(url)
    let body = reqwest::get(url)
        .await?
        .text()
        .await?;

    println!("{:#?}", body);
    let echo_json: serde_json::Value = match serde_json::from_str(&body) {
        Ok(json) => json,
        Err(err) => panic!("error parse json, {:?}", err)
    };
    println!("{:#?}", echo_json);
    Ok(())

}
