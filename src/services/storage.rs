use crate::client::{Client, ParamType};
use std::collections::HashMap;
use crate::services::AppwriteException;
use crate::models;
use serde_json::json;

#[derive(Clone)]
pub struct Storage {
  client: Client
}

impl Storage {  
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone()
        }
    }

    /// Get a list of all the user files. You can use the query params to filter
    /// your results. On admin mode, this endpoint will return a list of all of the
    /// project's files. [Learn more about different API modes](/docs/admin).
    pub fn list_files(&self, search: Option<&str>, limit: Option<i64>, offset: Option<i64>, cursor: Option<&str>, cursor_direction: Option<&str>, order_type: Option<&str>) -> Result<models::FileList, AppwriteException> {
        let path = "/storage/files";
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

        let processedResponse:models::FileList = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Create a new file. The user who creates the file will automatically be
    /// assigned to read and write access unless he has passed custom values for
    /// read and write arguments.
    pub fn create_file(&self, file_id: &str, file: std::path::PathBuf, read: Option<&[&str]>, write: Option<&[&str]>) -> Result<models::File, AppwriteException> {
        let path = "/storage/files";
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "multipart/form-data".to_string()),
        ].iter().cloned().collect();

        let read:&[&str] = match read {
            Some(data) => data,
            None => &[]
        };

        let write:&[&str] = match write {
            Some(data) => data,
            None => &[]
        };

        let params: HashMap<String, ParamType> = [
            ("fileId".to_string(), ParamType::String(file_id.to_string())),
            ("file".to_string(), ParamType::FilePath(file)),
            ("read".to_string(), ParamType::Array(read.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("write".to_string(), ParamType::Array(write.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("POST", &path, Some(headers), Some(params) );

        let processedResponse:models::File = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Get a file by its unique ID. This endpoint response returns a JSON object
    /// with the file metadata.
    pub fn get_file(&self, file_id: &str) -> Result<models::File, AppwriteException> {
        let path = "/storage/files/fileId".replace("fileId", &file_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::File = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Update a file by its unique ID. Only users with write permissions have
    /// access to update this resource.
    pub fn update_file(&self, file_id: &str, read: &[&str], write: &[&str]) -> Result<models::File, AppwriteException> {
        let path = "/storage/files/fileId".replace("fileId", &file_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("read".to_string(), ParamType::Array(read.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("write".to_string(), ParamType::Array(write.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("PUT", &path, Some(headers), Some(params) );

        let processedResponse:models::File = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Delete a file by its unique ID. Only users with write permissions have
    /// access to delete this resource.
    pub fn delete_file(&self, file_id: &str) -> Result<serde_json::value::Value, AppwriteException> {
        let path = "/storage/files/fileId".replace("fileId", &file_id);
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

    /// Get a file content by its unique ID. The endpoint response return with a
    /// 'Content-Disposition: attachment' header that tells the browser to start
    /// downloading the file to user downloads directory.
    pub fn get_file_download(&self, file_id: &str) -> Result<Vec<u8>, AppwriteException> {
        let path = "/storage/files/fileId/download".replace("fileId", &file_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
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

    /// Get a file preview image. Currently, this method supports preview for image
    /// files (jpg, png, and gif), other supported formats, like pdf, docs, slides,
    /// and spreadsheets, will return the file icon image. You can also pass query
    /// string arguments for cutting and resizing your preview image.
    pub fn get_file_preview(&self, file_id: &str, width: Option<i64>, height: Option<i64>, gravity: Option<&str>, quality: Option<i64>, border_width: Option<i64>, border_color: Option<&str>, border_radius: Option<i64>, opacity: Option<f64>, rotation: Option<i64>, background: Option<&str>, output: Option<&str>) -> Result<Vec<u8>, AppwriteException> {
        let path = "/storage/files/fileId/preview".replace("fileId", &file_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let gravity:&str = match gravity {
            Some(data) => data,
            None => ""
        };

        let border_color:&str = match border_color {
            Some(data) => data,
            None => ""
        };

        let background:&str = match background {
            Some(data) => data,
            None => ""
        };

        let output:&str = match output {
            Some(data) => data,
            None => ""
        };

        let params: HashMap<String, ParamType> = [
            ("width".to_string(),  ParamType::OptionalNumber(width)),
            ("height".to_string(),  ParamType::OptionalNumber(height)),
            ("gravity".to_string(), ParamType::String(gravity.to_string())),
            ("quality".to_string(),  ParamType::OptionalNumber(quality)),
            ("borderWidth".to_string(),  ParamType::OptionalNumber(border_width)),
            ("borderColor".to_string(), ParamType::String(border_color.to_string())),
            ("borderRadius".to_string(),  ParamType::OptionalNumber(border_radius)),
            ("opacity".to_string(),  ParamType::OptionalFloat(opacity)),
            ("rotation".to_string(),  ParamType::OptionalNumber(rotation)),
            ("background".to_string(), ParamType::String(background.to_string())),
            ("output".to_string(), ParamType::String(output.to_string())),
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

    /// Get a file content by its unique ID. This endpoint is similar to the
    /// download method but returns with no  'Content-Disposition: attachment'
    /// header.
    pub fn get_file_view(&self, file_id: &str) -> Result<Vec<u8>, AppwriteException> {
        let path = "/storage/files/fileId/view".replace("fileId", &file_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
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
