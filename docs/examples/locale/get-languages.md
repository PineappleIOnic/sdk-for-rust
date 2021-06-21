let mut client = appwrite::client::Client::new();

client.set_endpoint("https://[HOSTNAME_OR_IP]/v1"); // Your API Endpoint
client.set_project("5df5acd0d48c2"); // Your project ID

let locale = appwrite::services::Locale::new(&client);

let response = locale.get_languages().unwrap();

println!("{}", response.text().unwrap());