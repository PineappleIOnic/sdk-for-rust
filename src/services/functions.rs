use crate::client::{Client, ParamType};
use std::collections::HashMap;
use crate::services::AppwriteException;
use crate::models;
use serde_json::json;

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

    /// Get a list of all the project's functions. You can use the query params to
    /// filter your results.
    pub fn list(&self, search: Option<&str>, limit: Option<i64>, offset: Option<i64>, cursor: Option<&str>, cursor_direction: Option<&str>, order_type: Option<&str>) -> Result<models::FunctionList, AppwriteException> {
        let path = "/functions";
        let headers: HashMap<String, String> = [
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

        let params: HashMap<String, ParamType> = [
            ("search".to_string(), ParamType::String(search.to_string())),
            ("limit".to_string(),  ParamType::OptionalNumber(limit)),
            ("offset".to_string(),  ParamType::OptionalNumber(offset)),
            ("cursor".to_string(), ParamType::String(cursor.to_string())),
            ("cursorDirection".to_string(), ParamType::String(cursor_direction.to_string())),
            ("orderType".to_string(), ParamType::String(order_type.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::FunctionList = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Create a new function. You can pass a list of
    /// [permissions](/docs/permissions) to allow different project users or team
    /// with access to execute the function using the client API.
    pub fn create(&self, function_id: &str, name: &str, execute: &[&str], runtime: &str, vars: Option<Option<HashMap<String, crate::client::ParamType>>>, events: Option<&[&str]>, schedule: Option<&str>, timeout: Option<i64>) -> Result<models::Function, AppwriteException> {
        let path = "/functions";
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let events:&[&str] = match events {
            Some(data) => data,
            None => &[]
        };

        let schedule:&str = match schedule {
            Some(data) => data,
            None => ""
        };

        let params: HashMap<String, ParamType> = [
            ("functionId".to_string(), ParamType::String(function_id.to_string())),
            ("name".to_string(), ParamType::String(name.to_string())),
            ("execute".to_string(), ParamType::Array(execute.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("runtime".to_string(), ParamType::String(runtime.to_string())),
            ("vars".to_string(), ParamType::OptionalObject(vars.unwrap())),
            ("events".to_string(), ParamType::Array(events.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("schedule".to_string(), ParamType::String(schedule.to_string())),
            ("timeout".to_string(),  ParamType::OptionalNumber(timeout)),
        ].iter().cloned().collect();

        let response = self.client.clone().call("POST", &path, Some(headers), Some(params) );

        let processedResponse:models::Function = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Get a function by its unique ID.
    pub fn get(&self, function_id: &str) -> Result<models::Function, AppwriteException> {
        let path = "/functions/functionId".replace("functionId", &function_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::Function = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Update function by its unique ID.
    pub fn update(&self, function_id: &str, name: &str, execute: &[&str], vars: Option<Option<HashMap<String, crate::client::ParamType>>>, events: Option<&[&str]>, schedule: Option<&str>, timeout: Option<i64>) -> Result<models::Function, AppwriteException> {
        let path = "/functions/functionId".replace("functionId", &function_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let events:&[&str] = match events {
            Some(data) => data,
            None => &[]
        };

        let schedule:&str = match schedule {
            Some(data) => data,
            None => ""
        };

        let params: HashMap<String, ParamType> = [
            ("name".to_string(), ParamType::String(name.to_string())),
            ("execute".to_string(), ParamType::Array(execute.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("vars".to_string(), ParamType::OptionalObject(vars.unwrap())),
            ("events".to_string(), ParamType::Array(events.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("schedule".to_string(), ParamType::String(schedule.to_string())),
            ("timeout".to_string(),  ParamType::OptionalNumber(timeout)),
        ].iter().cloned().collect();

        let response = self.client.clone().call("PUT", &path, Some(headers), Some(params) );

        let processedResponse:models::Function = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Delete a function by its unique ID.
    pub fn delete(&self, function_id: &str) -> Result<serde_json::value::Value, AppwriteException> {
        let path = "/functions/functionId".replace("functionId", &function_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("DELETE", &path, Some(headers), Some(params) );

        match response {
            Ok(r) => {
                Ok(serde_json::from_str(&r.text().unwrap()).unwrap())
            }
            Err(e) => {
                Err(e)
            }
        }

    }

    /// Get a list of all the current user function execution logs. You can use the
    /// query params to filter your results. On admin mode, this endpoint will
    /// return a list of all of the project's executions. [Learn more about
    /// different API modes](/docs/admin).
    pub fn list_executions(&self, function_id: &str, limit: Option<i64>, offset: Option<i64>, search: Option<&str>, cursor: Option<&str>, cursor_direction: Option<&str>) -> Result<models::ExecutionList, AppwriteException> {
        let path = "/functions/functionId/executions".replace("functionId", &function_id);
        let headers: HashMap<String, String> = [
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

        let params: HashMap<String, ParamType> = [
            ("limit".to_string(),  ParamType::OptionalNumber(limit)),
            ("offset".to_string(),  ParamType::OptionalNumber(offset)),
            ("search".to_string(), ParamType::String(search.to_string())),
            ("cursor".to_string(), ParamType::String(cursor.to_string())),
            ("cursorDirection".to_string(), ParamType::String(cursor_direction.to_string())),
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

    /// Update the function code tag ID using the unique function ID. Use this
    /// endpoint to switch the code tag that should be executed by the execution
    /// endpoint.
    pub fn update_tag(&self, function_id: &str, tag: &str) -> Result<models::Function, AppwriteException> {
        let path = "/functions/functionId/tag".replace("functionId", &function_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("tag".to_string(), ParamType::String(tag.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("PATCH", &path, Some(headers), Some(params) );

        let processedResponse:models::Function = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Get a list of all the project's code tags. You can use the query params to
    /// filter your results.
    pub fn list_tags(&self, function_id: &str, search: Option<&str>, limit: Option<i64>, offset: Option<i64>, cursor: Option<&str>, cursor_direction: Option<&str>, order_type: Option<&str>) -> Result<models::TagList, AppwriteException> {
        let path = "/functions/functionId/tags".replace("functionId", &function_id);
        let headers: HashMap<String, String> = [
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

        let params: HashMap<String, ParamType> = [
            ("search".to_string(), ParamType::String(search.to_string())),
            ("limit".to_string(),  ParamType::OptionalNumber(limit)),
            ("offset".to_string(),  ParamType::OptionalNumber(offset)),
            ("cursor".to_string(), ParamType::String(cursor.to_string())),
            ("cursorDirection".to_string(), ParamType::String(cursor_direction.to_string())),
            ("orderType".to_string(), ParamType::String(order_type.to_string())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::TagList = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Create a new function code tag. Use this endpoint to upload a new version
    /// of your code function. To execute your newly uploaded code, you'll need to
    /// update the function's tag to use your new tag UID.
    /// 
    /// This endpoint accepts a tar.gz file compressed with your code. Make sure to
    /// include any dependencies your code has within the compressed file. You can
    /// learn more about code packaging in the [Appwrite Cloud Functions
    /// tutorial](/docs/functions).
    /// 
    /// Use the "command" param to set the entry point used to execute your code.
    pub fn create_tag(&self, function_id: &str, command: &str, code: std::path::PathBuf) -> Result<models::Tag, AppwriteException> {
        let path = "/functions/functionId/tags".replace("functionId", &function_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "multipart/form-data".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("command".to_string(), ParamType::String(command.to_string())),
            ("code".to_string(), ParamType::FilePath(code)),
        ].iter().cloned().collect();

        let response = self.client.clone().call("POST", &path, Some(headers), Some(params) );

        let processedResponse:models::Tag = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Get a code tag by its unique ID.
    pub fn get_tag(&self, function_id: &str, tag_id: &str) -> Result<models::Tag, AppwriteException> {
        let path = "/functions/functionId/tags/tagId".replace("functionId", &function_id).replace("tagId", &tag_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::Tag = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Delete a code tag by its unique ID.
    pub fn delete_tag(&self, function_id: &str, tag_id: &str) -> Result<serde_json::value::Value, AppwriteException> {
        let path = "/functions/functionId/tags/tagId".replace("functionId", &function_id).replace("tagId", &tag_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("DELETE", &path, Some(headers), Some(params) );

        match response {
            Ok(r) => {
                Ok(serde_json::from_str(&r.text().unwrap()).unwrap())
            }
            Err(e) => {
                Err(e)
            }
        }

    }
}
