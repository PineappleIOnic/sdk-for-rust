use crate::client::{Client, ParamType};
use std::collections::HashMap;
use crate::services::AppwriteException;

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
    pub fn get(&self) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/account";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    /// Use this endpoint to allow a new user to register a new account in your
    /// project. After the user registration completes successfully, you can use
    /// the [/account/verfication](/docs/client/account#accountCreateVerification)
    /// route to start verifying the user email address. To allow the new user to
    /// login to their new account, you need to create a new [account
    /// session](/docs/client/account#accountCreateSession).
    pub fn create(&self, email: &str, password: &str, name: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/account";

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

    /// Delete a currently logged in user account. Behind the scene, the user
    /// record is not deleted but permanently blocked from any access. This is done
    /// to avoid deleted accounts being overtaken by new users with the same email
    /// address. Any user-related resources like documents or storage files should
    /// be deleted separately.
    pub fn delete(&self) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/account";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("DELETE", &path, Some(headers), Some(params) );
    }

    /// Update currently logged in user account email address. After changing user
    /// address, user confirmation status is being reset and a new confirmation
    /// mail is sent. For security measures, user password is required to complete
    /// this request.
    /// This endpoint can also be used to convert an anonymous account to a normal
    /// one, by passing an email address and a new password.
    pub fn update_email(&self, email: &str, password: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/account/email";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("email".to_string(), ParamType::String(email.to_string())),
            ("password".to_string(), ParamType::String(password.to_string())),
        ].iter().cloned().collect();

        return self.client.clone().call("PATCH", &path, Some(headers), Some(params) );
    }

    /// Use this endpoint to create a JSON Web Token. You can use the resulting JWT
    /// to authenticate on behalf of the current user when working with the
    /// Appwrite server-side API and SDKs. The JWT secret is valid for 15 minutes
    /// from its creation and will be invalid if the user will logout.
    pub fn create_jwt(&self) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/account/jwt";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("POST", &path, Some(headers), Some(params) );
    }

    /// Get currently logged in user list of latest security activity logs. Each
    /// log returns user IP address, location and date and time of log.
    pub fn get_logs(&self) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/account/logs";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    /// Update currently logged in user account name.
    pub fn update_name(&self, name: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/account/name";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("name".to_string(), ParamType::String(name.to_string())),
        ].iter().cloned().collect();

        return self.client.clone().call("PATCH", &path, Some(headers), Some(params) );
    }

    /// Update currently logged in user password. For validation, user is required
    /// to pass in the new password, and the old password. For users created with
    /// OAuth and Team Invites, oldPassword is optional.
    pub fn update_password(&self, password: &str, old_password: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/account/password";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("password".to_string(), ParamType::String(password.to_string())),
            ("oldPassword".to_string(), ParamType::String(old_password.to_string())),
        ].iter().cloned().collect();

        return self.client.clone().call("PATCH", &path, Some(headers), Some(params) );
    }

    /// Get currently logged in user preferences as a key-value object.
    pub fn get_prefs(&self) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/account/prefs";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    /// Update currently logged in user account preferences. You can pass only the
    /// specific settings you wish to update.
    pub fn update_prefs(&self, prefs: Option<HashMap<String, crate::client::ParamType>>) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/account/prefs";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("prefs".to_string(), ParamType::Object(prefs.unwrap())),
        ].iter().cloned().collect();

        return self.client.clone().call("PATCH", &path, Some(headers), Some(params) );
    }

    /// Sends the user an email with a temporary secret key for password reset.
    /// When the user clicks the confirmation link he is redirected back to your
    /// app password reset URL with the secret key and email address values
    /// attached to the URL query string. Use the query string params to submit a
    /// request to the [PUT
    /// /account/recovery](/docs/client/account#accountUpdateRecovery) endpoint to
    /// complete the process. The verification link sent to the user's email
    /// address is valid for 1 hour.
    pub fn create_recovery(&self, email: &str, url: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/account/recovery";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("email".to_string(), ParamType::String(email.to_string())),
            ("url".to_string(), ParamType::String(url.to_string())),
        ].iter().cloned().collect();

        return self.client.clone().call("POST", &path, Some(headers), Some(params) );
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
    pub fn update_recovery(&self, user_id: &str, secret: &str, password: &str, password_again: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/account/recovery";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("userId".to_string(), ParamType::String(user_id.to_string())),
            ("secret".to_string(), ParamType::String(secret.to_string())),
            ("password".to_string(), ParamType::String(password.to_string())),
            ("passwordAgain".to_string(), ParamType::String(password_again.to_string())),
        ].iter().cloned().collect();

        return self.client.clone().call("PUT", &path, Some(headers), Some(params) );
    }

    /// Get currently logged in user list of active sessions across different
    /// devices.
    pub fn get_sessions(&self) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/account/sessions";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    /// Allow the user to login into their account by providing a valid email and
    /// password combination. This route will create a new session for the user.
    pub fn create_session(&self, email: &str, password: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/account/sessions";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("email".to_string(), ParamType::String(email.to_string())),
            ("password".to_string(), ParamType::String(password.to_string())),
        ].iter().cloned().collect();

        return self.client.clone().call("POST", &path, Some(headers), Some(params) );
    }

    /// Delete all sessions from the user account and remove any sessions cookies
    /// from the end client.
    pub fn delete_sessions(&self) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/account/sessions";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("DELETE", &path, Some(headers), Some(params) );
    }

    /// Use this endpoint to allow a new user to register an anonymous account in
    /// your project. This route will also create a new session for the user. To
    /// allow the new user to convert an anonymous account to a normal account
    /// account, you need to update its [email and
    /// password](/docs/client/account#accountUpdateEmail).
    pub fn create_anonymous_session(&self) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/account/sessions/anonymous";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("POST", &path, Some(headers), Some(params) );
    }

    /// Allow the user to login to their account using the OAuth2 provider of their
    /// choice. Each OAuth2 provider should be enabled from the Appwrite console
    /// first. Use the success and failure arguments to provide a redirect URL's
    /// back to your app when login is completed.
    pub fn create_o_auth2_session(&self, provider: &str, success: &str, failure: &str, scopes: &[&str]) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/account/sessions/oauth2/provider".replace("provider", &provider);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("success".to_string(), ParamType::String(success.to_string())),
            ("failure".to_string(), ParamType::String(failure.to_string())),
            ("scopes".to_string(), ParamType::Array(scopes.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    /// Use this endpoint to log out the currently logged in user from all their
    /// account sessions across all of their different devices. When using the
    /// option id argument, only the session unique ID provider will be deleted.
    pub fn delete_session(&self, session_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/account/sessions/sessionId".replace("sessionId", &session_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("DELETE", &path, Some(headers), Some(params) );
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
    pub fn create_verification(&self, url: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/account/verification";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("url".to_string(), ParamType::String(url.to_string())),
        ].iter().cloned().collect();

        return self.client.clone().call("POST", &path, Some(headers), Some(params) );
    }

    /// Use this endpoint to complete the user email verification process. Use both
    /// the **userId** and **secret** parameters that were attached to your app URL
    /// to verify the user email ownership. If confirmed this route will return a
    /// 200 status code.
    pub fn update_verification(&self, user_id: &str, secret: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/account/verification";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("userId".to_string(), ParamType::String(user_id.to_string())),
            ("secret".to_string(), ParamType::String(secret.to_string())),
        ].iter().cloned().collect();

        return self.client.clone().call("PUT", &path, Some(headers), Some(params) );
    }
}
