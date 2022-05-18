let mut client = appwrite::client::Client::new();

client.set_endpoint("https://[HOSTNAME_OR_IP]/v1"); // Your API Endpoint
client.set_project("5df5acd0d48c2"); // Your project ID

let database = appwrite::services::Database::new(&client);

let response = match database.create_string_attribute("[COLLECTION_ID]", String::new(), 1, false) {
    Ok(response) => response,
    Err(error) => {
        println!("Error: {}", error);
        return;
    }
};

println!("{:?}", response);