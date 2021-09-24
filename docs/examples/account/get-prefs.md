let mut client = appwrite::client::Client::new();

client.set_endpoint("https://[HOSTNAME_OR_IP]/v1"); // Your API Endpoint
client.set_project("5df5acd0d48c2"); // Your project ID
client.set_jwt("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ..."); // Your secret JSON Web Token

let account = appwrite::services::Account::new(&client);

let response = account.get_prefs().unwrap();

println!("{}", response.text().unwrap());