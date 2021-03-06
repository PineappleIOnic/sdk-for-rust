use reqwest::header::HeaderMap;
use std::{collections::HashMap, str::FromStr};
use std::path::PathBuf;
use crate::services::AppwriteException;

#[derive(Clone)]
pub struct Client {
    endpoint: url::Url,
    headers: HeaderMap,
    client: reqwest::blocking::Client,
}

pub static CHUNK_SIZE: u64 = 5*1024*1024; // 5MB

#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum ParamType {
    Bool(bool),
    Number(i64),
    String(String),
    Array(Vec<ParamType>),
    FilePath(PathBuf),
    Object(HashMap<String, ParamType>),
    Float(f64),
    StreamData(Vec<u8>, String),
    OptionalBool(Option<bool>),
    OptionalNumber(Option<i64>),
    OptionalArray(Option<Vec<ParamType>>),
    OptionalFilePath(Option<PathBuf>),
    OptionalObject(Option<HashMap<String, ParamType>>),
    OptionalFloat(Option<f64>),
    DocumentList(crate::models::DocumentList),
    CollectionList(crate::models::CollectionList),
    IndexList(crate::models::IndexList),
    UserList(crate::models::UserList),
    SessionList(crate::models::SessionList),
    LogList(crate::models::LogList),
    FileList(crate::models::FileList),
    BucketList(crate::models::BucketList),
    TeamList(crate::models::TeamList),
    MembershipList(crate::models::MembershipList),
    FunctionList(crate::models::FunctionList),
    RuntimeList(crate::models::RuntimeList),
    DeploymentList(crate::models::DeploymentList),
    ExecutionList(crate::models::ExecutionList),
    ProjectList(crate::models::ProjectList),
    WebhookList(crate::models::WebhookList),
    KeyList(crate::models::KeyList),
    PlatformList(crate::models::PlatformList),
    DomainList(crate::models::DomainList),
    CountryList(crate::models::CountryList),
    ContinentList(crate::models::ContinentList),
    LanguageList(crate::models::LanguageList),
    CurrencyList(crate::models::CurrencyList),
    PhoneList(crate::models::PhoneList),
    MetricList(crate::models::MetricList),
    Collection(crate::models::Collection),
    AttributeList(crate::models::AttributeList),
    AttributeString(crate::models::AttributeString),
    AttributeInteger(crate::models::AttributeInteger),
    AttributeFloat(crate::models::AttributeFloat),
    AttributeBoolean(crate::models::AttributeBoolean),
    AttributeEmail(crate::models::AttributeEmail),
    AttributeEnum(crate::models::AttributeEnum),
    AttributeIp(crate::models::AttributeIp),
    AttributeUrl(crate::models::AttributeUrl),
    Index(crate::models::Index),
    Document(crate::models::Document),
    Log(crate::models::Log),
    User(crate::models::User),
    Preferences(crate::models::Preferences),
    Session(crate::models::Session),
    Token(crate::models::Token),
    Jwt(crate::models::Jwt),
    Locale(crate::models::Locale),
    File(crate::models::File),
    Bucket(crate::models::Bucket),
    Team(crate::models::Team),
    Membership(crate::models::Membership),
    Function(crate::models::Function),
    Runtime(crate::models::Runtime),
    Deployment(crate::models::Deployment),
    Execution(crate::models::Execution),
    Project(crate::models::Project),
    Webhook(crate::models::Webhook),
    Key(crate::models::Key),
    Domain(crate::models::Domain),
    Platform(crate::models::Platform),
    Country(crate::models::Country),
    Continent(crate::models::Continent),
    Language(crate::models::Language),
    Currency(crate::models::Currency),
    Phone(crate::models::Phone),
    HealthAntivirus(crate::models::HealthAntivirus),
    HealthQueue(crate::models::HealthQueue),
    HealthStatus(crate::models::HealthStatus),
    HealthTime(crate::models::HealthTime),
    Metric(crate::models::Metric),
    UsageDatabase(crate::models::UsageDatabase),
    UsageCollection(crate::models::UsageCollection),
    UsageUsers(crate::models::UsageUsers),
    UsageStorage(crate::models::UsageStorage),
    UsageBuckets(crate::models::UsageBuckets),
    UsageFunctions(crate::models::UsageFunctions),
    UsageProject(crate::models::UsageProject),
    }

