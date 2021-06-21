let mut client = appwrite::client::Client::new();

client.set_endpoint("https://[HOSTNAME_OR_IP]/v1"); // Your API Endpoint
client.set_project("5df5acd0d48c2"); // Your project ID

let database = appwrite::services::Database::new(&client);

let response = database.create_document("[COLLECTION_ID]", {}).unwrap();

println!("{}", response.text().unwrap());