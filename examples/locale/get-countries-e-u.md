let mut client = appwrite::client::Client::new();

client.set_endpoint("https://[HOSTNAME_OR_IP]/v1"); // Your API Endpoint
client.set_project("5df5acd0d48c2"); // Your project ID
client.set_key("919c2d18fb5d4...a2ae413da83346ad2"); // Your secret API key

let locale = appwrite::services::Locale::new(&client);

let response = locale.get_countries_eu().unwrap();

println!("{}", response.text().unwrap());