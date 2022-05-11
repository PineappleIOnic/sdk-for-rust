use crate::client::{Client, ParamType};
use std::collections::HashMap;
use crate::services::AppwriteException;
use crate::models;
use serde_json::json;
use std::io::Read;

#[derive(Clone)]
pub struct Account {
  client: Client
}

impl Account {  
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone()
        }
    }

    /// Get currently logged in user data as JSON object.
    pub fn get(&self) -> Result<models::User, AppwriteException> {
        let path = "/account";
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

    /// Delete a currently logged in user account. Behind the scene, the user
    /// record is not deleted but permanently blocked from any access. This is done
    /// to avoid deleted accounts being overtaken by new users with the same email
    /// address. Any user-related resources like documents or storage files should
    /// be deleted separately.
    pub fn delete(&self) -> Result<serde_json::value::Value, AppwriteException> {
        let path = "/account";
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

    /// Update currently logged in user account email address. After changing user
    /// address, the user confirmation status will get reset. A new confirmation
    /// email is not sent automatically however you can use the send confirmation
    /// email endpoint again to send the confirmation email. For security measures,
    /// user password is required to complete this request.
    /// This endpoint can also be used to convert an anonymous account to a normal
    /// one, by passing an email address and a new password.
    /// 
    pub fn update_email(&self, email: &str, password: &str) -> Result<models::User, AppwriteException> {
        let path = "/account/email";
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
            ("email".to_string(), ParamType::String(email.to_string())),
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

    /// Get currently logged in user list of latest security activity logs. Each
    /// log returns user IP address, location and date and time of log.
    pub fn get_logs(&self, limit: Option<i64>, offset: Option<i64>) -> Result<models::LogList, AppwriteException> {
        let path = "/account/logs";
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

    /// Update currently logged in user account name.
    pub fn update_name(&self, name: &str) -> Result<models::User, AppwriteException> {
        let path = "/account/name";
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

    /// Update currently logged in user password. For validation, user is required
    /// to pass in the new password, and the old password. For users created with
    /// OAuth and Team Invites, oldPassword is optional.
    pub fn update_password(&self, password: &str, old_password: Option<&str>) -> Result<models::User, AppwriteException> {
        let path = "/account/password";
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let old_password:&str = match old_password {
            Some(data) => data,
            None => ""
        };

        let  params: HashMap<String, ParamType> = [
            ("password".to_string(), ParamType::String(password.to_string())),
            ("oldPassword".to_string(), ParamType::String(old_password.to_string())),
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

    /// Get currently logged in user preferences as a key-value object.
    pub fn get_prefs(&self) -> Result<models::Preferences, AppwriteException> {
        let path = "/account/prefs";
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

    /// Update currently logged in user account preferences. The object you pass is
    /// stored as is, and replaces any previous value. The maximum allowed prefs
    /// size is 64kB and throws error if exceeded.
    pub fn update_prefs(&self, prefs: Option<HashMap<String, crate::client::ParamType>>) -> Result<models::User, AppwriteException> {
        let path = "/account/prefs";
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
            ("prefs".to_string(), ParamType::Object(prefs.unwrap())),
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

    /// Sends the user an email with a temporary secret key for password reset.
    /// When the user clicks the confirmation link he is redirected back to your
    /// app password reset URL with the secret key and email address values
    /// attached to the URL query string. Use the query string params to submit a
    /// request to the [PUT
    /// /account/recovery](/docs/client/account#accountUpdateRecovery) endpoint to
    /// complete the process. The verification link sent to the user's email
    /// address is valid for 1 hour.
    pub fn create_recovery(&self, email: &str, url: &str) -> Result<models::Token, AppwriteException> {
        let path = "/account/recovery";
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
            ("email".to_string(), ParamType::String(email.to_string())),
            ("url".to_string(), ParamType::String(url.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("POST", &path, Some(headers), Some(params) );

        let processedResponse:models::Token = match response {
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

    /// Use this endpoint to complete the user account password reset. Both the
    /// **userId** and **secret** arguments will be passed as query parameters to
    /// the redirect URL you have provided when sending your request to the [POST
    /// /account/recovery](/docs/client/account#accountCreateRecovery) endpoint.
    /// 
    /// Please note that in order to avoid a [Redirect
    /// Attack](https://github.com/OWASP/CheatSheetSeries/blob/master/cheatsheets/Unvalidated_Redirects_and_Forwards_Cheat_Sheet.md)
    /// the only valid redirect URLs are the ones from domains you have set when
    /// adding your platforms in the console interface.
    pub fn update_recovery(&self, user_id: &str, secret: &str, password: &str, password_again: &str) -> Result<models::Token, AppwriteException> {
        let path = "/account/recovery";
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
            ("userId".to_string(), ParamType::String(user_id.to_string())),
            ("secret".to_string(), ParamType::String(secret.to_string())),
            ("password".to_string(), ParamType::String(password.to_string())),
            ("passwordAgain".to_string(), ParamType::String(password_again.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("PUT", &path, Some(headers), Some(params) );

        let processedResponse:models::Token = match response {
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

    /// Get currently logged in user list of active sessions across different
    /// devices.
    pub fn get_sessions(&self) -> Result<models::SessionList, AppwriteException> {
        let path = "/account/sessions";
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

    /// Delete all sessions from the user account and remove any sessions cookies
    /// from the end client.
    pub fn delete_sessions(&self) -> Result<serde_json::value::Value, AppwriteException> {
        let path = "/account/sessions";
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

    /// Use this endpoint to get a logged in user's session using a Session ID.
    /// Inputting 'current' will return the current session being used.
    pub fn get_session(&self, session_id: &str) -> Result<models::Session, AppwriteException> {
        let path = "/account/sessions/sessionId".replace("sessionId", &session_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::Session = match response {
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

    pub fn update_session(&self, session_id: &str) -> Result<models::Session, AppwriteException> {
        let path = "/account/sessions/sessionId".replace("sessionId", &session_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("PATCH", &path, Some(headers), Some(params) );

        let processedResponse:models::Session = match response {
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

    /// Use this endpoint to log out the currently logged in user from all their
    /// account sessions across all of their different devices. When using the
    /// Session ID argument, only the unique session ID provided is deleted.
    /// 
    pub fn delete_session(&self, session_id: &str) -> Result<serde_json::value::Value, AppwriteException> {
        let path = "/account/sessions/sessionId".replace("sessionId", &session_id);
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

    /// Use this endpoint to send a verification message to your user email address
    /// to confirm they are the valid owners of that address. Both the **userId**
    /// and **secret** arguments will be passed as query parameters to the URL you
    /// have provided to be attached to the verification email. The provided URL
    /// should redirect the user back to your app and allow you to complete the
    /// verification process by verifying both the **userId** and **secret**
    /// parameters. Learn more about how to [complete the verification
    /// process](/docs/client/account#accountUpdateVerification). The verification
    /// link sent to the user's email address is valid for 7 days.
    /// 
    /// Please note that in order to avoid a [Redirect
    /// Attack](https://github.com/OWASP/CheatSheetSeries/blob/master/cheatsheets/Unvalidated_Redirects_and_Forwards_Cheat_Sheet.md),
    /// the only valid redirect URLs are the ones from domains you have set when
    /// adding your platforms in the console interface.
    /// 
    pub fn create_verification(&self, url: &str) -> Result<models::Token, AppwriteException> {
        let path = "/account/verification";
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
            ("url".to_string(), ParamType::String(url.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("POST", &path, Some(headers), Some(params) );

        let processedResponse:models::Token = match response {
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

    /// Use this endpoint to complete the user email verification process. Use both
    /// the **userId** and **secret** parameters that were attached to your app URL
    /// to verify the user email ownership. If confirmed this route will return a
    /// 200 status code.
    pub fn update_verification(&self, user_id: &str, secret: &str) -> Result<models::Token, AppwriteException> {
        let path = "/account/verification";
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
            ("userId".to_string(), ParamType::String(user_id.to_string())),
            ("secret".to_string(), ParamType::String(secret.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("PUT", &path, Some(headers), Some(params) );

        let processedResponse:models::Token = match response {
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
