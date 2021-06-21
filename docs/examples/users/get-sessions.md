let mut client = appwrite::client::Client::new();

client.set_endpoint("https://[HOSTNAME_OR_IP]/v1"); // Your API Endpoint
client.set_project("5df5acd0d48c2"); // Your project ID

let users = appwrite::services::Users::new(&client);

let response = users.get_sessions("[USER_ID]").unwrap();

println!("{}", response.text().unwrap());