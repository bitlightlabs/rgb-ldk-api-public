// Generated. Do not edit.
// Run: `python3 scripts/rgb-ldk-api.py gen`

use std::path::PathBuf;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Url;
use thiserror::Error;

#[derive(Debug, Clone)]
pub enum Auth {
	None,
	FixedToken(String),
	TokenFile(PathBuf),
}

#[derive(Debug, Error)]
pub enum ClientError {
	#[error("invalid base url: {0}")]
	InvalidBaseUrl(String),
	#[error("invalid bearer token")]
	InvalidBearerToken,
	#[error("read token file failed: {0}")]
	ReadTokenFile(std::io::Error),
	#[error("http error: {0}")]
	Http(#[from] reqwest::Error),
	#[error("json error: {0}")]
	Json(#[from] serde_json::Error),
	#[error("non-success status: {status} body={body}")]
	NonSuccess { status: u16, body: String },
}

#[derive(Clone)]
pub struct Client {
	base_url: Url,
	http: reqwest::Client,
	auth: Auth,
}

impl Client {
	pub fn new(base_url: &str, auth: Auth) -> Result<Self, ClientError> {
		let base_url = Url::parse(base_url)
			.map_err(|_| ClientError::InvalidBaseUrl(base_url.to_string()))?;
		Ok(Self { base_url, http: reqwest::Client::new(), auth })
	}

	fn read_token_file(path: &std::path::Path) -> Result<String, std::io::Error> {
		Ok(std::fs::read_to_string(path)?.trim().to_string())
	}

	async fn headers(&self) -> Result<HeaderMap, ClientError> {
		let mut headers = HeaderMap::new();
		let token_opt = match &self.auth {
			Auth::None => None,
			Auth::FixedToken(t) => Some(t.clone()),
			Auth::TokenFile(p) => {
				Some(Self::read_token_file(p).map_err(ClientError::ReadTokenFile)?)
			},
		};
		if let Some(token) = token_opt {
			let v = HeaderValue::from_str(&format!("Bearer {token}"))
				.map_err(|_| ClientError::InvalidBearerToken)?;
			headers.insert(AUTHORIZATION, v);
		}
		Ok(headers)
	}

	pub async fn get_json<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, ClientError> {
		let url = self.base_url.join(path)
			.map_err(|_| ClientError::InvalidBaseUrl(path.to_string()))?;
		let headers = self.headers().await?;
		let resp = self.http.get(url).headers(headers).send().await?;
		let status = resp.status();
		let body = resp.text().await?;
		if !status.is_success() {
			return Err(ClientError::NonSuccess { status: status.as_u16(), body });
		}
		Ok(serde_json::from_str(&body)?)
	}

	pub async fn post_json<B: serde::Serialize, T: serde::de::DeserializeOwned>(
		&self,
		path: &str,
		body: &B,
	) -> Result<T, ClientError> {
		let url = self.base_url.join(path)
			.map_err(|_| ClientError::InvalidBaseUrl(path.to_string()))?;
		let headers = self.headers().await?;
		let resp = self.http.post(url).headers(headers).json(body).send().await?;
		let status = resp.status();
		let body = resp.text().await?;
		if !status.is_success() {
			return Err(ClientError::NonSuccess { status: status.as_u16(), body });
		}
		Ok(serde_json::from_str(&body)?)
	}
}

