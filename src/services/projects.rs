use crate::client::{Client, ParamType};
use std::collections::HashMap;
use crate::services::AppwriteException;

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

    pub fn list(&self, search: &str, limit: i64, offset: i64, order_type: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects";

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

    pub fn create(&self, name: &str, team_id: &str, description: &str, logo: &str, url: &str, legal_name: &str, legal_country: &str, legal_state: &str, legal_city: &str, legal_address: &str, legal_tax_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
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

        return self.client.clone().call("POST", &path, Some(headers), Some(params) );
    }

    pub fn get(&self, project_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId".replace("projectId", &project_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    pub fn update(&self, project_id: &str, name: &str, description: &str, logo: &str, url: &str, legal_name: &str, legal_country: &str, legal_state: &str, legal_city: &str, legal_address: &str, legal_tax_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId".replace("projectId", &project_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
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

        return self.client.clone().call("PATCH", &path, Some(headers), Some(params) );
    }

    pub fn delete(&self, project_id: &str, password: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId".replace("projectId", &project_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("password".to_string(), ParamType::String(password.to_string())),
        ].iter().cloned().collect();

        return self.client.clone().call("DELETE", &path, Some(headers), Some(params) );
    }

    pub fn update_auth_limit(&self, project_id: &str, limit: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/auth/limit".replace("projectId", &project_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("limit".to_string(), ParamType::String(limit.to_string())),
        ].iter().cloned().collect();

        return self.client.clone().call("PATCH", &path, Some(headers), Some(params) );
    }

    pub fn update_auth_status(&self, project_id: &str, method: &str, status: bool) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/auth/method".replace("projectId", &project_id).replace("method", &method);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("status".to_string(), ParamType::Bool(status)),
        ].iter().cloned().collect();

        return self.client.clone().call("PATCH", &path, Some(headers), Some(params) );
    }

    pub fn list_domains(&self, project_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/domains".replace("projectId", &project_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    pub fn create_domain(&self, project_id: &str, domain: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/domains".replace("projectId", &project_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("domain".to_string(), ParamType::String(domain.to_string())),
        ].iter().cloned().collect();

        return self.client.clone().call("POST", &path, Some(headers), Some(params) );
    }

    pub fn get_domain(&self, project_id: &str, domain_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/domains/domainId".replace("projectId", &project_id).replace("domainId", &domain_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    pub fn delete_domain(&self, project_id: &str, domain_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/domains/domainId".replace("projectId", &project_id).replace("domainId", &domain_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("DELETE", &path, Some(headers), Some(params) );
    }

    pub fn update_domain_verification(&self, project_id: &str, domain_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/domains/domainId/verification".replace("projectId", &project_id).replace("domainId", &domain_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("PATCH", &path, Some(headers), Some(params) );
    }

    pub fn list_keys(&self, project_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/keys".replace("projectId", &project_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    pub fn create_key(&self, project_id: &str, name: &str, scopes: &[&str]) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/keys".replace("projectId", &project_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("name".to_string(), ParamType::String(name.to_string())),
            ("scopes".to_string(), ParamType::Array(scopes.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
        ].iter().cloned().collect();

        return self.client.clone().call("POST", &path, Some(headers), Some(params) );
    }

    pub fn get_key(&self, project_id: &str, key_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/keys/keyId".replace("projectId", &project_id).replace("keyId", &key_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    pub fn update_key(&self, project_id: &str, key_id: &str, name: &str, scopes: &[&str]) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/keys/keyId".replace("projectId", &project_id).replace("keyId", &key_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("name".to_string(), ParamType::String(name.to_string())),
            ("scopes".to_string(), ParamType::Array(scopes.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
        ].iter().cloned().collect();

        return self.client.clone().call("PUT", &path, Some(headers), Some(params) );
    }

    pub fn delete_key(&self, project_id: &str, key_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/keys/keyId".replace("projectId", &project_id).replace("keyId", &key_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("DELETE", &path, Some(headers), Some(params) );
    }

    pub fn update_o_auth2(&self, project_id: &str, provider: &str, app_id: &str, secret: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/oauth2".replace("projectId", &project_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("provider".to_string(), ParamType::String(provider.to_string())),
            ("appId".to_string(), ParamType::String(app_id.to_string())),
            ("secret".to_string(), ParamType::String(secret.to_string())),
        ].iter().cloned().collect();

        return self.client.clone().call("PATCH", &path, Some(headers), Some(params) );
    }

    pub fn list_platforms(&self, project_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/platforms".replace("projectId", &project_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    pub fn create_platform(&self, project_id: &str, xtype: &str, name: &str, key: &str, store: &str, hostname: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/platforms".replace("projectId", &project_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("type".to_string(), ParamType::String(xtype.to_string())),
            ("name".to_string(), ParamType::String(name.to_string())),
            ("key".to_string(), ParamType::String(key.to_string())),
            ("store".to_string(), ParamType::String(store.to_string())),
            ("hostname".to_string(), ParamType::String(hostname.to_string())),
        ].iter().cloned().collect();

        return self.client.clone().call("POST", &path, Some(headers), Some(params) );
    }

    pub fn get_platform(&self, project_id: &str, platform_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/platforms/platformId".replace("projectId", &project_id).replace("platformId", &platform_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    pub fn update_platform(&self, project_id: &str, platform_id: &str, name: &str, key: &str, store: &str, hostname: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/platforms/platformId".replace("projectId", &project_id).replace("platformId", &platform_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("name".to_string(), ParamType::String(name.to_string())),
            ("key".to_string(), ParamType::String(key.to_string())),
            ("store".to_string(), ParamType::String(store.to_string())),
            ("hostname".to_string(), ParamType::String(hostname.to_string())),
        ].iter().cloned().collect();

        return self.client.clone().call("PUT", &path, Some(headers), Some(params) );
    }

    pub fn delete_platform(&self, project_id: &str, platform_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/platforms/platformId".replace("projectId", &project_id).replace("platformId", &platform_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("DELETE", &path, Some(headers), Some(params) );
    }

    pub fn list_tasks(&self, project_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/tasks".replace("projectId", &project_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    pub fn create_task(&self, project_id: &str, name: &str, status: &str, schedule: &str, security: bool, http_method: &str, http_url: &str, http_headers: &[&str], http_user: &str, http_pass: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/tasks".replace("projectId", &project_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("name".to_string(), ParamType::String(name.to_string())),
            ("status".to_string(), ParamType::String(status.to_string())),
            ("schedule".to_string(), ParamType::String(schedule.to_string())),
            ("security".to_string(), ParamType::Bool(security)),
            ("httpMethod".to_string(), ParamType::String(http_method.to_string())),
            ("httpUrl".to_string(), ParamType::String(http_url.to_string())),
            ("httpHeaders".to_string(), ParamType::Array(http_headers.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("httpUser".to_string(), ParamType::String(http_user.to_string())),
            ("httpPass".to_string(), ParamType::String(http_pass.to_string())),
        ].iter().cloned().collect();

        return self.client.clone().call("POST", &path, Some(headers), Some(params) );
    }

    pub fn get_task(&self, project_id: &str, task_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/tasks/taskId".replace("projectId", &project_id).replace("taskId", &task_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    pub fn update_task(&self, project_id: &str, task_id: &str, name: &str, status: &str, schedule: &str, security: bool, http_method: &str, http_url: &str, http_headers: &[&str], http_user: &str, http_pass: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/tasks/taskId".replace("projectId", &project_id).replace("taskId", &task_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("name".to_string(), ParamType::String(name.to_string())),
            ("status".to_string(), ParamType::String(status.to_string())),
            ("schedule".to_string(), ParamType::String(schedule.to_string())),
            ("security".to_string(), ParamType::Bool(security)),
            ("httpMethod".to_string(), ParamType::String(http_method.to_string())),
            ("httpUrl".to_string(), ParamType::String(http_url.to_string())),
            ("httpHeaders".to_string(), ParamType::Array(http_headers.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("httpUser".to_string(), ParamType::String(http_user.to_string())),
            ("httpPass".to_string(), ParamType::String(http_pass.to_string())),
        ].iter().cloned().collect();

        return self.client.clone().call("PUT", &path, Some(headers), Some(params) );
    }

    pub fn delete_task(&self, project_id: &str, task_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/tasks/taskId".replace("projectId", &project_id).replace("taskId", &task_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("DELETE", &path, Some(headers), Some(params) );
    }

    pub fn get_usage(&self, project_id: &str, range: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/usage".replace("projectId", &project_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("range".to_string(), ParamType::String(range.to_string())),
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    pub fn list_webhooks(&self, project_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/webhooks".replace("projectId", &project_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    pub fn create_webhook(&self, project_id: &str, name: &str, events: &[&str], url: &str, security: bool, http_user: &str, http_pass: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/webhooks".replace("projectId", &project_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("name".to_string(), ParamType::String(name.to_string())),
            ("events".to_string(), ParamType::Array(events.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("url".to_string(), ParamType::String(url.to_string())),
            ("security".to_string(), ParamType::Bool(security)),
            ("httpUser".to_string(), ParamType::String(http_user.to_string())),
            ("httpPass".to_string(), ParamType::String(http_pass.to_string())),
        ].iter().cloned().collect();

        return self.client.clone().call("POST", &path, Some(headers), Some(params) );
    }

    pub fn get_webhook(&self, project_id: &str, webhook_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/webhooks/webhookId".replace("projectId", &project_id).replace("webhookId", &webhook_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    pub fn update_webhook(&self, project_id: &str, webhook_id: &str, name: &str, events: &[&str], url: &str, security: bool, http_user: &str, http_pass: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/webhooks/webhookId".replace("projectId", &project_id).replace("webhookId", &webhook_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("name".to_string(), ParamType::String(name.to_string())),
            ("events".to_string(), ParamType::Array(events.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("url".to_string(), ParamType::String(url.to_string())),
            ("security".to_string(), ParamType::Bool(security)),
            ("httpUser".to_string(), ParamType::String(http_user.to_string())),
            ("httpPass".to_string(), ParamType::String(http_pass.to_string())),
        ].iter().cloned().collect();

        return self.client.clone().call("PUT", &path, Some(headers), Some(params) );
    }

    pub fn delete_webhook(&self, project_id: &str, webhook_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/projects/projectId/webhooks/webhookId".replace("projectId", &project_id).replace("webhookId", &webhook_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("DELETE", &path, Some(headers), Some(params) );
    }
}
