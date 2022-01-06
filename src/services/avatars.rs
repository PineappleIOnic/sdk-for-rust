use crate::client::{Client, ParamType};
use std::collections::HashMap;
use crate::services::AppwriteException;
use crate::models;
use serde_json::json;

#[derive(Clone)]
pub struct Avatars {
  client: Client
}

impl Avatars {  
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone()
        }
    }

    /// You can use this endpoint to show different browser icons to your users.
    /// The code argument receives the browser code as it appears in your user
    /// /account/sessions endpoint. Use width, height and quality arguments to
    /// change the output settings.
    pub fn get_browser(&self, code: &str, width: Option<i64>, height: Option<i64>, quality: Option<i64>) -> Result<Vec<u8>, AppwriteException> {
        let path = "/avatars/browsers/code".replace("code", &code);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("width".to_string(),  ParamType::OptionalNumber(width)),
            ("height".to_string(),  ParamType::OptionalNumber(height)),
            ("quality".to_string(),  ParamType::OptionalNumber(quality)),
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:Vec<u8> = match response {
            Ok(mut r) => {
                let mut buf: Vec<u8> = vec![];
                match r.copy_to(&mut buf) {
                    Ok(_) => (),
                    Err(e) => {
                        return Err(AppwriteException::new(format!("Error copying response to buffer: {}", e), 0, "".to_string()));
                    }
                };
                buf
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// The credit card endpoint will return you the icon of the credit card
    /// provider you need. Use width, height and quality arguments to change the
    /// output settings.
    pub fn get_credit_card(&self, code: &str, width: Option<i64>, height: Option<i64>, quality: Option<i64>) -> Result<Vec<u8>, AppwriteException> {
        let path = "/avatars/credit-cards/code".replace("code", &code);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("width".to_string(),  ParamType::OptionalNumber(width)),
            ("height".to_string(),  ParamType::OptionalNumber(height)),
            ("quality".to_string(),  ParamType::OptionalNumber(quality)),
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:Vec<u8> = match response {
            Ok(mut r) => {
                let mut buf: Vec<u8> = vec![];
                match r.copy_to(&mut buf) {
                    Ok(_) => (),
                    Err(e) => {
                        return Err(AppwriteException::new(format!("Error copying response to buffer: {}", e), 0, "".to_string()));
                    }
                };
                buf
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Use this endpoint to fetch the favorite icon (AKA favicon) of any remote
    /// website URL.
    /// 
    pub fn get_favicon(&self, url: &str) -> Result<Vec<u8>, AppwriteException> {
        let path = "/avatars/favicon";
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("url".to_string(), ParamType::String(url.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:Vec<u8> = match response {
            Ok(mut r) => {
                let mut buf: Vec<u8> = vec![];
                match r.copy_to(&mut buf) {
                    Ok(_) => (),
                    Err(e) => {
                        return Err(AppwriteException::new(format!("Error copying response to buffer: {}", e), 0, "".to_string()));
                    }
                };
                buf
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// You can use this endpoint to show different country flags icons to your
    /// users. The code argument receives the 2 letter country code. Use width,
    /// height and quality arguments to change the output settings.
    pub fn get_flag(&self, code: &str, width: Option<i64>, height: Option<i64>, quality: Option<i64>) -> Result<Vec<u8>, AppwriteException> {
        let path = "/avatars/flags/code".replace("code", &code);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("width".to_string(),  ParamType::OptionalNumber(width)),
            ("height".to_string(),  ParamType::OptionalNumber(height)),
            ("quality".to_string(),  ParamType::OptionalNumber(quality)),
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:Vec<u8> = match response {
            Ok(mut r) => {
                let mut buf: Vec<u8> = vec![];
                match r.copy_to(&mut buf) {
                    Ok(_) => (),
                    Err(e) => {
                        return Err(AppwriteException::new(format!("Error copying response to buffer: {}", e), 0, "".to_string()));
                    }
                };
                buf
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Use this endpoint to fetch a remote image URL and crop it to any image size
    /// you want. This endpoint is very useful if you need to crop and display
    /// remote images in your app or in case you want to make sure a 3rd party
    /// image is properly served using a TLS protocol.
    pub fn get_image(&self, url: &str, width: Option<i64>, height: Option<i64>) -> Result<Vec<u8>, AppwriteException> {
        let path = "/avatars/image";
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("url".to_string(), ParamType::String(url.to_string())),
            ("width".to_string(),  ParamType::OptionalNumber(width)),
            ("height".to_string(),  ParamType::OptionalNumber(height)),
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:Vec<u8> = match response {
            Ok(mut r) => {
                let mut buf: Vec<u8> = vec![];
                match r.copy_to(&mut buf) {
                    Ok(_) => (),
                    Err(e) => {
                        return Err(AppwriteException::new(format!("Error copying response to buffer: {}", e), 0, "".to_string()));
                    }
                };
                buf
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Use this endpoint to show your user initials avatar icon on your website or
    /// app. By default, this route will try to print your logged-in user name or
    /// email initials. You can also overwrite the user name if you pass the 'name'
    /// parameter. If no name is given and no user is logged, an empty avatar will
    /// be returned.
    /// 
    /// You can use the color and background params to change the avatar colors. By
    /// default, a random theme will be selected. The random theme will persist for
    /// the user's initials when reloading the same theme will always return for
    /// the same initials.
    pub fn get_initials(&self, name: Option<&str>, width: Option<i64>, height: Option<i64>, color: Option<&str>, background: Option<&str>) -> Result<Vec<u8>, AppwriteException> {
        let path = "/avatars/initials";
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let name:&str = match name {
            Some(data) => data,
            None => ""
        };

        let color:&str = match color {
            Some(data) => data,
            None => ""
        };

        let background:&str = match background {
            Some(data) => data,
            None => ""
        };

        let params: HashMap<String, ParamType> = [
            ("name".to_string(), ParamType::String(name.to_string())),
            ("width".to_string(),  ParamType::OptionalNumber(width)),
            ("height".to_string(),  ParamType::OptionalNumber(height)),
            ("color".to_string(), ParamType::String(color.to_string())),
            ("background".to_string(), ParamType::String(background.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:Vec<u8> = match response {
            Ok(mut r) => {
                let mut buf: Vec<u8> = vec![];
                match r.copy_to(&mut buf) {
                    Ok(_) => (),
                    Err(e) => {
                        return Err(AppwriteException::new(format!("Error copying response to buffer: {}", e), 0, "".to_string()));
                    }
                };
                buf
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Converts a given plain text to a QR code image. You can use the query
    /// parameters to change the size and style of the resulting image.
    pub fn get_qr(&self, text: &str, size: Option<i64>, margin: Option<i64>, download: Option<bool>) -> Result<Vec<u8>, AppwriteException> {
        let path = "/avatars/qr";
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("text".to_string(), ParamType::String(text.to_string())),
            ("size".to_string(),  ParamType::OptionalNumber(size)),
            ("margin".to_string(),  ParamType::OptionalNumber(margin)),
            ("download".to_string(), ParamType::OptionalBool(download)),
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:Vec<u8> = match response {
            Ok(mut r) => {
                let mut buf: Vec<u8> = vec![];
                match r.copy_to(&mut buf) {
                    Ok(_) => (),
                    Err(e) => {
                        return Err(AppwriteException::new(format!("Error copying response to buffer: {}", e), 0, "".to_string()));
                    }
                };
                buf
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }
}
