let mut client = appwrite::client::Client::new();

client.set_endpoint("https://[HOSTNAME_OR_IP]/v1"); // Your API Endpoint
client.set_project("5df5acd0d48c2"); // Your project ID
client.set_key("919c2d18fb5d4...a2ae413da83346ad2"); // Your secret API key

let storage = appwrite::services::Storage::new(&client);

let response = storage.get_file_preview("[BUCKET_ID]", "[FILE_ID]").unwrap();

println!("{}", response.text().unwrap());