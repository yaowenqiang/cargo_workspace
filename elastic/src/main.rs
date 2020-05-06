extern crate elastic;
#[macro_use]

extern crate elastic_derive;

use elastic::prelude::*;
use elastic::http::header::Authorization;

    
fn main() {
    let builder = SyncClientBuilder::new()
        .base_url("https://es-lrulcngq.public.tencentelasticsearch.com:9200/")
        .params(|p| {
            p.url_param("pretty", true)
             .header(Authorization("Let me in ".to_owned()))
        });

    let client = builder.build();

    let response = client.send()?;

    println!("{:?}", response);

}
