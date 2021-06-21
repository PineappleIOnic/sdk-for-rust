let mut client = appwrite::client::Client::new();

client.set_endpoint("https://[HOSTNAME_OR_IP]/v1"); // Your API Endpoint
client.set_project("5df5acd0d48c2"); // Your project ID

let storage = appwrite::services::Storage::new(&client);

let response = storage.create_file(std::path::PathBuf::from("./path-to-files/image.jpg")).unwrap();

println!("{}", response.text().unwrap());