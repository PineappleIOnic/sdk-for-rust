let mut client = Appwrite::client::Client::new();

client.set_endpoint("https://[HOSTNAME_OR_IP]/v1"); // Your API Endpoint
client.set_project("5df5acd0d48c2"); // Your project ID
client.set_jwt("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ..."); // Your secret JSON Web Token

let account = Appwrite::services::Account::new(&client);

let response = account.get_session("[SESSION_ID]").unwrap();

println!("{}", response.text().unwrap());