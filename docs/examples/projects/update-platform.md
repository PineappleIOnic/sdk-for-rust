let mut client = appwrite::client::Client::new();

client.set_endpoint("https://[HOSTNAME_OR_IP]/v1"); // Your API Endpoint
client.set_project("5df5acd0d48c2"); // Your project ID

let projects = appwrite::services::Projects::new(&client);

let response = projects.update_platform("[PROJECT_ID]", "[PLATFORM_ID]", "[NAME]").unwrap();

println!("{}", response.text().unwrap());