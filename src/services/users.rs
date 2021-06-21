use crate::client::{Client, ParamType};
use std::collections::HashMap;
use crate::services::AppwriteException;

#[derive(Clone)]
pub struct Users {
  client: Client
}

impl Users {  
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone()
        }
    }

    /// Get a list of all the project's users. You can use the query params to
    /// filter your results.
    pub fn list(&self, search: &str, limit: i64, offset: i64, order_type: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/users";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("search".to_string(), ParamType::String(search.to_string())),
            ("limit".to_string(),  ParamType::Number(limit)),
            ("offset".to_string(),  ParamType::Number(offset)),
            ("orderType".to_string(), ParamType::String(order_type.to_string())),
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    /// Create a new user.
    pub fn create(&self, email: &str, password: &str, name: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/users";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("email".to_string(), ParamType::String(email.to_string())),
            ("password".to_string(), ParamType::String(password.to_string())),
            ("name".to_string(), ParamType::String(name.to_string())),
        ].iter().cloned().collect();

        return self.client.clone().call("POST", &path, Some(headers), Some(params) );
    }

    /// Get a user by its unique ID.
    pub fn get(&self, user_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/users/userId".replace("userId", &user_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    /// Delete a user by its unique ID.
    pub fn delete(&self, user_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/users/userId".replace("userId", &user_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("DELETE", &path, Some(headers), Some(params) );
    }

    /// Get a user activity logs list by its unique ID.
    pub fn get_logs(&self, user_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/users/userId/logs".replace("userId", &user_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    /// Get the user preferences by its unique ID.
    pub fn get_prefs(&self, user_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/users/userId/prefs".replace("userId", &user_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    /// Update the user preferences by its unique ID. You can pass only the
    /// specific settings you wish to update.
    pub fn update_prefs(&self, user_id: &str, prefs: Option<HashMap<String, crate::client::ParamType>>) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/users/userId/prefs".replace("userId", &user_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("prefs".to_string(), ParamType::Object(prefs.unwrap())),
        ].iter().cloned().collect();

        return self.client.clone().call("PATCH", &path, Some(headers), Some(params) );
    }

    /// Get the user sessions list by its unique ID.
    pub fn get_sessions(&self, user_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/users/userId/sessions".replace("userId", &user_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    /// Delete all user's sessions by using the user's unique ID.
    pub fn delete_sessions(&self, user_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/users/userId/sessions".replace("userId", &user_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("DELETE", &path, Some(headers), Some(params) );
    }

    /// Delete a user sessions by its unique ID.
    pub fn delete_session(&self, user_id: &str, session_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/users/userId/sessions/sessionId".replace("userId", &user_id).replace("sessionId", &session_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("DELETE", &path, Some(headers), Some(params) );
    }

    /// Update the user status by its unique ID.
    pub fn update_status(&self, user_id: &str, status: i64) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/users/userId/status".replace("userId", &user_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("status".to_string(),  ParamType::Number(status)),
        ].iter().cloned().collect();

        return self.client.clone().call("PATCH", &path, Some(headers), Some(params) );
    }
}
