//
// WARNING! DO NOT RUN THIS TEST IF YOU ARE NOT USING THE APPWRITE SDK-GENERATOR TESTING STACK
// IT WILL FAIL!
//

use std::path::PathBuf;
use appwrite;
use reqwest::blocking::Response;
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct DefaultResponse {
  result: Option<String>,
  message: Option<String>
}

fn parse_response(data: Response) -> String {
  let response:DefaultResponse = match data.json() {
    Ok(value) => value,
    Err(err) => {
      return err.to_string()
    }
  };

  match response.result {
    Some(data) => return data,
    None => return match response.message {
      Some(value) => value,
      None => "Couldn't parse error.".to_string()
    }
  }
}

fn main() {
    let mut client = appwrite::client::Client::new();

    client.set_endpoint("https://appwrite.io/");
    client.set_project("");

    println!("Test Started");

    let foo = appwrite::services::Foo::new(&client);
    let bar = appwrite::services::Bar::new(&client);
    let general = appwrite::services::General::new(&client);

    // Foo

    println!("{}", parse_response(foo.get("string", 123, &["string in array"]).unwrap()));
    println!("{}", parse_response(foo.post("string", 123, &["string in array"]).unwrap()));
    println!("{}", parse_response(foo.put("string", 123, &["string in array"]).unwrap()));
    println!("{}", parse_response(foo.patch("string", 123, &["string in array"]).unwrap()));
    println!("{}", parse_response(foo.delete("string", 123, &["string in array"]).unwrap()));

    // Bar

    println!("{}", parse_response(bar.get("string", 123, &["string in array"]).unwrap()));
    println!("{}", parse_response(bar.post("string", 123, &["string in array"]).unwrap()));
    println!("{}", parse_response(bar.put("string", 123, &["string in array"]).unwrap()));
    println!("{}", parse_response(bar.patch("string", 123, &["string in array"]).unwrap()));
    println!("{}", parse_response(bar.delete("string", 123, &["string in array"]).unwrap()));

    // General

    println!("{}", parse_response(general.redirect().unwrap()));
    println!("{}", parse_response(general.upload("string", 123, &["string in array"], PathBuf::from("../../resources/file.png")).unwrap()));

    match general.error400() {
      Ok(data) => println!("{}", data.text().unwrap()),
      Err(err) => println!("{}", err.message)
    }

    match general.error500() {
      Ok(data) => println!("{}", data.text().unwrap()),
      Err(err) => println!("{}", err.message)
    }

    match general.error502() {
      Ok(data) => println!("{}", data.text().unwrap()),
      Err(err) => println!("{}", err.message)
    }
}