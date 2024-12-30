use crate::error::Error;
use serde::Deserialize;
use std::path::Path;
use yup_oauth2::{ServiceAccountAuthenticator, ServiceAccountKey};

#[derive(Deserialize)]
pub struct ServiceAccountCreds {
    #[serde(rename = "type")]
    auth_type: String,
    project_id: String,
    private_key_id: String,
    private_key: String,
    client_email: String,
    client_id: String,
    auth_uri: String,
    token_uri: String,
    auth_provider_x509_cert_url: String,
    client_x509_cert_url: String,
}

pub struct Authenticator {
    api_key: Option<String>,
    service_account: Option<ServiceAccountCreds>,
}

impl Authenticator {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            api_key,
            service_account: None,
        }
    }

    pub fn from_service_account_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let cred_bytes = std::fs::read_to_string(path).map_err(|e| {
            Error::AuthenticationError(format!("Failed to read credentials file: {}", e))
        })?;

        let creds: ServiceAccountCreds = serde_json::from_str(&cred_bytes).map_err(|e| {
            Error::AuthenticationError(format!("Failed to parse credentials: {}", e))
        })?;

        Ok(Self {
            api_key: None,
            service_account: Some(creds),
        })
    }

    pub fn get_api_key(&self) -> Result<String, Error> {
        match &self.api_key {
            Some(key) => Ok(key.clone()),
            None => Err(Error::AuthenticationError(String::from(
                "No API key provided.",
            ))),
        }
    }

    pub fn has_service_account(&self) -> bool {
        self.service_account.is_some()
    }

    pub async fn get_token(&self) -> Result<String, Error> {
        let creds = self.service_account.as_ref().ok_or_else(|| {
            Error::AuthenticationError("No service account credentials available.".to_string())
        })?;

        let service_account_key = ServiceAccountKey {
            key_type: Some(creds.auth_type.clone()),
            project_id: Some(creds.project_id.clone()),
            private_key_id: Some(creds.private_key_id.clone()),
            private_key: creds.private_key.clone(),
            client_email: creds.client_email.clone(),
            client_id: Some(creds.client_id.clone()),
            auth_uri: Some(creds.auth_uri.clone()),
            token_uri: creds.token_uri.clone(),
            auth_provider_x509_cert_url: Some(creds.auth_provider_x509_cert_url.clone()),
            client_x509_cert_url: Some(creds.client_x509_cert_url.clone()),
        };

        let auth = ServiceAccountAuthenticator::builder(service_account_key)
            .build()
            .await
            .map_err(|e| {
                Error::AuthenticationError(format!("Failed to create authenticator: {}", e))
            })?;

        let scopes = &["https://www.googleapis.com/auth/spreadsheets"];
        let token = auth
            .token(scopes)
            .await
            .map_err(|e| Error::AuthenticationError(format!("Failed to get token: {}", e)))?;

        Ok(token.as_str().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_authenticator() {
        let auth = Authenticator::new(Some("test_key".to_string()));
        assert!(auth.get_api_key().is_ok());
        assert!(!auth.has_service_account());
    }

    #[tokio::test]
    async fn test_get_token_no_service_account() {
        let auth = Authenticator::new(None);
        let result = auth.get_token().await;
        assert!(result.is_err());
        match result {
            Err(Error::AuthenticationError(msg)) => {
                assert_eq!(msg, "No service account credentials available.");
            }
            _ => panic!("Expected AuthenticationError"),
        }
    }
}