// Converts optionals into normal ParamTypes
fn handleOptional(param: ParamType) -> Option<ParamType> {
    match param {
        ParamType::OptionalBool(value) => match value {
            Some(data) => Some(ParamType::Bool(data)),
            None => None
        }
        ParamType::OptionalNumber(value) => match value {
            Some(data) => Some(ParamType::Number(data)),
            None => None
        }
        ParamType::OptionalArray(value) => match value {
            Some(data) => Some(ParamType::Array(data)),
            None => None
        }
        ParamType::OptionalFilePath(value) => match value {
            Some(data) => Some(ParamType::FilePath(data)),
            None => None
        }
        ParamType::OptionalObject(value) => match value {
            Some(data) => Some(ParamType::Object(data)),
            None => None
        }
        ParamType::OptionalFloat(value) => match value {
            Some(data) => Some(ParamType::Float(data)),
            None => None
        }
        _ => Some(param)
    }
}

/// Example
/// ```rust
/// let mut client = appwrite::client::Client::new();
/// 
/// client.set_endpoint("Your Endpoint URL");
/// client.set_project("Your Project ID");
/// client.set_key("Your API Key");
/// 
/// // Create a user as a example
/// let userService = appwrite::services::Users::new(&client);
/// let response = userService.create("amadeus@example.com", "supersecurepassword", "Wolfgang Amadeus Mozart");
/// 
/// println!("{}", response.text().unwrap()); // Here you can also check the status code to see success
/// ```
impl Client {
    pub fn new() -> Self {
        let mut new_headers = HeaderMap::new();

        new_headers.insert("x-sdk-version", "appwrite:rust:0.0.2".parse().unwrap());
        new_headers.insert("user-agent", format!("{}-rust-{}", std::env::consts::OS, "0.0.2").parse().unwrap());
        new_headers.insert("X-Appwrite-Response-Format", "0.7.0".parse().unwrap());

        Self {
            endpoint: "https://HOSTNAME/v1".parse().unwrap(),
            headers: new_headers,
            client: reqwest::blocking::Client::builder()
            .build().unwrap(),
        }
    }

    pub fn add_header(&mut self, key: String, value: String) {
        self.headers.append(
            reqwest::header::HeaderName::from_str(&key).unwrap(),
            (&value.to_lowercase()).parse().unwrap(),
        );
    }

    pub fn add_self_signed(&mut self, value: bool) {
      self.client = reqwest::blocking::Client::builder().danger_accept_invalid_certs(value).build().unwrap();
    }

    /// Sets Your project ID
    pub fn set_project(&mut self, value: &str) {
        self.add_header("X-Appwrite-Project".to_string(), value.to_string())
    }

    /// Sets Your secret API key
    pub fn set_key(&mut self, value: &str) {
        self.add_header("X-Appwrite-Key".to_string(), value.to_string())
    }

    /// Sets Your secret JSON Web Token
    pub fn set_jwt(&mut self, value: &str) {
        self.add_header("X-Appwrite-JWT".to_string(), value.to_string())
    }

    pub fn set_locale(&mut self, value: &str) {
        self.add_header("X-Appwrite-Locale".to_string(), value.to_string())
    }

    pub fn set_mode(&mut self, value: &str) {
        self.add_header("X-Appwrite-Mode".to_string(), value.to_string())
    }


    pub fn set_endpoint(&mut self, endpoint: &str) {
        self.endpoint = endpoint.parse().unwrap()
    }

