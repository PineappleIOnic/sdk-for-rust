use crate::client::{Client, ParamType};
use std::collections::HashMap;
use crate::services::AppwriteException;
use crate::models;

#[derive(Clone)]
pub struct Locale {
  client: Client
}

impl Locale {  
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone()
        }
    }

    /// Get the current user location based on IP. Returns an object with user
    /// country code, country name, continent name, continent code, ip address and
    /// suggested currency. You can use the locale header to get the data in a
    /// supported language.
    /// 
    /// ([IP Geolocation by DB-IP](https://db-ip.com))
    pub fn get(&self) -> Result<models::Locale, AppwriteException> {
        let path = "/locale";
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::Locale = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// List of all continents. You can use the locale header to get the data in a
    /// supported language.
    pub fn get_continents(&self) -> Result<models::ContinentList, AppwriteException> {
        let path = "/locale/continents";
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::ContinentList = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// List of all countries. You can use the locale header to get the data in a
    /// supported language.
    pub fn get_countries(&self) -> Result<models::CountryList, AppwriteException> {
        let path = "/locale/countries";
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::CountryList = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// List of all countries that are currently members of the EU. You can use the
    /// locale header to get the data in a supported language.
    pub fn get_countries_eu(&self) -> Result<models::CountryList, AppwriteException> {
        let path = "/locale/countries/eu";
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::CountryList = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// List of all countries phone codes. You can use the locale header to get the
    /// data in a supported language.
    pub fn get_countries_phones(&self) -> Result<models::PhoneList, AppwriteException> {
        let path = "/locale/countries/phones";
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::PhoneList = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// List of all currencies, including currency symbol, name, plural, and
    /// decimal digits for all major and minor currencies. You can use the locale
    /// header to get the data in a supported language.
    pub fn get_currencies(&self) -> Result<models::CurrencyList, AppwriteException> {
        let path = "/locale/currencies";
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::CurrencyList = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// List of all languages classified by ISO 639-1 including 2-letter code, name
    /// in English, and name in the respective language.
    pub fn get_languages(&self) -> Result<models::LanguageList, AppwriteException> {
        let path = "/locale/languages";
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::LanguageList = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }
}
