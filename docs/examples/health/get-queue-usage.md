let mut client = appwrite::client::Client::new();

client.set_endpoint("https://[HOSTNAME_OR_IP]/v1"); // Your API Endpoint
client.set_project("5df5acd0d48c2"); // Your project ID

let health = appwrite::services::Health::new(&client);

let response = health.get_queue_usage().unwrap();

println!("{}", response.text().unwrap());