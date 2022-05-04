use crate::client::{Client, ParamType};
use std::collections::HashMap;
use crate::services::AppwriteException;
use crate::models;
use serde_json::json;

#[derive(Clone)]
pub struct Database {
  client: Client
}

impl Database {  
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone()
        }
    }

    /// Get a list of all the user collections. You can use the query params to
    /// filter your results. On admin mode, this endpoint will return a list of all
    /// of the project's collections. [Learn more about different API
    /// modes](/docs/admin).
    pub fn list_collections(&self, search: Option<&str>, limit: Option<i64>, offset: Option<i64>, cursor: Option<&str>, cursor_direction: Option<&str>, order_type: Option<&str>) -> Result<models::CollectionList, AppwriteException> {
        let path = "/database/collections";
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

        let processedResponse:models::CollectionList = match response {
            Ok(r) => {
                println!("{}", r.text().unwrap());
                panic!("lol. lmao.");
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Create a new Collection.
    pub fn create_collection(&self, collection_id: &str, name: &str, permission: &str, read: &[&str], write: &[&str]) -> Result<models::Collection, AppwriteException> {
        let path = "/database/collections";
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("collectionId".to_string(), ParamType::String(collection_id.to_string())),
            ("name".to_string(), ParamType::String(name.to_string())),
            ("permission".to_string(), ParamType::String(permission.to_string())),
            ("read".to_string(), ParamType::Array(read.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("write".to_string(), ParamType::Array(write.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("POST", &path, Some(headers), Some(params) );

        let processedResponse:models::Collection = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Get a collection by its unique ID. This endpoint response returns a JSON
    /// object with the collection metadata.
    pub fn get_collection(&self, collection_id: &str) -> Result<models::Collection, AppwriteException> {
        let path = "/database/collections/collectionId".replace("collectionId", &collection_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::Collection = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Update a collection by its unique ID.
    pub fn update_collection(&self, collection_id: &str, name: &str, permission: &str, read: Option<&[&str]>, write: Option<&[&str]>, enabled: Option<bool>) -> Result<models::Collection, AppwriteException> {
        let path = "/database/collections/collectionId".replace("collectionId", &collection_id);
        let headers: HashMap<String, String> = [
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

        let params: HashMap<String, ParamType> = [
            ("name".to_string(), ParamType::String(name.to_string())),
            ("permission".to_string(), ParamType::String(permission.to_string())),
            ("read".to_string(), ParamType::Array(read.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("write".to_string(), ParamType::Array(write.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("enabled".to_string(), ParamType::OptionalBool(enabled)),
        ].iter().cloned().collect();

        let response = self.client.clone().call("PUT", &path, Some(headers), Some(params) );

        let processedResponse:models::Collection = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Delete a collection by its unique ID. Only users with write permissions
    /// have access to delete this resource.
    pub fn delete_collection(&self, collection_id: &str) -> Result<serde_json::value::Value, AppwriteException> {
        let path = "/database/collections/collectionId".replace("collectionId", &collection_id);
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

    pub fn list_attributes(&self, collection_id: &str) -> Result<models::AttributeList, AppwriteException> {
        let path = "/database/collections/collectionId/attributes".replace("collectionId", &collection_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::AttributeList = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Create a boolean attribute.
    /// 
    pub fn create_boolean_attribute(&self, collection_id: &str, key: &str, required: bool, default: Option<bool>, array: Option<bool>) -> Result<models::AttributeBoolean, AppwriteException> {
        let path = "/database/collections/collectionId/attributes/boolean".replace("collectionId", &collection_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("key".to_string(), ParamType::String(key.to_string())),
            ("required".to_string(), ParamType::Bool(required)),
            ("default".to_string(), ParamType::OptionalBool(default)),
            ("array".to_string(), ParamType::OptionalBool(array)),
        ].iter().cloned().collect();

        let response = self.client.clone().call("POST", &path, Some(headers), Some(params) );

        let processedResponse:models::AttributeBoolean = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Create an email attribute.
    /// 
    pub fn create_email_attribute(&self, collection_id: &str, key: &str, required: bool, default: Option<&str>, array: Option<bool>) -> Result<models::AttributeEmail, AppwriteException> {
        let path = "/database/collections/collectionId/attributes/email".replace("collectionId", &collection_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let default:&str = match default {
            Some(data) => data,
            None => ""
        };

        let params: HashMap<String, ParamType> = [
            ("key".to_string(), ParamType::String(key.to_string())),
            ("required".to_string(), ParamType::Bool(required)),
            ("default".to_string(), ParamType::String(default.to_string())),
            ("array".to_string(), ParamType::OptionalBool(array)),
        ].iter().cloned().collect();

        let response = self.client.clone().call("POST", &path, Some(headers), Some(params) );

        let processedResponse:models::AttributeEmail = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    pub fn create_enum_attribute(&self, collection_id: &str, key: &str, elements: &[&str], required: bool, default: Option<&str>, array: Option<bool>) -> Result<models::AttributeEnum, AppwriteException> {
        let path = "/database/collections/collectionId/attributes/enum".replace("collectionId", &collection_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let default:&str = match default {
            Some(data) => data,
            None => ""
        };

        let params: HashMap<String, ParamType> = [
            ("key".to_string(), ParamType::String(key.to_string())),
            ("elements".to_string(), ParamType::Array(elements.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("required".to_string(), ParamType::Bool(required)),
            ("default".to_string(), ParamType::String(default.to_string())),
            ("array".to_string(), ParamType::OptionalBool(array)),
        ].iter().cloned().collect();

        let response = self.client.clone().call("POST", &path, Some(headers), Some(params) );

        let processedResponse:models::AttributeEnum = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Create a float attribute. Optionally, minimum and maximum values can be
    /// provided.
    /// 
    pub fn create_float_attribute(&self, collection_id: &str, key: &str, required: bool, min: Option<&str>, max: Option<&str>, default: Option<&str>, array: Option<bool>) -> Result<models::AttributeFloat, AppwriteException> {
        let path = "/database/collections/collectionId/attributes/float".replace("collectionId", &collection_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let min:&str = match min {
            Some(data) => data,
            None => ""
        };

        let max:&str = match max {
            Some(data) => data,
            None => ""
        };

        let default:&str = match default {
            Some(data) => data,
            None => ""
        };

        let params: HashMap<String, ParamType> = [
            ("key".to_string(), ParamType::String(key.to_string())),
            ("required".to_string(), ParamType::Bool(required)),
            ("min".to_string(), ParamType::String(min.to_string())),
            ("max".to_string(), ParamType::String(max.to_string())),
            ("default".to_string(), ParamType::String(default.to_string())),
            ("array".to_string(), ParamType::OptionalBool(array)),
        ].iter().cloned().collect();

        let response = self.client.clone().call("POST", &path, Some(headers), Some(params) );

        let processedResponse:models::AttributeFloat = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Create an integer attribute. Optionally, minimum and maximum values can be
    /// provided.
    /// 
    pub fn create_integer_attribute(&self, collection_id: &str, key: &str, required: bool, min: Option<i64>, max: Option<i64>, default: Option<i64>, array: Option<bool>) -> Result<models::AttributeInteger, AppwriteException> {
        let path = "/database/collections/collectionId/attributes/integer".replace("collectionId", &collection_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
            ("key".to_string(), ParamType::String(key.to_string())),
            ("required".to_string(), ParamType::Bool(required)),
            ("min".to_string(),  ParamType::OptionalNumber(min)),
            ("max".to_string(),  ParamType::OptionalNumber(max)),
            ("default".to_string(),  ParamType::OptionalNumber(default)),
            ("array".to_string(), ParamType::OptionalBool(array)),
        ].iter().cloned().collect();

        let response = self.client.clone().call("POST", &path, Some(headers), Some(params) );

        let processedResponse:models::AttributeInteger = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Create IP address attribute.
    /// 
    pub fn create_ip_attribute(&self, collection_id: &str, key: &str, required: bool, default: Option<&str>, array: Option<bool>) -> Result<models::AttributeIp, AppwriteException> {
        let path = "/database/collections/collectionId/attributes/ip".replace("collectionId", &collection_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let default:&str = match default {
            Some(data) => data,
            None => ""
        };

        let params: HashMap<String, ParamType> = [
            ("key".to_string(), ParamType::String(key.to_string())),
            ("required".to_string(), ParamType::Bool(required)),
            ("default".to_string(), ParamType::String(default.to_string())),
            ("array".to_string(), ParamType::OptionalBool(array)),
        ].iter().cloned().collect();

        let response = self.client.clone().call("POST", &path, Some(headers), Some(params) );

        let processedResponse:models::AttributeIp = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Create a string attribute.
    /// 
    pub fn create_string_attribute(&self, collection_id: &str, key: &str, size: i64, required: bool, default: Option<&str>, array: Option<bool>) -> Result<models::AttributeString, AppwriteException> {
        let path = "/database/collections/collectionId/attributes/string".replace("collectionId", &collection_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let default:&str = match default {
            Some(data) => data,
            None => ""
        };

        let params: HashMap<String, ParamType> = [
            ("key".to_string(), ParamType::String(key.to_string())),
            ("size".to_string(),  ParamType::Number(size)),
            ("required".to_string(), ParamType::Bool(required)),
            ("default".to_string(), ParamType::String(default.to_string())),
            ("array".to_string(), ParamType::OptionalBool(array)),
        ].iter().cloned().collect();

        let response = self.client.clone().call("POST", &path, Some(headers), Some(params) );

        let processedResponse:models::AttributeString = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Create a URL attribute.
    /// 
    pub fn create_url_attribute(&self, collection_id: &str, key: &str, required: bool, default: Option<&str>, array: Option<bool>) -> Result<models::AttributeUrl, AppwriteException> {
        let path = "/database/collections/collectionId/attributes/url".replace("collectionId", &collection_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let default:&str = match default {
            Some(data) => data,
            None => ""
        };

        let params: HashMap<String, ParamType> = [
            ("key".to_string(), ParamType::String(key.to_string())),
            ("required".to_string(), ParamType::Bool(required)),
            ("default".to_string(), ParamType::String(default.to_string())),
            ("array".to_string(), ParamType::OptionalBool(array)),
        ].iter().cloned().collect();

        let response = self.client.clone().call("POST", &path, Some(headers), Some(params) );

        let processedResponse:models::AttributeUrl = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    pub fn get_attribute(&self, collection_id: &str, key: &str) -> Result<serde_json::value::Value, AppwriteException> {
        let path = "/database/collections/collectionId/attributes/key".replace("collectionId", &collection_id).replace("key", &key);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        match response {
            Ok(r) => {
                Ok(serde_json::from_str(&r.text().unwrap()).unwrap())
            }
            Err(e) => {
                Err(e)
            }
        }

    }

    pub fn delete_attribute(&self, collection_id: &str, key: &str) -> Result<serde_json::value::Value, AppwriteException> {
        let path = "/database/collections/collectionId/attributes/key".replace("collectionId", &collection_id).replace("key", &key);
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

    /// Get a list of all the user documents. You can use the query params to
    /// filter your results. On admin mode, this endpoint will return a list of all
    /// of the project's documents. [Learn more about different API
    /// modes](/docs/admin).
    pub fn list_documents(&self, collection_id: &str, queries: Option<&[&str]>, limit: Option<i64>, offset: Option<i64>, cursor: Option<&str>, cursor_direction: Option<&str>, order_attributes: Option<&[&str]>, order_types: Option<&[&str]>) -> Result<models::DocumentList, AppwriteException> {
        let path = "/database/collections/collectionId/documents".replace("collectionId", &collection_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let queries:&[&str] = match queries {
            Some(data) => data,
            None => &[]
        };

        let cursor:&str = match cursor {
            Some(data) => data,
            None => ""
        };

        let cursor_direction:&str = match cursor_direction {
            Some(data) => data,
            None => ""
        };

        let order_attributes:&[&str] = match order_attributes {
            Some(data) => data,
            None => &[]
        };

        let order_types:&[&str] = match order_types {
            Some(data) => data,
            None => &[]
        };

        let params: HashMap<String, ParamType> = [
            ("queries".to_string(), ParamType::Array(queries.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("limit".to_string(),  ParamType::OptionalNumber(limit)),
            ("offset".to_string(),  ParamType::OptionalNumber(offset)),
            ("cursor".to_string(), ParamType::String(cursor.to_string())),
            ("cursorDirection".to_string(), ParamType::String(cursor_direction.to_string())),
            ("orderAttributes".to_string(), ParamType::Array(order_attributes.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("orderTypes".to_string(), ParamType::Array(order_types.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::DocumentList = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Create a new Document. Before using this route, you should create a new
    /// collection resource using either a [server
    /// integration](/docs/server/database#databaseCreateCollection) API or
    /// directly from your database console.
    pub fn create_document(&self, collection_id: &str, document_id: &str, data: Option<HashMap<String, crate::client::ParamType>>, read: Option<&[&str]>, write: Option<&[&str]>) -> Result<models::Document, AppwriteException> {
        let path = "/database/collections/collectionId/documents".replace("collectionId", &collection_id);
        let headers: HashMap<String, String> = [
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

        let params: HashMap<String, ParamType> = [
            ("documentId".to_string(), ParamType::String(document_id.to_string())),
            ("data".to_string(), ParamType::Object(data.unwrap())),
            ("read".to_string(), ParamType::Array(read.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("write".to_string(), ParamType::Array(write.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("POST", &path, Some(headers), Some(params) );

        let processedResponse:models::Document = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Get a document by its unique ID. This endpoint response returns a JSON
    /// object with the document data.
    pub fn get_document(&self, collection_id: &str, document_id: &str) -> Result<models::Document, AppwriteException> {
        let path = "/database/collections/collectionId/documents/documentId".replace("collectionId", &collection_id).replace("documentId", &document_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::Document = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Update a document by its unique ID. Using the patch method you can pass
    /// only specific fields that will get updated.
    pub fn update_document(&self, collection_id: &str, document_id: &str, data: Option<HashMap<String, crate::client::ParamType>>, read: Option<&[&str]>, write: Option<&[&str]>) -> Result<models::Document, AppwriteException> {
        let path = "/database/collections/collectionId/documents/documentId".replace("collectionId", &collection_id).replace("documentId", &document_id);
        let headers: HashMap<String, String> = [
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

        let params: HashMap<String, ParamType> = [
            ("data".to_string(), ParamType::Object(data.unwrap())),
            ("read".to_string(), ParamType::Array(read.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("write".to_string(), ParamType::Array(write.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("PATCH", &path, Some(headers), Some(params) );

        let processedResponse:models::Document = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    /// Delete a document by its unique ID. This endpoint deletes only the parent
    /// documents, its attributes and relations to other documents. Child documents
    /// **will not** be deleted.
    pub fn delete_document(&self, collection_id: &str, document_id: &str) -> Result<serde_json::value::Value, AppwriteException> {
        let path = "/database/collections/collectionId/documents/documentId".replace("collectionId", &collection_id).replace("documentId", &document_id);
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

    pub fn list_indexes(&self, collection_id: &str) -> Result<models::IndexList, AppwriteException> {
        let path = "/database/collections/collectionId/indexes".replace("collectionId", &collection_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::IndexList = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    pub fn create_index(&self, collection_id: &str, key: &str, xtype: &str, attributes: &[&str], orders: Option<&[&str]>) -> Result<models::Index, AppwriteException> {
        let path = "/database/collections/collectionId/indexes".replace("collectionId", &collection_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let orders:&[&str] = match orders {
            Some(data) => data,
            None => &[]
        };

        let params: HashMap<String, ParamType> = [
            ("key".to_string(), ParamType::String(key.to_string())),
            ("type".to_string(), ParamType::String(xtype.to_string())),
            ("attributes".to_string(), ParamType::Array(attributes.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("orders".to_string(), ParamType::Array(orders.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
        ].iter().cloned().collect();

        let response = self.client.clone().call("POST", &path, Some(headers), Some(params) );

        let processedResponse:models::Index = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    pub fn get_index(&self, collection_id: &str, key: &str) -> Result<models::Index, AppwriteException> {
        let path = "/database/collections/collectionId/indexes/key".replace("collectionId", &collection_id).replace("key", &key);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("GET", &path, Some(headers), Some(params) );

        let processedResponse:models::Index = match response {
            Ok(r) => {
                r.json().unwrap()
            }
            Err(e) => {
                return Err(e);
            }
        };

        Ok(processedResponse)

    }

    pub fn delete_index(&self, collection_id: &str, key: &str) -> Result<serde_json::value::Value, AppwriteException> {
        let path = "/database/collections/collectionId/indexes/key".replace("collectionId", &collection_id).replace("key", &key);
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
