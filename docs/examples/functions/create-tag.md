let mut client = appwrite::client::Client::new();

client.set_endpoint("https://[HOSTNAME_OR_IP]/v1"); // Your API Endpoint
client.set_project("5df5acd0d48c2"); // Your project ID

let functions = appwrite::services::Functions::new(&client);

let response = functions.create_tag("[FUNCTION_ID]", "[COMMAND]", std::path::PathBuf::from("./path-to-files/image.jpg")).unwrap();

println!("{}", response.text().unwrap());