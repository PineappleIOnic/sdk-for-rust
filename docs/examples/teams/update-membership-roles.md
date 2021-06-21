let mut client = appwrite::client::Client::new();

client.set_endpoint("https://[HOSTNAME_OR_IP]/v1"); // Your API Endpoint
client.set_project("5df5acd0d48c2"); // Your project ID

let teams = appwrite::services::Teams::new(&client);

let response = teams.update_membership_roles("[TEAM_ID]", "[MEMBERSHIP_ID]", &[]).unwrap();

println!("{}", response.text().unwrap());