    pub fn call(
        self,
        method: &str,
        path: &str,
        headers: Option<HashMap<String, String>>,
        params: Option<HashMap<String, ParamType>>,
    ) -> Result<reqwest::blocking::Response, AppwriteException> {
        // If we have headers in the function call we combine them with the client headers.

        let mut content_type: String = "application/json".to_string();

        let request_headers: HeaderMap = match headers {
            Some(data) => {
                let mut headers = self.headers.clone();

                for (k, v) in data {
                  if k == "content-type" {
                    content_type = v.to_string()
                  } else {
                    headers.append(
                        reqwest::header::HeaderName::from_lowercase(k.as_bytes()).unwrap(),
                        (&v.to_lowercase()).parse().unwrap(),
                    );
                  }
                }

                headers
            }
            None => self.headers.clone(),
        };

        // Now start building request with reqwest
        let method_type = match method {
            "GET" => reqwest::Method::GET,
            "POST" => reqwest::Method::POST,
            "OPTIONS" => reqwest::Method::OPTIONS,
            "PUT" => reqwest::Method::PUT,
            "DELETE" => reqwest::Method::DELETE,
            "HEAD" => reqwest::Method::HEAD,
            "PATCH" => reqwest::Method::PATCH,
            _ => reqwest::Method::GET,
        };

        let mut request = self
            .client
            .request(method_type, self.endpoint.join(&format!("{}{}", "v1", path)).unwrap());

        match params {
            Some(data) => {
                let flattened_data = flatten(FlattenType::Normal(data.clone()), None);

                // Handle Optional Values
                // Remove all optionals that result in None
                // Turn all Optional____ into their non optional equivilants.
                let mut buffer: Vec<(String, ParamType)> = Vec::new();
                for (k, v) in flattened_data {
                    match handleOptional(v) {
                        Some(data) => buffer.push((k, data)),
                        None => {}
                    }
                }
                let flattened_data = buffer;

                // First flatten the data and feed it into a FormData
                if content_type.starts_with("multipart/form-data") {
                    let mut form = reqwest::blocking::multipart::Form::new();

                    for (k, v) in flattened_data.clone() {
                        match v {
                            ParamType::Bool(data) => {
                                form = form.text(k, data.to_string());
                            }
                            ParamType::String(data) => form = form.text(k, data),
                            ParamType::FilePath(data) => form = form.file(k, data).unwrap(),
                            ParamType::Number(data) => form = form.text(k, data.to_string()),
                            ParamType::Float(data) => form = form.text(k, data.to_string()),
                            ParamType::StreamData(data, filename) => form = form.part(k, reqwest::blocking::multipart::Part::bytes(data).file_name(filename)),
                            // This shouldn't be possible due to the flatten function, so we won't handle this for now
                            ParamType::Array(_data) => {
                                //todo: Feed this back into a flatten function if needed
                            },
                            ParamType::Object(_data) => {
                              // Same for this
                            },
                            _ => {}
                        }
                    }
                    request = request.multipart(form);
                }

                if content_type.starts_with("application/json") && method != "GET" {
                    request = request.json(&data);
                }

                if method == "GET" {
                    request = request.query(&queryize_data(flatten(FlattenType::Normal(data), None)));
                }
            }
            None => {}
        }

        request = request.headers(request_headers);

        match request.send() {
            Ok(data) => {
                if data.status().is_success() {
                    Ok(data)    
                } else {
                    let dataString = match data.text() {
                      Ok(data) => {data},
                      Err(err) => {
                        // Last Resort. Called if string isn't even readable text.
                        return Err(AppwriteException::new(format!("A error occoured. ERR: {}, This could either be a connection error or an internal Appwrite error. Please check your Appwrite instance logs. ", err), 0, "".to_string()))
                      }
                    };

                    // Format error
                    Err(match serde_json::from_str(&dataString) {
                        Ok(data) => data,
                        Err(_err) => {
                            AppwriteException::new(format!("{}", dataString), 0, "".to_string())
                        }
                    })
                }
            },
            Err(err) => {
                // Throw Appwrite Exception
                Err(AppwriteException::new(format!("{}", err), 0, "".to_string()))
             },
        }
    }
}

enum FlattenType {
    Normal(HashMap<String, ParamType>),
    Nested(Vec<ParamType>),
}

fn queryize_data(data: Vec<(String, ParamType)>) -> Vec<(String, String)> {
    let mut output: Vec<(String, String)> = Default::default();

    for (k, v) in data {
        match v {
            ParamType::Bool(value) => output.push((k, value.to_string())),
            ParamType::String(value) => output.push((k, value)),
            ParamType::Number(value) => output.push((k, value.to_string())),
            _ => {}
        }
    }

    output
}

fn flatten(data: FlattenType, prefix: Option<String>) -> Vec<(String, ParamType)> {
    let mut output: Vec<(String, ParamType)> = Default::default();

    match data {
        FlattenType::Normal(data) => {
            for (k, v) in data {
                let final_key = match &prefix {
                    Some(current_prefix) => format!("{}[{}]", current_prefix, k),
                    None => k,
                };

                match v {
                    ParamType::Array(value) => {
                        output.append(&mut flatten(FlattenType::Nested(value), Some(final_key)));
                    }
                    ParamType::Object(value) => {
                        output.extend(flatten(FlattenType::Normal(value), Some(final_key)).into_iter())
                    },
                    value => {
                        output.push((final_key, value));
                    }
                }
            }
        }

        FlattenType::Nested(data) => {
            for (k, v) in data.iter().enumerate() {
                let final_key = match &prefix {
                    Some(current_prefix) => format!("{}[{}]", current_prefix, k),
                    None => k.to_string(),
                };

                match v {
                    ParamType::Array(value) => {
                        flatten(FlattenType::Nested(value.to_owned()), Some(final_key))
                            .append(&mut output);
                    }
                    value => {
                        output.push((final_key, value.to_owned()));
                    }
                }
            }
        }
    }

    output
}
