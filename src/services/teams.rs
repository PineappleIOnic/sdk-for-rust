use crate::client::{Client, ParamType};
use std::collections::HashMap;
use crate::services::AppwriteException;

#[derive(Clone)]
pub struct Teams {
  client: Client
}

impl Teams {  
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone()
        }
    }

    /// Get a list of all the current user teams. You can use the query params to
    /// filter your results. On admin mode, this endpoint will return a list of all
    /// of the project's teams. [Learn more about different API
    /// modes](/docs/admin).
    pub fn list(&self, search: Option<&str>, limit: Option<i64>, offset: Option<i64>, order_type: Option<&str>) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/teams";

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

    /// Create a new team. The user who creates the team will automatically be
    /// assigned as the owner of the team. The team owner can invite new members,
    /// who will be able add new owners and update or delete the team from your
    /// project.
    pub fn create(&self, name: &str, roles: Option<&[&str]>) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/teams";

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

    let roles:&[&str] = match roles {
        Some(data) => data,
        None => &[]
    };

        let params: HashMap<String, ParamType> = [
            ("name".to_string(), ParamType::String(name.to_string())),
            ("roles".to_string(), ParamType::Array(roles.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
        ].iter().cloned().collect();

        return self.client.clone().call("POST", &path, Some(headers), Some(params) );
    }

    /// Get a team by its unique ID. All team members have read access for this
    /// resource.
    pub fn get(&self, team_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/teams/teamId".replace("teamId", &team_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();


        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("GET", &path, Some(headers), Some(params) );
    }

    /// Update a team by its unique ID. Only team owners have write access for this
    /// resource.
    pub fn update(&self, team_id: &str, name: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/teams/teamId".replace("teamId", &team_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();


        let params: HashMap<String, ParamType> = [
            ("name".to_string(), ParamType::String(name.to_string())),
        ].iter().cloned().collect();

        return self.client.clone().call("PUT", &path, Some(headers), Some(params) );
    }

    /// Delete a team by its unique ID. Only team owners have write access for this
    /// resource.
    pub fn delete(&self, team_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/teams/teamId".replace("teamId", &team_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();


        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("DELETE", &path, Some(headers), Some(params) );
    }

    /// Get a team members by the team unique ID. All team members have read access
    /// for this list of resources.
    pub fn get_memberships(&self, team_id: &str, search: Option<&str>, limit: Option<i64>, offset: Option<i64>, order_type: Option<&str>) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/teams/teamId/memberships".replace("teamId", &team_id);

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

    /// Use this endpoint to invite a new member to join your team. An email with a
    /// link to join the team will be sent to the new member email address if the
    /// member doesn't exist in the project it will be created automatically.
    /// 
    /// Use the 'URL' parameter to redirect the user from the invitation email back
    /// to your app. When the user is redirected, use the [Update Team Membership
    /// Status](/docs/client/teams#teamsUpdateMembershipStatus) endpoint to allow
    /// the user to accept the invitation to the team.
    /// 
    /// Please note that in order to avoid a [Redirect
    /// Attacks](https://github.com/OWASP/CheatSheetSeries/blob/master/cheatsheets/Unvalidated_Redirects_and_Forwards_Cheat_Sheet.md)
    /// the only valid redirect URL's are the once from domains you have set when
    /// added your platforms in the console interface.
    pub fn create_membership(&self, team_id: &str, email: &str, roles: &[&str], url: &str, name: Option<&str>) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/teams/teamId/memberships".replace("teamId", &team_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();

    let name:&str = match name {
        Some(data) => data,
        None => ""
    };

        let params: HashMap<String, ParamType> = [
            ("email".to_string(), ParamType::String(email.to_string())),
            ("name".to_string(), ParamType::String(name.to_string())),
            ("roles".to_string(), ParamType::Array(roles.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
            ("url".to_string(), ParamType::String(url.to_string())),
        ].iter().cloned().collect();

        return self.client.clone().call("POST", &path, Some(headers), Some(params) );
    }

    pub fn update_membership_roles(&self, team_id: &str, membership_id: &str, roles: &[&str]) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/teams/teamId/memberships/membershipId".replace("teamId", &team_id).replace("membershipId", &membership_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();


        let params: HashMap<String, ParamType> = [
            ("roles".to_string(), ParamType::Array(roles.into_iter().map(|x| ParamType::String(x.to_string())).collect())),
        ].iter().cloned().collect();

        return self.client.clone().call("PATCH", &path, Some(headers), Some(params) );
    }

    /// This endpoint allows a user to leave a team or for a team owner to delete
    /// the membership of any other team member. You can also use this endpoint to
    /// delete a user membership even if it is not accepted.
    pub fn delete_membership(&self, team_id: &str, membership_id: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/teams/teamId/memberships/membershipId".replace("teamId", &team_id).replace("membershipId", &membership_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();


        let params: HashMap<String, ParamType> = [
        ].iter().cloned().collect();

        return self.client.clone().call("DELETE", &path, Some(headers), Some(params) );
    }

    /// Use this endpoint to allow a user to accept an invitation to join a team
    /// after being redirected back to your app from the invitation email recieved
    /// by the user.
    pub fn update_membership_status(&self, team_id: &str, membership_id: &str, user_id: &str, secret: &str) -> Result<reqwest::blocking::Response, AppwriteException> {
        let path = "/teams/teamId/memberships/membershipId/status".replace("teamId", &team_id).replace("membershipId", &membership_id);

        let headers: HashMap<String, String> = [
            ("content-type".to_string(), "application/json".to_string()),
        ].iter().cloned().collect();


        let params: HashMap<String, ParamType> = [
            ("userId".to_string(), ParamType::String(user_id.to_string())),
            ("secret".to_string(), ParamType::String(secret.to_string())),
        ].iter().cloned().collect();

        return self.client.clone().call("PATCH", &path, Some(headers), Some(params) );
    }
}
