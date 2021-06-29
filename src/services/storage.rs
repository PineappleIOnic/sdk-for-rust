use crate::client::{Client, ParamType};
use std::collections::HashMap;
use crate::services::AppwriteException;

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
    pub fn list_files(&self, search: Option<&str>, limit: Option<i64>, offset: Option<i64>, order_type: Option<&str>) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/storage/files";

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

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    /// Create a new file. The user who creates the file will automatically be
    /// assigned to read and write access unless he has passed custom values for
    /// read and write arguments.
    pub fn create_file(&self, file: std::path::PathBuf, read: Option<&[&str]>, write: Option<&[&str]>) -> Result<reqwest::blocking::Response, AppwriteException> {
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
            ("file".to_string(), ParamType::FilePath(file)),
            ("read".to_string(), ParamType::Array(read.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("write".to_string(), ParamType::Array(write.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
        ].iter().cloned().collect();

        return self.client.clone().call("POST", &path, Some(headers), Some(params) );
    }

    /// Get a file by its unique ID. This endpoint response returns a JSON object
    /// with the file metadata.
    pub fn get_file(&self, file_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/storage/files/fileId".replace("fileId", &file_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();


        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    /// Update a file by its unique ID. Only users with write permissions have
    /// access to update this resource.
    pub fn update_file(&self, file_id: &str, read: &[&str], write: &[&str]) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/storage/files/fileId".replace("fileId", &file_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();


        let params: HashMap<String, ParamType> = [
            ("read".to_string(), ParamType::Array(read.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("write".to_string(), ParamType::Array(write.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
        ].iter().cloned().collect();

        return self.client.clone().call("PUT", &path, Some(headers), Some(params) );
    }

    /// Delete a file by its unique ID. Only users with write permissions have
    /// access to delete this resource.
    pub fn delete_file(&self, file_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/storage/files/fileId".replace("fileId", &file_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();


        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("DELETE", &path, Some(headers), Some(params) );
    }

    /// Get a file content by its unique ID. The endpoint response return with a
    /// 'Content-Disposition: attachment' header that tells the browser to start
    /// downloading the file to user downloads directory.
    pub fn get_file_download(&self, file_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/storage/files/fileId/download".replace("fileId", &file_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();


        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    /// Get a file preview image. Currently, this method supports preview for image
    /// files (jpg, png, and gif), other supported formats, like pdf, docs, slides,
    /// and spreadsheets, will return the file icon image. You can also pass query
    /// string arguments for cutting and resizing your preview image.
    pub fn get_file_preview(&self, file_id: &str, width: Option<i64>, height: Option<i64>, quality: Option<i64>, border_width: Option<i64>, border_color: Option<&str>, border_radius: Option<i64>, opacity: Option<f64>, rotation: Option<i64>, background: Option<&str>, output: Option<&str>) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/storage/files/fileId/preview".replace("fileId", &file_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

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
            ("quality".to_string(),  ParamType::OptionalNumber(quality)),
            ("borderWidth".to_string(),  ParamType::OptionalNumber(border_width)),
            ("borderColor".to_string(), ParamType::String(border_color.to_string())),
            ("borderRadius".to_string(),  ParamType::OptionalNumber(border_radius)),
            ("opacity".to_string(),  ParamType::OptionalFloat(opacity)),
            ("rotation".to_string(),  ParamType::OptionalNumber(rotation)),
            ("background".to_string(), ParamType::String(background.to_string())),
            ("output".to_string(), ParamType::String(output.to_string())),
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    /// Get a file content by its unique ID. This endpoint is similar to the
    /// download method but returns with no  'Content-Disposition: attachment'
    /// header.
    pub fn get_file_view(&self, file_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/storage/files/fileId/view".replace("fileId", &file_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();


        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }
}
