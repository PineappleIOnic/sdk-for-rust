let mut client = appwrite::client::Client::new();

client.set_endpoint("https://[HOSTNAME_OR_IP]/v1"); // Your API Endpoint
client.set_project("5df5acd0d48c2"); // Your project ID
client.set_key("919c2d18fb5d4...a2ae413da83346ad2"); // Your secret API key

let users = appwrite::services::Users::new(&client);

let response = users.update_email("[USER_ID]", "email@example.com").unwrap();

println!("{}", response.text().unwrap());