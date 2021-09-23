use crate::client::{Client, ParamType};
use std::collections::HashMap;
use crate::services::AppwriteException;
use crate::models;

#[derive(Clone)]
pub struct Functions {
  client: Client
}

impl Functions {  
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone()
        }
    }

    /// Get a list of all the current user function execution logs. You can use the
    /// query params to filter your results. On admin mode, this endpoint will
    /// return a list of all of the project's executions. [Learn more about
    /// different API modes](/docs/admin).
    pub fn list_executions(&self, function_id: &str, search: Option<&str>, limit: Option<i64>, offset: Option<i64>, order_type: Option<&str>) -> Result<models::ExecutionList, AppwriteException> {
        let path = "/functions/functionId/executions".replace("functionId", &function_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let search:&str = match search {
            Some(data) => data,
            None => ""
        };

        let order_type:&str = match order_type {
            Some(data) => data,
            None => ""
        };

        let params: HashMap<String, ParamType> = [
            ("search".to_string(), ParamType::String(search.to_string())),
            ("limit".to_string(),  ParamType::OptionalNumber(limit)),
            ("offset".to_string(),  ParamType::OptionalNumber(offset)),
            ("orderType".to_string(), ParamType::String(order_type.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::ExecutionList = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Trigger a function execution. The returned object will return you the
    /// current execution status. You can ping the `Get Execution` endpoint to get
    /// updates on the current execution status. Once this endpoint is called, your
    /// function execution process will start asynchronously.
    pub fn create_execution(&self, function_id: &str, data: Option<&str>) -> Result<models::Execution, AppwriteException> {
        let path = "/functions/functionId/executions".replace("functionId", &function_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let data:&str = match data {
            Some(data) => data,
            None => ""
        };

        let params: HashMap<String, ParamType> = [
            ("data".to_string(), ParamType::String(data.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("POST", &path, Some(headers), Some(params) );

        let processedResponse:models::Execution = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Get a function execution log by its unique ID.
    pub fn get_execution(&self, function_id: &str, execution_id: &str) -> Result<models::Execution, AppwriteException> {
        let path = "/functions/functionId/executions/executionId".replace("functionId", &function_id).replace("executionId", &execution_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::Execution = match response {
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
