use crate::client::{Client, ParamType};
use std::collections::HashMap;
use crate::services::AppwriteException;
use crate::models;
use serde_json::json;
use std::io::Read;

#[derive(Clone)]
pub struct Projects {
  client: Client
}

impl Projects {  
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone()
        }
    }

    pub fn list(&self, search: Option<&str>, limit: Option<i64>, offset: Option<i64>, cursor: Option<&str>, cursor_direction: Option<&str>, order_type: Option<&str>) -> Result<models::ProjectList, AppwriteException> {
        let path = "/projects";
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

        let processedResponse:models::ProjectList = match response {
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

    pub fn create(&self, project_id: &str, name: &str, team_id: &str, description: Option<&str>, logo: Option<&str>, url: Option<&str>, legal_name: Option<&str>, legal_country: Option<&str>, legal_state: Option<&str>, legal_city: Option<&str>, legal_address: Option<&str>, legal_tax_id: Option<&str>) -> Result<models::Project, AppwriteException> {
        let path = "/projects";
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let description:&str = match description {
            Some(data) => data,
            None => ""
        };

        let logo:&str = match logo {
            Some(data) => data,
            None => ""
        };

        let url:&str = match url {
            Some(data) => data,
            None => ""
        };

        let legal_name:&str = match legal_name {
            Some(data) => data,
            None => ""
        };

        let legal_country:&str = match legal_country {
            Some(data) => data,
            None => ""
        };

        let legal_state:&str = match legal_state {
            Some(data) => data,
            None => ""
        };

        let legal_city:&str = match legal_city {
            Some(data) => data,
            None => ""
        };

        let legal_address:&str = match legal_address {
            Some(data) => data,
            None => ""
        };

        let legal_tax_id:&str = match legal_tax_id {
            Some(data) => data,
            None => ""
        };

        let  params: HashMap<String, ParamType> = [
            ("projectId".to_string(), ParamType::String(project_id.to_string())),
            ("name".to_string(), ParamType::String(name.to_string())),
            ("teamId".to_string(), ParamType::String(team_id.to_string())),
            ("description".to_string(), ParamType::String(description.to_string())),
            ("logo".to_string(), ParamType::String(logo.to_string())),
            ("url".to_string(), ParamType::String(url.to_string())),
            ("legalName".to_string(), ParamType::String(legal_name.to_string())),
            ("legalCountry".to_string(), ParamType::String(legal_country.to_string())),
            ("legalState".to_string(), ParamType::String(legal_state.to_string())),
            ("legalCity".to_string(), ParamType::String(legal_city.to_string())),
            ("legalAddress".to_string(), ParamType::String(legal_address.to_string())),
            ("legalTaxId".to_string(), ParamType::String(legal_tax_id.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("POST", &path, Some(headers), Some(params) );

        let processedResponse:models::Project = match response {
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

    pub fn get(&self, project_id: &str) -> Result<models::Project, AppwriteException> {
        let path = "/projects/projectId".replace("projectId", &project_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::Project = match response {
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

    pub fn update(&self, project_id: &str, name: &str, description: Option<&str>, logo: Option<&str>, url: Option<&str>, legal_name: Option<&str>, legal_country: Option<&str>, legal_state: Option<&str>, legal_city: Option<&str>, legal_address: Option<&str>, legal_tax_id: Option<&str>) -> Result<models::Project, AppwriteException> {
        let path = "/projects/projectId".replace("projectId", &project_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let description:&str = match description {
            Some(data) => data,
            None => ""
        };

        let logo:&str = match logo {
            Some(data) => data,
            None => ""
        };

        let url:&str = match url {
            Some(data) => data,
            None => ""
        };

        let legal_name:&str = match legal_name {
            Some(data) => data,
            None => ""
        };

        let legal_country:&str = match legal_country {
            Some(data) => data,
            None => ""
        };

        let legal_state:&str = match legal_state {
            Some(data) => data,
            None => ""
        };

        let legal_city:&str = match legal_city {
            Some(data) => data,
            None => ""
        };

        let legal_address:&str = match legal_address {
            Some(data) => data,
            None => ""
        };

        let legal_tax_id:&str = match legal_tax_id {
            Some(data) => data,
            None => ""
        };

        let  params: HashMap<String, ParamType> = [
            ("name".to_string(), ParamType::String(name.to_string())),
            ("description".to_string(), ParamType::String(description.to_string())),
            ("logo".to_string(), ParamType::String(logo.to_string())),
            ("url".to_string(), ParamType::String(url.to_string())),
            ("legalName".to_string(), ParamType::String(legal_name.to_string())),
            ("legalCountry".to_string(), ParamType::String(legal_country.to_string())),
            ("legalState".to_string(), ParamType::String(legal_state.to_string())),
            ("legalCity".to_string(), ParamType::String(legal_city.to_string())),
            ("legalAddress".to_string(), ParamType::String(legal_address.to_string())),
            ("legalTaxId".to_string(), ParamType::String(legal_tax_id.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("PATCH", &path, Some(headers), Some(params) );

        let processedResponse:models::Project = match response {
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

    pub fn delete(&self, project_id: &str, password: &str) -> Result<serde_json::value::Value, AppwriteException> {
        let path = "/projects/projectId".replace("projectId", &project_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
            ("password".to_string(), ParamType::String(password.to_string())),
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

    pub fn update_auth_limit(&self, project_id: &str, limit: i64) -> Result<models::Project, AppwriteException> {
        let path = "/projects/projectId/auth/limit".replace("projectId", &project_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
            ("limit".to_string(),  ParamType::Number(limit)),
        ].iter().cloned().collect();

        let response = self.client.clone().call("PATCH", &path, Some(headers), Some(params) );

        let processedResponse:models::Project = match response {
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

    pub fn update_auth_status(&self, project_id: &str, method: &str, status: bool) -> Result<models::Project, AppwriteException> {
        let path = "/projects/projectId/auth/method".replace("projectId", &project_id).replace("method", &method);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
            ("status".to_string(), ParamType::Bool(status)),
        ].iter().cloned().collect();

        let response = self.client.clone().call("PATCH", &path, Some(headers), Some(params) );

        let processedResponse:models::Project = match response {
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

    pub fn list_domains(&self, project_id: &str) -> Result<models::DomainList, AppwriteException> {
        let path = "/projects/projectId/domains".replace("projectId", &project_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::DomainList = match response {
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

    pub fn create_domain(&self, project_id: &str, domain: &str) -> Result<models::Domain, AppwriteException> {
        let path = "/projects/projectId/domains".replace("projectId", &project_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
            ("domain".to_string(), ParamType::String(domain.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("POST", &path, Some(headers), Some(params) );

        let processedResponse:models::Domain = match response {
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

    pub fn get_domain(&self, project_id: &str, domain_id: &str) -> Result<models::Domain, AppwriteException> {
        let path = "/projects/projectId/domains/domainId".replace("projectId", &project_id).replace("domainId", &domain_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::Domain = match response {
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

    pub fn delete_domain(&self, project_id: &str, domain_id: &str) -> Result<serde_json::value::Value, AppwriteException> {
        let path = "/projects/projectId/domains/domainId".replace("projectId", &project_id).replace("domainId", &domain_id);
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

    pub fn update_domain_verification(&self, project_id: &str, domain_id: &str) -> Result<models::Domain, AppwriteException> {
        let path = "/projects/projectId/domains/domainId/verification".replace("projectId", &project_id).replace("domainId", &domain_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("PATCH", &path, Some(headers), Some(params) );

        let processedResponse:models::Domain = match response {
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

    pub fn list_keys(&self, project_id: &str) -> Result<models::KeyList, AppwriteException> {
        let path = "/projects/projectId/keys".replace("projectId", &project_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::KeyList = match response {
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

    pub fn create_key(&self, project_id: &str, name: &str, scopes: &[&str]) -> Result<models::Key, AppwriteException> {
        let path = "/projects/projectId/keys".replace("projectId", &project_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
            ("name".to_string(), ParamType::String(name.to_string())),
            ("scopes".to_string(), ParamType::Array(scopes.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("POST", &path, Some(headers), Some(params) );

        let processedResponse:models::Key = match response {
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

    pub fn get_key(&self, project_id: &str, key_id: &str) -> Result<models::Key, AppwriteException> {
        let path = "/projects/projectId/keys/keyId".replace("projectId", &project_id).replace("keyId", &key_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::Key = match response {
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

    pub fn update_key(&self, project_id: &str, key_id: &str, name: &str, scopes: &[&str]) -> Result<models::Key, AppwriteException> {
        let path = "/projects/projectId/keys/keyId".replace("projectId", &project_id).replace("keyId", &key_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
            ("name".to_string(), ParamType::String(name.to_string())),
            ("scopes".to_string(), ParamType::Array(scopes.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("PUT", &path, Some(headers), Some(params) );

        let processedResponse:models::Key = match response {
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

    pub fn delete_key(&self, project_id: &str, key_id: &str) -> Result<serde_json::value::Value, AppwriteException> {
        let path = "/projects/projectId/keys/keyId".replace("projectId", &project_id).replace("keyId", &key_id);
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

    pub fn update_o_auth2(&self, project_id: &str, provider: &str, app_id: Option<&str>, secret: Option<&str>) -> Result<models::Project, AppwriteException> {
        let path = "/projects/projectId/oauth2".replace("projectId", &project_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let app_id:&str = match app_id {
            Some(data) => data,
            None => ""
        };

        let secret:&str = match secret {
            Some(data) => data,
            None => ""
        };

        let  params: HashMap<String, ParamType> = [
            ("provider".to_string(), ParamType::String(provider.to_string())),
            ("appId".to_string(), ParamType::String(app_id.to_string())),
            ("secret".to_string(), ParamType::String(secret.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("PATCH", &path, Some(headers), Some(params) );

        let processedResponse:models::Project = match response {
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

    pub fn list_platforms(&self, project_id: &str) -> Result<models::PlatformList, AppwriteException> {
        let path = "/projects/projectId/platforms".replace("projectId", &project_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::PlatformList = match response {
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

    pub fn create_platform(&self, project_id: &str, xtype: &str, name: &str, key: Option<&str>, store: Option<&str>, hostname: Option<&str>) -> Result<models::Platform, AppwriteException> {
        let path = "/projects/projectId/platforms".replace("projectId", &project_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let key:&str = match key {
            Some(data) => data,
            None => ""
        };

        let store:&str = match store {
            Some(data) => data,
            None => ""
        };

        let hostname:&str = match hostname {
            Some(data) => data,
            None => ""
        };

        let  params: HashMap<String, ParamType> = [
            ("type".to_string(), ParamType::String(xtype.to_string())),
            ("name".to_string(), ParamType::String(name.to_string())),
            ("key".to_string(), ParamType::String(key.to_string())),
            ("store".to_string(), ParamType::String(store.to_string())),
            ("hostname".to_string(), ParamType::String(hostname.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("POST", &path, Some(headers), Some(params) );

        let processedResponse:models::Platform = match response {
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

    pub fn get_platform(&self, project_id: &str, platform_id: &str) -> Result<models::Platform, AppwriteException> {
        let path = "/projects/projectId/platforms/platformId".replace("projectId", &project_id).replace("platformId", &platform_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::Platform = match response {
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

    pub fn update_platform(&self, project_id: &str, platform_id: &str, name: &str, key: Option<&str>, store: Option<&str>, hostname: Option<&str>) -> Result<models::Platform, AppwriteException> {
        let path = "/projects/projectId/platforms/platformId".replace("projectId", &project_id).replace("platformId", &platform_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let key:&str = match key {
            Some(data) => data,
            None => ""
        };

        let store:&str = match store {
            Some(data) => data,
            None => ""
        };

        let hostname:&str = match hostname {
            Some(data) => data,
            None => ""
        };

        let  params: HashMap<String, ParamType> = [
            ("name".to_string(), ParamType::String(name.to_string())),
            ("key".to_string(), ParamType::String(key.to_string())),
            ("store".to_string(), ParamType::String(store.to_string())),
            ("hostname".to_string(), ParamType::String(hostname.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("PUT", &path, Some(headers), Some(params) );

        let processedResponse:models::Platform = match response {
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

    pub fn delete_platform(&self, project_id: &str, platform_id: &str) -> Result<serde_json::value::Value, AppwriteException> {
        let path = "/projects/projectId/platforms/platformId".replace("projectId", &project_id).replace("platformId", &platform_id);
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

    pub fn update_service_status(&self, project_id: &str, service: &str, status: bool) -> Result<models::Project, AppwriteException> {
        let path = "/projects/projectId/service".replace("projectId", &project_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
            ("service".to_string(), ParamType::String(service.to_string())),
            ("status".to_string(), ParamType::Bool(status)),
        ].iter().cloned().collect();

        let response = self.client.clone().call("PATCH", &path, Some(headers), Some(params) );

        let processedResponse:models::Project = match response {
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

    pub fn get_usage(&self, project_id: &str, range: Option<&str>) -> Result<models::UsageProject, AppwriteException> {
        let path = "/projects/projectId/usage".replace("projectId", &project_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let range:&str = match range {
            Some(data) => data,
            None => ""
        };

        let  params: HashMap<String, ParamType> = [
            ("range".to_string(), ParamType::String(range.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::UsageProject = match response {
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

    pub fn list_webhooks(&self, project_id: &str) -> Result<models::WebhookList, AppwriteException> {
        let path = "/projects/projectId/webhooks".replace("projectId", &project_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::WebhookList = match response {
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

    pub fn create_webhook(&self, project_id: &str, name: &str, events: &[&str], url: &str, security: bool, http_user: Option<&str>, http_pass: Option<&str>) -> Result<models::Webhook, AppwriteException> {
        let path = "/projects/projectId/webhooks".replace("projectId", &project_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let http_user:&str = match http_user {
            Some(data) => data,
            None => ""
        };

        let http_pass:&str = match http_pass {
            Some(data) => data,
            None => ""
        };

        let  params: HashMap<String, ParamType> = [
            ("name".to_string(), ParamType::String(name.to_string())),
            ("events".to_string(), ParamType::Array(events.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("url".to_string(), ParamType::String(url.to_string())),
            ("security".to_string(), ParamType::Bool(security)),
            ("httpUser".to_string(), ParamType::String(http_user.to_string())),
            ("httpPass".to_string(), ParamType::String(http_pass.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("POST", &path, Some(headers), Some(params) );

        let processedResponse:models::Webhook = match response {
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

    pub fn get_webhook(&self, project_id: &str, webhook_id: &str) -> Result<models::Webhook, AppwriteException> {
        let path = "/projects/projectId/webhooks/webhookId".replace("projectId", &project_id).replace("webhookId", &webhook_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::Webhook = match response {
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

    pub fn update_webhook(&self, project_id: &str, webhook_id: &str, name: &str, events: &[&str], url: &str, security: bool, http_user: Option<&str>, http_pass: Option<&str>) -> Result<models::Webhook, AppwriteException> {
        let path = "/projects/projectId/webhooks/webhookId".replace("projectId", &project_id).replace("webhookId", &webhook_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let http_user:&str = match http_user {
            Some(data) => data,
            None => ""
        };

        let http_pass:&str = match http_pass {
            Some(data) => data,
            None => ""
        };

        let  params: HashMap<String, ParamType> = [
            ("name".to_string(), ParamType::String(name.to_string())),
            ("events".to_string(), ParamType::Array(events.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("url".to_string(), ParamType::String(url.to_string())),
            ("security".to_string(), ParamType::Bool(security)),
            ("httpUser".to_string(), ParamType::String(http_user.to_string())),
            ("httpPass".to_string(), ParamType::String(http_pass.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("PUT", &path, Some(headers), Some(params) );

        let processedResponse:models::Webhook = match response {
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

    pub fn delete_webhook(&self, project_id: &str, webhook_id: &str) -> Result<serde_json::value::Value, AppwriteException> {
        let path = "/projects/projectId/webhooks/webhookId".replace("projectId", &project_id).replace("webhookId", &webhook_id);
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
}
