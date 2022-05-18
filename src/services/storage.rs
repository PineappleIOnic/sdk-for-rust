use crate::client::{Client, ParamType};
use std::collections::HashMap;
use crate::services::AppwriteException;
use crate::models;
use serde_json::json;
use std::io::Read;

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

    /// Get a list of all the storage buckets. You can use the query params to
    /// filter your results.
    pub fn list_buckets(&self, search: Option<&str>, limit: Option<i64>, offset: Option<i64>, cursor: Option<&str>, cursor_direction: Option<&str>, order_type: Option<&str>) -> Result<models::BucketList, AppwriteException> {
        let path = "/storage/buckets";
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

        let processedResponse:models::BucketList = match response {
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

    /// Create a new storage bucket.
    pub fn create_bucket(&self, bucket_id: &str, name: &str, permission: &str, read: Option<&[&str]>, write: Option<&[&str]>, enabled: Option<bool>, maximum_file_size: Option<i64>, allowed_file_extensions: Option<&[&str]>, encryption: Option<bool>, antivirus: Option<bool>) -> Result<models::Bucket, AppwriteException> {
        let path = "/storage/buckets";
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let read:&[&str] = match read {
            Some(data) => data,
            None => &[]
        };

        let write:&[&str] = match write {
            Some(data) => data,
            None => &[]
        };

        let allowed_file_extensions:&[&str] = match allowed_file_extensions {
            Some(data) => data,
            None => &[]
        };

        let  params: HashMap<String, ParamType> = [
            ("bucketId".to_string(), ParamType::String(bucket_id.to_string())),
            ("name".to_string(), ParamType::String(name.to_string())),
            ("permission".to_string(), ParamType::String(permission.to_string())),
            ("read".to_string(), ParamType::Array(read.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("write".to_string(), ParamType::Array(write.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("enabled".to_string(), ParamType::OptionalBool(enabled)),
            ("maximumFileSize".to_string(),  ParamType::OptionalNumber(maximum_file_size)),
            ("allowedFileExtensions".to_string(), ParamType::Array(allowed_file_extensions.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("encryption".to_string(), ParamType::OptionalBool(encryption)),
            ("antivirus".to_string(), ParamType::OptionalBool(antivirus)),
        ].iter().cloned().collect();

        let response = self.client.clone().call("POST", &path, Some(headers), Some(params) );

        let processedResponse:models::Bucket = match response {
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

    /// Get a storage bucket by its unique ID. This endpoint response returns a
    /// JSON object with the storage bucket metadata.
    pub fn get_bucket(&self, bucket_id: &str) -> Result<models::Bucket, AppwriteException> {
        let path = "/storage/buckets/bucketId".replace("bucketId", &bucket_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::Bucket = match response {
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

    /// Update a storage bucket by its unique ID.
    pub fn update_bucket(&self, bucket_id: &str, name: &str, permission: &str, read: Option<&[&str]>, write: Option<&[&str]>, enabled: Option<bool>, maximum_file_size: Option<i64>, allowed_file_extensions: Option<&[&str]>, encryption: Option<bool>, antivirus: Option<bool>) -> Result<models::Bucket, AppwriteException> {
        let path = "/storage/buckets/bucketId".replace("bucketId", &bucket_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let read:&[&str] = match read {
            Some(data) => data,
            None => &[]
        };

        let write:&[&str] = match write {
            Some(data) => data,
            None => &[]
        };

        let allowed_file_extensions:&[&str] = match allowed_file_extensions {
            Some(data) => data,
            None => &[]
        };

        let  params: HashMap<String, ParamType> = [
            ("name".to_string(), ParamType::String(name.to_string())),
            ("permission".to_string(), ParamType::String(permission.to_string())),
            ("read".to_string(), ParamType::Array(read.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("write".to_string(), ParamType::Array(write.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("enabled".to_string(), ParamType::OptionalBool(enabled)),
            ("maximumFileSize".to_string(),  ParamType::OptionalNumber(maximum_file_size)),
            ("allowedFileExtensions".to_string(), ParamType::Array(allowed_file_extensions.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("encryption".to_string(), ParamType::OptionalBool(encryption)),
            ("antivirus".to_string(), ParamType::OptionalBool(antivirus)),
        ].iter().cloned().collect();

        let response = self.client.clone().call("PUT", &path, Some(headers), Some(params) );

        let processedResponse:models::Bucket = match response {
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

    /// Delete a storage bucket by its unique ID.
    pub fn delete_bucket(&self, bucket_id: &str) -> Result<serde_json::value::Value, AppwriteException> {
        let path = "/storage/buckets/bucketId".replace("bucketId", &bucket_id);
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

    /// Get a list of all the user files. You can use the query params to filter
    /// your results. On admin mode, this endpoint will return a list of all of the
    /// project's files. [Learn more about different API modes](/docs/admin).
    pub fn list_files(&self, bucket_id: &str, search: Option<&str>, limit: Option<i64>, offset: Option<i64>, cursor: Option<&str>, cursor_direction: Option<&str>, order_type: Option<&str>) -> Result<models::FileList, AppwriteException> {
        let path = "/storage/buckets/bucketId/files".replace("bucketId", &bucket_id);
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

        let processedResponse:models::FileList = match response {
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

    /// Create a new file. Before using this route, you should create a new bucket
    /// resource using either a [server
    /// integration](/docs/server/database#storageCreateBucket) API or directly
    /// from your Appwrite console.
    /// 
    /// Larger files should be uploaded using multiple requests with the
    /// [content-range](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Range)
    /// header to send a partial request with a maximum supported chunk of `5MB`.
    /// The `content-range` header values should always be in bytes.
    /// 
    /// When the first request is sent, the server will return the **File** object,
    /// and the subsequent part request must include the file's **id** in
    /// `x-appwrite-id` header to allow the server to know that the partial upload
    /// is for the existing file and not for a new one.
    /// 
    /// If you're creating a new file using one of the Appwrite SDKs, all the
    /// chunking logic will be managed by the SDK internally.
    /// 
    pub fn create_file(&self, bucket_id: &str, file_id: &str, file: std::path::PathBuf, read: Option<&[&str]>, write: Option<&[&str]>) -> Result<models::File, AppwriteException> {
        let path = "/storage/buckets/bucketId/files".replace("bucketId", &bucket_id);
        let mut headers: HashMap<String, String> = [
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

        let mut params: HashMap<String, ParamType> = [
            ("fileId".to_string(), ParamType::String(file_id.to_string())),
            ("read".to_string(), ParamType::Array(read.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("write".to_string(), ParamType::Array(write.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
        ].iter().cloned().collect();

        let mut fileBuf = std::fs::File::open(file.clone()).unwrap();

        let size = fileBuf.metadata().unwrap().len();

        match size {
            size if size <= crate::client::CHUNK_SIZE => {
                params.insert("file".to_string(), ParamType::FilePath(file));
                match self.client.clone().call("POST", &path, Some(headers), Some(params)) {
                    Ok(r) => {
                        Ok(r.json::<models::File>().unwrap())
                    }
                    Err(e) => {
                        Err(e)
                    }
                }
            }
            _ => {
                // Stream Data.
                let mut id = "".to_string();

                let mut resumeCounter: u64 = 0;
                let totalCounters = (((size / crate::client::CHUNK_SIZE) as f64).ceil() as u64) + 1;

                if file_id != "unique()" {
                    let filePath = format!("/storage/buckets/bucketId/files{}", file_id);
                    match self.client.clone().call("GET", &filePath, Some(headers.clone()), None) {
                        Ok(r) => {
                            match r.json::<serde_json::Value>() {
                                Ok(data) => {
                                    resumeCounter = data["chunksUploaded"].as_u64().unwrap();
                                },
                                Err(_e) => ()
                            };
                        }
                        Err(_e) => ()
                    };
                }

                let response: reqwest::blocking::Response;

                for counter in resumeCounter..totalCounters {
                    let mut headers: HashMap<String, String> = [
                        ("content-type".to_string(), "multipart/form-data".to_string()),
                    ].iter().cloned().collect();

                    let mut params = params.clone();

                    headers.insert("content-range".to_string(), format!("bytes {}-{}/{}", (counter * crate::client::CHUNK_SIZE),
                        std::cmp::min((counter * crate::client::CHUNK_SIZE) + crate::client::CHUNK_SIZE - 1, size), size));

                    if id.len() != 0 {
                        headers.insert("x-appwrite-id".to_string(), id.to_string());
                    }

                    let mut chunk = Vec::with_capacity(crate::client::CHUNK_SIZE as usize);

                    match fileBuf.by_ref().take(crate::client::CHUNK_SIZE).read_to_end(&mut chunk) {
                        Ok(_) => (),
                        Err(e) => {
                            return Err(AppwriteException::new(format!("A error occoured. ERR: {}, This could either be a connection error or an internal Appwrite error. Please check your Appwrite instance logs. ", e), 0, "".to_string()))
                        }
                    };

                    params.insert("file".to_string(), ParamType::StreamData(chunk, file.file_name().unwrap().to_string_lossy().to_string()));

                    let response = match self.client.clone().call("POST", &path, Some(headers), Some(params)) {
                        Ok(r) => r,
                        Err(e) => {
                            return Err(e);
                        }
                    };

                    // If last chunk, return the response.
                    if counter == totalCounters - 1 {
                        return Ok(response.json::<models::File>().unwrap());
                    } else {
                        if id.len() == 0 {
                            id = response.json::<serde_json::Value>().unwrap()["$id"].as_str().unwrap().to_owned();
                        }
                    }
                };

                return Err(AppwriteException::new("Error uploading chunk data.".to_string(), 500, "0".to_string()));
            }
        }
    }

    /// Get a file by its unique ID. This endpoint response returns a JSON object
    /// with the file metadata.
    pub fn get_file(&self, bucket_id: &str, file_id: &str) -> Result<models::File, AppwriteException> {
        let path = "/storage/buckets/bucketId/files/fileId".replace("bucketId", &bucket_id).replace("fileId", &file_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::File = match response {
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

    /// Update a file by its unique ID. Only users with write permissions have
    /// access to update this resource.
    pub fn update_file(&self, bucket_id: &str, file_id: &str, read: Option<&[&str]>, write: Option<&[&str]>) -> Result<models::File, AppwriteException> {
        let path = "/storage/buckets/bucketId/files/fileId".replace("bucketId", &bucket_id).replace("fileId", &file_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let read:&[&str] = match read {
            Some(data) => data,
            None => &[]
        };

        let write:&[&str] = match write {
            Some(data) => data,
            None => &[]
        };

        let  params: HashMap<String, ParamType> = [
            ("read".to_string(), ParamType::Array(read.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("write".to_string(), ParamType::Array(write.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("PUT", &path, Some(headers), Some(params) );

        let processedResponse:models::File = match response {
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

    /// Delete a file by its unique ID. Only users with write permissions have
    /// access to delete this resource.
    pub fn delete_file(&self, bucket_id: &str, file_id: &str) -> Result<serde_json::value::Value, AppwriteException> {
        let path = "/storage/buckets/bucketId/files/fileId".replace("bucketId", &bucket_id).replace("fileId", &file_id);
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

    /// Get a file content by its unique ID. The endpoint response return with a
    /// 'Content-Disposition: attachment' header that tells the browser to start
    /// downloading the file to user downloads directory.
    pub fn get_file_download(&self, bucket_id: &str, file_id: &str) -> Result<Vec<u8>, AppwriteException> {
        let path = "/storage/buckets/bucketId/files/fileId/download".replace("bucketId", &bucket_id).replace("fileId", &file_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
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
    /// string arguments for cutting and resizing your preview image. Preview is
    /// supported only for image files smaller than 10MB.
    pub fn get_file_preview(&self, bucket_id: &str, file_id: &str, width: Option<i64>, height: Option<i64>, gravity: Option<&str>, quality: Option<i64>, border_width: Option<i64>, border_color: Option<&str>, border_radius: Option<i64>, opacity: Option<f64>, rotation: Option<i64>, background: Option<&str>, output: Option<&str>) -> Result<Vec<u8>, AppwriteException> {
        let path = "/storage/buckets/bucketId/files/fileId/preview".replace("bucketId", &bucket_id).replace("fileId", &file_id);
        let  headers: HashMap<String, String> = [
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

        let  params: HashMap<String, ParamType> = [
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
    pub fn get_file_view(&self, bucket_id: &str, file_id: &str) -> Result<Vec<u8>, AppwriteException> {
        let path = "/storage/buckets/bucketId/files/fileId/view".replace("bucketId", &bucket_id).replace("fileId", &file_id);
        let  headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let  params: HashMap<String, ParamType> = [
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

    pub fn get_usage(&self, range: Option<&str>) -> Result<models::UsageStorage, AppwriteException> {
        let path = "/storage/usage";
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

        let processedResponse:models::UsageStorage = match response {
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

    pub fn get_bucket_usage(&self, bucket_id: &str, range: Option<&str>) -> Result<models::UsageBuckets, AppwriteException> {
        let path = "/storage/bucketId/usage".replace("bucketId", &bucket_id);
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

        let processedResponse:models::UsageBuckets = match response {
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
