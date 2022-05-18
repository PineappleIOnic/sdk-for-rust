#![allow(unused)]
use serde::{Deserialize, Serialize, Deserializer};
use std::collections::HashMap;
use serde_json::value::Value;
use std::fmt::Display;
use super::*;

#[derive(Debug, Serialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum EmptyOption<T> {
    Some(T),
    None {},
}

impl<T> Display for EmptyOption<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EmptyOption::Some(t) => write!(f, "{}", t),
            EmptyOption::None {} => write!(f, ""),
        }
    }
}

impl<'de, T> Deserialize<'de> for EmptyOption<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::deserialize(deserializer).map(Into::into)
    }
}

impl<T> From<EmptyOption<T>> for Option<T> {
    fn from(empty_option: EmptyOption<T>) -> Option<T> {
        match empty_option {
            EmptyOption::Some(option) => Some(option),
            EmptyOption::None {} => None,
        }
    }
}

impl<T> From<Option<T>> for EmptyOption<T> {
    fn from(option: Option<T>) -> EmptyOption<T> {
        match option {
            Some(option) => EmptyOption::Some(option),
            None {} => EmptyOption::None {},
        }
    }
}

impl<T> EmptyOption<T> {
    fn into_option(self) -> Option<T> {
        self.into()
    }
    fn as_option(&self) -> Option<&T> {
        match self {
            EmptyOption::Some(option) => Some(option),
            EmptyOption::None {} => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
        #[serde(rename(serialize = "id", deserialize = "$id"))]
        pub id: String,
        pub name: String,
        pub description: String,
        pub teamId: String,
        pub logo: String,
        pub url: String,
        pub legalName: String,
        pub legalCountry: String,
        pub legalState: String,
        pub legalCity: String,
        pub legalAddress: String,
        pub legalTaxId: String,
        pub authLimit: i64,
        pub platforms: Vec<Platform>,
        pub webhooks: Vec<Webhook>,
        pub keys: Vec<Key>,
        pub domains: Vec<Domain>,
        pub providerAmazonAppid: String,
        pub providerAmazonSecret: String,
        pub providerAppleAppid: String,
        pub providerAppleSecret: String,
        pub providerBitbucketAppid: String,
        pub providerBitbucketSecret: String,
        pub providerBitlyAppid: String,
        pub providerBitlySecret: String,
        pub providerBoxAppid: String,
        pub providerBoxSecret: String,
        pub providerDiscordAppid: String,
        pub providerDiscordSecret: String,
        pub providerDropboxAppid: String,
        pub providerDropboxSecret: String,
        pub providerFacebookAppid: String,
        pub providerFacebookSecret: String,
        pub providerGithubAppid: String,
        pub providerGithubSecret: String,
        pub providerGitlabAppid: String,
        pub providerGitlabSecret: String,
        pub providerGoogleAppid: String,
        pub providerGoogleSecret: String,
        pub providerLinkedinAppid: String,
        pub providerLinkedinSecret: String,
        pub providerMicrosoftAppid: String,
        pub providerMicrosoftSecret: String,
        pub providerNotionAppid: String,
        pub providerNotionSecret: String,
        pub providerPaypalAppid: String,
        pub providerPaypalSecret: String,
        pub providerPaypalSandboxAppid: String,
        pub providerPaypalSandboxSecret: String,
        pub providerSalesforceAppid: String,
        pub providerSalesforceSecret: String,
        pub providerSlackAppid: String,
        pub providerSlackSecret: String,
        pub providerSpotifyAppid: String,
        pub providerSpotifySecret: String,
        pub providerTradeshiftAppid: String,
        pub providerTradeshiftSecret: String,
        pub providerTradeshiftBoxAppid: String,
        pub providerTradeshiftBoxSecret: String,
        pub providerTwitchAppid: String,
        pub providerTwitchSecret: String,
        pub providerVkAppid: String,
        pub providerVkSecret: String,
        pub providerYahooAppid: String,
        pub providerYahooSecret: String,
        pub providerYammerAppid: String,
        pub providerYammerSecret: String,
        pub providerYandexAppid: String,
        pub providerYandexSecret: String,
        pub providerWordpressAppid: String,
        pub providerWordpressSecret: String,
        pub providerStripeAppid: String,
        pub providerStripeSecret: String,
        pub providerMockAppid: String,
        pub providerMockSecret: String,
        pub authEmailPassword: bool,
        pub authUsersAuthMagicURL: bool,
        pub authAnonymous: bool,
        pub authInvites: bool,
        pub authJWT: bool,
        pub authPhone: bool,
        pub serviceStatusForAccount: bool,
        pub serviceStatusForAvatars: bool,
        pub serviceStatusForDatabase: bool,
        pub serviceStatusForLocale: bool,
        pub serviceStatusForHealth: bool,
        pub serviceStatusForStorage: bool,
        pub serviceStatusForTeams: bool,
        pub serviceStatusForUsers: bool,
        pub serviceStatusForFunctions: bool,
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatBuffer = String::new();
        formatBuffer.push_str(&format!("{:?}", self.id));
        formatBuffer.push_str(&format!("{:?}", self.name));
        formatBuffer.push_str(&format!("{:?}", self.description));
        formatBuffer.push_str(&format!("{:?}", self.teamId));
        formatBuffer.push_str(&format!("{:?}", self.logo));
        formatBuffer.push_str(&format!("{:?}", self.url));
        formatBuffer.push_str(&format!("{:?}", self.legalName));
        formatBuffer.push_str(&format!("{:?}", self.legalCountry));
        formatBuffer.push_str(&format!("{:?}", self.legalState));
        formatBuffer.push_str(&format!("{:?}", self.legalCity));
        formatBuffer.push_str(&format!("{:?}", self.legalAddress));
        formatBuffer.push_str(&format!("{:?}", self.legalTaxId));
        formatBuffer.push_str(&format!("{:?}", self.authLimit));
        for item in &self.platforms {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.webhooks {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.keys {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        for item in &self.domains {
            formatBuffer.push_str(&format!("{:?}", item));
        }
        formatBuffer.push_str(&format!("{:?}", self.providerAmazonAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerAmazonSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerAppleAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerAppleSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerBitbucketAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerBitbucketSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerBitlyAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerBitlySecret));
        formatBuffer.push_str(&format!("{:?}", self.providerBoxAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerBoxSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerDiscordAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerDiscordSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerDropboxAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerDropboxSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerFacebookAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerFacebookSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerGithubAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerGithubSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerGitlabAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerGitlabSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerGoogleAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerGoogleSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerLinkedinAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerLinkedinSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerMicrosoftAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerMicrosoftSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerNotionAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerNotionSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerPaypalAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerPaypalSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerPaypalSandboxAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerPaypalSandboxSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerSalesforceAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerSalesforceSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerSlackAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerSlackSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerSpotifyAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerSpotifySecret));
        formatBuffer.push_str(&format!("{:?}", self.providerTradeshiftAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerTradeshiftSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerTradeshiftBoxAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerTradeshiftBoxSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerTwitchAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerTwitchSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerVkAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerVkSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerYahooAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerYahooSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerYammerAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerYammerSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerYandexAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerYandexSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerWordpressAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerWordpressSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerStripeAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerStripeSecret));
        formatBuffer.push_str(&format!("{:?}", self.providerMockAppid));
        formatBuffer.push_str(&format!("{:?}", self.providerMockSecret));
        formatBuffer.push_str(&format!("{:?}", self.authEmailPassword));
        formatBuffer.push_str(&format!("{:?}", self.authUsersAuthMagicURL));
        formatBuffer.push_str(&format!("{:?}", self.authAnonymous));
        formatBuffer.push_str(&format!("{:?}", self.authInvites));
        formatBuffer.push_str(&format!("{:?}", self.authJWT));
        formatBuffer.push_str(&format!("{:?}", self.authPhone));
        formatBuffer.push_str(&format!("{:?}", self.serviceStatusForAccount));
        formatBuffer.push_str(&format!("{:?}", self.serviceStatusForAvatars));
        formatBuffer.push_str(&format!("{:?}", self.serviceStatusForDatabase));
        formatBuffer.push_str(&format!("{:?}", self.serviceStatusForLocale));
        formatBuffer.push_str(&format!("{:?}", self.serviceStatusForHealth));
        formatBuffer.push_str(&format!("{:?}", self.serviceStatusForStorage));
        formatBuffer.push_str(&format!("{:?}", self.serviceStatusForTeams));
        formatBuffer.push_str(&format!("{:?}", self.serviceStatusForUsers));
        formatBuffer.push_str(&format!("{:?}", self.serviceStatusForFunctions));

        write!(f, "{}", formatBuffer)
    }
}

impl Project {
    pub fn new(id: String, name: String, description: String, teamId: String, logo: String, url: String, legalName: String, legalCountry: String, legalState: String, legalCity: String, legalAddress: String, legalTaxId: String, authLimit: i64, platforms: Vec<Platform>, webhooks: Vec<Webhook>, keys: Vec<Key>, domains: Vec<Domain>, providerAmazonAppid: String, providerAmazonSecret: String, providerAppleAppid: String, providerAppleSecret: String, providerBitbucketAppid: String, providerBitbucketSecret: String, providerBitlyAppid: String, providerBitlySecret: String, providerBoxAppid: String, providerBoxSecret: String, providerDiscordAppid: String, providerDiscordSecret: String, providerDropboxAppid: String, providerDropboxSecret: String, providerFacebookAppid: String, providerFacebookSecret: String, providerGithubAppid: String, providerGithubSecret: String, providerGitlabAppid: String, providerGitlabSecret: String, providerGoogleAppid: String, providerGoogleSecret: String, providerLinkedinAppid: String, providerLinkedinSecret: String, providerMicrosoftAppid: String, providerMicrosoftSecret: String, providerNotionAppid: String, providerNotionSecret: String, providerPaypalAppid: String, providerPaypalSecret: String, providerPaypalSandboxAppid: String, providerPaypalSandboxSecret: String, providerSalesforceAppid: String, providerSalesforceSecret: String, providerSlackAppid: String, providerSlackSecret: String, providerSpotifyAppid: String, providerSpotifySecret: String, providerTradeshiftAppid: String, providerTradeshiftSecret: String, providerTradeshiftBoxAppid: String, providerTradeshiftBoxSecret: String, providerTwitchAppid: String, providerTwitchSecret: String, providerVkAppid: String, providerVkSecret: String, providerYahooAppid: String, providerYahooSecret: String, providerYammerAppid: String, providerYammerSecret: String, providerYandexAppid: String, providerYandexSecret: String, providerWordpressAppid: String, providerWordpressSecret: String, providerStripeAppid: String, providerStripeSecret: String, providerMockAppid: String, providerMockSecret: String, authEmailPassword: bool, authUsersAuthMagicURL: bool, authAnonymous: bool, authInvites: bool, authJWT: bool, authPhone: bool, serviceStatusForAccount: bool, serviceStatusForAvatars: bool, serviceStatusForDatabase: bool, serviceStatusForLocale: bool, serviceStatusForHealth: bool, serviceStatusForStorage: bool, serviceStatusForTeams: bool, serviceStatusForUsers: bool, serviceStatusForFunctions: bool, ) -> Self {
        Self {
            id: id,
            name: name,
            description: description,
            teamId: teamId,
            logo: logo,
            url: url,
            legalName: legalName,
            legalCountry: legalCountry,
            legalState: legalState,
            legalCity: legalCity,
            legalAddress: legalAddress,
            legalTaxId: legalTaxId,
            authLimit: authLimit,
            platforms: platforms,
            webhooks: webhooks,
            keys: keys,
            domains: domains,
            providerAmazonAppid: providerAmazonAppid,
            providerAmazonSecret: providerAmazonSecret,
            providerAppleAppid: providerAppleAppid,
            providerAppleSecret: providerAppleSecret,
            providerBitbucketAppid: providerBitbucketAppid,
            providerBitbucketSecret: providerBitbucketSecret,
            providerBitlyAppid: providerBitlyAppid,
            providerBitlySecret: providerBitlySecret,
            providerBoxAppid: providerBoxAppid,
            providerBoxSecret: providerBoxSecret,
            providerDiscordAppid: providerDiscordAppid,
            providerDiscordSecret: providerDiscordSecret,
            providerDropboxAppid: providerDropboxAppid,
            providerDropboxSecret: providerDropboxSecret,
            providerFacebookAppid: providerFacebookAppid,
            providerFacebookSecret: providerFacebookSecret,
            providerGithubAppid: providerGithubAppid,
            providerGithubSecret: providerGithubSecret,
            providerGitlabAppid: providerGitlabAppid,
            providerGitlabSecret: providerGitlabSecret,
            providerGoogleAppid: providerGoogleAppid,
            providerGoogleSecret: providerGoogleSecret,
            providerLinkedinAppid: providerLinkedinAppid,
            providerLinkedinSecret: providerLinkedinSecret,
            providerMicrosoftAppid: providerMicrosoftAppid,
            providerMicrosoftSecret: providerMicrosoftSecret,
            providerNotionAppid: providerNotionAppid,
            providerNotionSecret: providerNotionSecret,
            providerPaypalAppid: providerPaypalAppid,
            providerPaypalSecret: providerPaypalSecret,
            providerPaypalSandboxAppid: providerPaypalSandboxAppid,
            providerPaypalSandboxSecret: providerPaypalSandboxSecret,
            providerSalesforceAppid: providerSalesforceAppid,
            providerSalesforceSecret: providerSalesforceSecret,
            providerSlackAppid: providerSlackAppid,
            providerSlackSecret: providerSlackSecret,
            providerSpotifyAppid: providerSpotifyAppid,
            providerSpotifySecret: providerSpotifySecret,
            providerTradeshiftAppid: providerTradeshiftAppid,
            providerTradeshiftSecret: providerTradeshiftSecret,
            providerTradeshiftBoxAppid: providerTradeshiftBoxAppid,
            providerTradeshiftBoxSecret: providerTradeshiftBoxSecret,
            providerTwitchAppid: providerTwitchAppid,
            providerTwitchSecret: providerTwitchSecret,
            providerVkAppid: providerVkAppid,
            providerVkSecret: providerVkSecret,
            providerYahooAppid: providerYahooAppid,
            providerYahooSecret: providerYahooSecret,
            providerYammerAppid: providerYammerAppid,
            providerYammerSecret: providerYammerSecret,
            providerYandexAppid: providerYandexAppid,
            providerYandexSecret: providerYandexSecret,
            providerWordpressAppid: providerWordpressAppid,
            providerWordpressSecret: providerWordpressSecret,
            providerStripeAppid: providerStripeAppid,
            providerStripeSecret: providerStripeSecret,
            providerMockAppid: providerMockAppid,
            providerMockSecret: providerMockSecret,
            authEmailPassword: authEmailPassword,
            authUsersAuthMagicURL: authUsersAuthMagicURL,
            authAnonymous: authAnonymous,
            authInvites: authInvites,
            authJWT: authJWT,
            authPhone: authPhone,
            serviceStatusForAccount: serviceStatusForAccount,
            serviceStatusForAvatars: serviceStatusForAvatars,
            serviceStatusForDatabase: serviceStatusForDatabase,
            serviceStatusForLocale: serviceStatusForLocale,
            serviceStatusForHealth: serviceStatusForHealth,
            serviceStatusForStorage: serviceStatusForStorage,
            serviceStatusForTeams: serviceStatusForTeams,
            serviceStatusForUsers: serviceStatusForUsers,
            serviceStatusForFunctions: serviceStatusForFunctions,
            }
    }
}