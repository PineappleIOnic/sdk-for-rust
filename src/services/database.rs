use crate::client::{Client, ParamType};
use std::collections::HashMap;
use crate::services::AppwriteException;
use crate::models;

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

    /// Get a list of all the user documents. You can use the query params to
    /// filter your results. On admin mode, this endpoint will return a list of all
    /// of the project's documents. [Learn more about different API
    /// modes](/docs/admin).
    pub fn list_documents(&self, collection_id: &str, filters: Option<&[&str]>, limit: Option<i64>, offset: Option<i64>, order_field: Option<&str>, order_type: Option<&str>, order_cast: Option<&str>, search: Option<&str>) -> Result<models::DocumentList, AppwriteException> {
        let path = "/database/collections/collectionId/documents".replace("collectionId", &collection_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let filters:&[&str] = match filters {
            Some(data) => data,
            None => &[]
        };

        let order_field:&str = match order_field {
            Some(data) => data,
            None => ""
        };

        let order_type:&str = match order_type {
            Some(data) => data,
            None => ""
        };

        let order_cast:&str = match order_cast {
            Some(data) => data,
            None => ""
        };

        let search:&str = match search {
            Some(data) => data,
            None => ""
        };

        let params: HashMap<String, ParamType> = [
            ("filters".to_string(), ParamType::Array(filters.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("limit".to_string(),  ParamType::OptionalNumber(limit)),
            ("offset".to_string(),  ParamType::OptionalNumber(offset)),
            ("orderField".to_string(), ParamType::String(order_field.to_string())),
            ("orderType".to_string(), ParamType::String(order_type.to_string())),
            ("orderCast".to_string(), ParamType::String(order_cast.to_string())),
            ("search".to_string(), ParamType::String(search.to_string())),
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
    pub fn create_document(&self, collection_id: &str, data: Option<HashMap<String, crate::client::ParamType>>, read: Option<&[&str]>, write: Option<&[&str]>, parent_document: Option<&str>, parent_property: Option<&str>, parent_property_type: Option<&str>) -> Result<models::Document, AppwriteException> {
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

        let parent_document:&str = match parent_document {
            Some(data) => data,
            None => ""
        };

        let parent_property:&str = match parent_property {
            Some(data) => data,
            None => ""
        };

        let parent_property_type:&str = match parent_property_type {
            Some(data) => data,
            None => ""
        };

        let params: HashMap<String, ParamType> = [
            ("data".to_string(), ParamType::Object(data.unwrap())),
            ("read".to_string(), ParamType::Array(read.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("write".to_string(), ParamType::Array(write.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("parentDocument".to_string(), ParamType::String(parent_document.to_string())),
            ("parentProperty".to_string(), ParamType::String(parent_property.to_string())),
            ("parentPropertyType".to_string(), ParamType::String(parent_property_type.to_string())),
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
    pub fn delete_document(&self, collection_id: &str, document_id: &str) -> Result<bool, AppwriteException> {
        let path = "/database/collections/collectionId/documents/documentId".replace("collectionId", &collection_id).replace("documentId", &document_id);
        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        let response = self.client.clone().call("DELETE", &path, Some(headers), Some(params) );

        Ok(response.unwrap().status().is_success())

    }
}
