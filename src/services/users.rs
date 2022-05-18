use crate::client::{Client, ParamType};
use std::collections::HashMap;
use crate::services::AppwriteException;
use crate::models;
use serde_json::json;
use std::io::Read;

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
    pub fn list(&self, search: Option<&str>, limit: Option<i64>, offset: Option<i64>, cursor: Option<&str>, cursor_direction: Option<&str>, order_type: Option<&str>) -> Result<models::UserList, AppwriteException> {
        let path = "/users";
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let search:&str = match search {
            Some(data) => data,
            None => ""
        };

        let cursor:&str = match cursor {
            Some(data) => data,
            None => ""
        };

        let cursor_direction:&str = match cursor_direction {
            Some(data) => data,
            None => ""
        };

        let order_type:&str = match order_type {
            Some(data) => data,
            None => ""
        };

        let  params: HashMap<String, ParamType> = [
            ("search".to_string(), ParamType::String(search.to_string())),
            ("limit".to_string(),  ParamType::OptionalNumber(limit)),
            ("offset".to_string(),  ParamType::OptionalNumber(offset)),
            ("cursor".to_string(), ParamType::String(cursor.to_string())),
            ("cursorDirection".to_string(), ParamType::String(cursor_direction.to_string())),
            ("orderType".to_string(), ParamType::String(order_type.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::UserList = match response {
            Ok(r) => {
                match r.json() {
                    Ok(json) => json,
                    Err(e) => {
                        return Err(AppwriteException::new(format!("Error parsing response json: {}", e), 0, "".to_string()));
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)
    }

    /// Create a new user.
    pub fn create(&self, user_id: &str, email: &str, password: &str, name: Option<&str>) -> Result<models::User, AppwriteException> {
        let path = "/users";
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let name:&str = match name {
            Some(data) => data,
            None => ""
        };

        let  params: HashMap<String, ParamType> = [
            ("userId".to_string(), ParamType::String(user_id.to_string())),
            ("email".to_string(), ParamType::String(email.to_string())),
            ("password".to_string(), ParamType::String(password.to_string())),
            ("name".to_string(), ParamType::String(name.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("POST", &path, Some(headers), Some(params) );

        let processedResponse:models::User = match response {
            Ok(r) => {
                match r.json() {
                    Ok(json) => json,
                    Err(e) => {
                        return Err(AppwriteException::new(format!("Error parsing response json: {}", e), 0, "".to_string()));
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)
    }

    pub fn get_usage(&self, range: Option<&str>, provider: Option<&str>) -> Result<models::UsageUsers, AppwriteException> {
        let path = "/users/usage";
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let range:&str = match range {
            Some(data) => data,
            None => ""
        };

        let provider:&str = match provider {
            Some(data) => data,
            None => ""
        };

        let  params: HashMap<String, ParamType> = [
            ("range".to_string(), ParamType::String(range.to_string())),
            ("provider".to_string(), ParamType::String(provider.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::UsageUsers = match response {
            Ok(r) => {
                match r.json() {
                    Ok(json) => json,
                    Err(e) => {
                        return Err(AppwriteException::new(format!("Error parsing response json: {}", e), 0, "".to_string()));
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)
    }

    /// Get a user by its unique ID.
    pub fn get(&self, user_id: &str) -> Result<models::User, AppwriteException> {
        let path = "/users/userId".replace("userId", &user_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::User = match response {
            Ok(r) => {
                match r.json() {
                    Ok(json) => json,
                    Err(e) => {
                        return Err(AppwriteException::new(format!("Error parsing response json: {}", e), 0, "".to_string()));
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)
    }

    /// Delete a user by its unique ID.
    pub fn delete(&self, user_id: &str) -> Result<serde_json::value::Value, AppwriteException> {
        let path = "/users/userId".replace("userId", &user_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("DELETE", &path, Some(headers), Some(params) );

        match response {
            Ok(r) => {
                let status_code = r.status();
                if status_code == reqwest::StatusCode::NO_CONTENT {
                    Ok(json!(true))
                } else {
                    Ok(serde_json::from_str(&r.text().unwrap()).unwrap())
                }
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    /// Update the user email by its unique ID.
    pub fn update_email(&self, user_id: &str, email: &str) -> Result<models::User, AppwriteException> {
        let path = "/users/userId/email".replace("userId", &user_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
            ("email".to_string(), ParamType::String(email.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("PATCH", &path, Some(headers), Some(params) );

        let processedResponse:models::User = match response {
            Ok(r) => {
                match r.json() {
                    Ok(json) => json,
                    Err(e) => {
                        return Err(AppwriteException::new(format!("Error parsing response json: {}", e), 0, "".to_string()));
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)
    }

    /// Get the user activity logs list by its unique ID.
    pub fn get_logs(&self, user_id: &str, limit: Option<i64>, offset: Option<i64>) -> Result<models::LogList, AppwriteException> {
        let path = "/users/userId/logs".replace("userId", &user_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
            ("limit".to_string(),  ParamType::OptionalNumber(limit)),
            ("offset".to_string(),  ParamType::OptionalNumber(offset)),
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::LogList = match response {
            Ok(r) => {
                match r.json() {
                    Ok(json) => json,
                    Err(e) => {
                        return Err(AppwriteException::new(format!("Error parsing response json: {}", e), 0, "".to_string()));
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)
    }

    /// Update the user name by its unique ID.
    pub fn update_name(&self, user_id: &str, name: &str) -> Result<models::User, AppwriteException> {
        let path = "/users/userId/name".replace("userId", &user_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
            ("name".to_string(), ParamType::String(name.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("PATCH", &path, Some(headers), Some(params) );

        let processedResponse:models::User = match response {
            Ok(r) => {
                match r.json() {
                    Ok(json) => json,
                    Err(e) => {
                        return Err(AppwriteException::new(format!("Error parsing response json: {}", e), 0, "".to_string()));
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)
    }

    /// Update the user password by its unique ID.
    pub fn update_password(&self, user_id: &str, password: &str) -> Result<models::User, AppwriteException> {
        let path = "/users/userId/password".replace("userId", &user_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
            ("password".to_string(), ParamType::String(password.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("PATCH", &path, Some(headers), Some(params) );

        let processedResponse:models::User = match response {
            Ok(r) => {
                match r.json() {
                    Ok(json) => json,
                    Err(e) => {
                        return Err(AppwriteException::new(format!("Error parsing response json: {}", e), 0, "".to_string()));
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)
    }

    /// Get the user preferences by its unique ID.
    pub fn get_prefs(&self, user_id: &str) -> Result<models::Preferences, AppwriteException> {
        let path = "/users/userId/prefs".replace("userId", &user_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::Preferences = match response {
            Ok(r) => {
                match r.json() {
                    Ok(json) => json,
                    Err(e) => {
                        return Err(AppwriteException::new(format!("Error parsing response json: {}", e), 0, "".to_string()));
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)
    }

    /// Update the user preferences by its unique ID. The object you pass is stored
    /// as is, and replaces any previous value. The maximum allowed prefs size is
    /// 64kB and throws error if exceeded.
    pub fn update_prefs(&self, user_id: &str, prefs: Option<HashMap<String, crate::client::ParamType>>) -> Result<models::Preferences, AppwriteException> {
        let path = "/users/userId/prefs".replace("userId", &user_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
            ("prefs".to_string(), ParamType::Object(prefs.unwrap())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("PATCH", &path, Some(headers), Some(params) );

        let processedResponse:models::Preferences = match response {
            Ok(r) => {
                match r.json() {
                    Ok(json) => json,
                    Err(e) => {
                        return Err(AppwriteException::new(format!("Error parsing response json: {}", e), 0, "".to_string()));
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)
    }

    /// Get the user sessions list by its unique ID.
    pub fn get_sessions(&self, user_id: &str) -> Result<models::SessionList, AppwriteException> {
        let path = "/users/userId/sessions".replace("userId", &user_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::SessionList = match response {
            Ok(r) => {
                match r.json() {
                    Ok(json) => json,
                    Err(e) => {
                        return Err(AppwriteException::new(format!("Error parsing response json: {}", e), 0, "".to_string()));
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)
    }

    /// Delete all user's sessions by using the user's unique ID.
    pub fn delete_sessions(&self, user_id: &str) -> Result<serde_json::value::Value, AppwriteException> {
        let path = "/users/userId/sessions".replace("userId", &user_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("DELETE", &path, Some(headers), Some(params) );

        match response {
            Ok(r) => {
                let status_code = r.status();
                if status_code == reqwest::StatusCode::NO_CONTENT {
                    Ok(json!(true))
                } else {
                    Ok(serde_json::from_str(&r.text().unwrap()).unwrap())
                }
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    /// Delete a user sessions by its unique ID.
    pub fn delete_session(&self, user_id: &str, session_id: &str) -> Result<serde_json::value::Value, AppwriteException> {
        let path = "/users/userId/sessions/sessionId".replace("userId", &user_id).replace("sessionId", &session_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("DELETE", &path, Some(headers), Some(params) );

        match response {
            Ok(r) => {
                let status_code = r.status();
                if status_code == reqwest::StatusCode::NO_CONTENT {
                    Ok(json!(true))
                } else {
                    Ok(serde_json::from_str(&r.text().unwrap()).unwrap())
                }
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    /// Update the user status by its unique ID.
    pub fn update_status(&self, user_id: &str, status: bool) -> Result<models::User, AppwriteException> {
        let path = "/users/userId/status".replace("userId", &user_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
            ("status".to_string(), ParamType::Bool(status)),
        ].iter().cloned().collect();

        let response = self.client.clone().call("PATCH", &path, Some(headers), Some(params) );

        let processedResponse:models::User = match response {
            Ok(r) => {
                match r.json() {
                    Ok(json) => json,
                    Err(e) => {
                        return Err(AppwriteException::new(format!("Error parsing response json: {}", e), 0, "".to_string()));
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)
    }

    /// Update the user email verification status by its unique ID.
    pub fn update_verification(&self, user_id: &str, email_verification: bool) -> Result<models::User, AppwriteException> {
        let path = "/users/userId/verification".replace("userId", &user_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
            ("emailVerification".to_string(), ParamType::Bool(email_verification)),
        ].iter().cloned().collect();

        let response = self.client.clone().call("PATCH", &path, Some(headers), Some(params) );

        let processedResponse:models::User = match response {
            Ok(r) => {
                match r.json() {
                    Ok(json) => json,
                    Err(e) => {
                        return Err(AppwriteException::new(format!("Error parsing response json: {}", e), 0, "".to_string()));
                    }
                }
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)
    }
}
