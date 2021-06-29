use crate::client::{Client, ParamType};
use std::collections::HashMap;
use crate::services::AppwriteException;

#[derive(Clone)]
pub struct Health {
  client: Client
}

impl Health {  
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone()
        }
    }

    /// Check the Appwrite HTTP server is up and responsive.
    pub fn get(&self) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/health";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();


        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    /// Check the Appwrite Anti Virus server is up and connection is successful.
    pub fn get_anti_virus(&self) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/health/anti-virus";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();


        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    /// Check the Appwrite in-memory cache server is up and connection is
    /// successful.
    pub fn get_cache(&self) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/health/cache";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();


        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    /// Check the Appwrite database server is up and connection is successful.
    pub fn get_db(&self) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/health/db";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();


        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    /// Get the number of certificates that are waiting to be issued against
    /// [Letsencrypt](https://letsencrypt.org/) in the Appwrite internal queue
    /// server.
    pub fn get_queue_certificates(&self) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/health/queue/certificates";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();


        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    pub fn get_queue_functions(&self) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/health/queue/functions";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();


        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    /// Get the number of logs that are waiting to be processed in the Appwrite
    /// internal queue server.
    pub fn get_queue_logs(&self) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/health/queue/logs";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();


        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    /// Get the number of tasks that are waiting to be processed in the Appwrite
    /// internal queue server.
    pub fn get_queue_tasks(&self) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/health/queue/tasks";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();


        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    /// Get the number of usage stats that are waiting to be processed in the
    /// Appwrite internal queue server.
    pub fn get_queue_usage(&self) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/health/queue/usage";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();


        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    /// Get the number of webhooks that are waiting to be processed in the Appwrite
    /// internal queue server.
    pub fn get_queue_webhooks(&self) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/health/queue/webhooks";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();


        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    /// Check the Appwrite local storage device is up and connection is successful.
    pub fn get_storage_local(&self) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/health/storage/local";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();


        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    /// Check the Appwrite server time is synced with Google remote NTP server. We
    /// use this technology to smoothly handle leap seconds with no disruptive
    /// events. The [Network Time
    /// Protocol](https://en.wikipedia.org/wiki/Network_Time_Protocol) (NTP) is
    /// used by hundreds of millions of computers and devices to synchronize their
    /// clocks over the Internet. If your computer sets its own clock, it likely
    /// uses NTP.
    pub fn get_time(&self) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/health/time";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();


        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }
}
