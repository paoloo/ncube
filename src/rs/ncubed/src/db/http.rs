//! When connecting to remote ncube installation all requests are done using
//! HTTP. Internally the HTTP endpoint is treated like a database.
//!
//! # Example
//!
//! ```no_run
//! use ncubed::db::http;
//! use hyper::{Body, Method, Request};
//! # #[tokio::main]
//! # async fn main() {
//! let endpoint = "https://example.org";
//! let cfg = endpoint.parse::<http::Config>().unwrap();
//! let client = http::Database::new(cfg);
//! // client.get("workspaces")
//! // client.put("workspaces/1", ..)
//! // ..
//! # }
//! ```
use bytes::{buf::BufExt as _, Buf};
use hyper::{client::HttpConnector, Body, Client, Method, Request, Uri};
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use thiserror::Error;
use url::Url;

use crate::errors::StoreError;
use crate::http::SuccessResponse;

#[derive(Error, Debug)]
pub struct HttpConfigError;

impl Display for HttpConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HttpConfigError")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub(crate) endpoint: Url,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            endpoint: Url::parse("http://127.0.0.1:40666").unwrap(),
        }
    }
}

impl FromStr for Config {
    type Err = HttpConfigError;

    fn from_str(s: &str) -> Result<Self, HttpConfigError> {
        let endpoint = Url::parse(s).map_err(|_| HttpConfigError)?;

        Ok(Config { endpoint })
    }
}

#[derive(Clone)]
pub struct Database {
    config: Config,
    client: ClientWrapper,
}

impl PartialEq for Database {
    fn eq(&self, other: &Self) -> bool {
        self.config == other.config
    }
}

impl Debug for Database {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "Http::Database({:?})", self.config)
    }
}

impl Database {
    /// Construct a HTTP client.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ncubed::db::http;
    /// # #[tokio::main]
    /// # async fn main () {
    /// let config = "http://example.org".parse::<http::Config>().unwrap();
    /// let db = http::Database::new(config);
    /// // Run a query on the connection object.
    /// # }
    /// ```
    pub fn new(config: Config) -> Self {
        let client = Client::new();

        Self {
            client: ClientWrapper::new(client),
            config,
        }
    }

    async fn execute(&self, req: Request<Body>) -> Result<impl Buf, hyper::error::Error> {
        let res = self.client.request(req).await?;
        let body = hyper::body::aggregate(res).await?;

        Ok(body)
    }

    fn url(&self, path: &str) -> Uri {
        let mut uri = self.config.endpoint.clone();
        uri.set_path(path);
        Uri::from_str(uri.as_str()).unwrap()
    }

    #[allow(dead_code)]
    pub(crate) async fn get<T>(&self, path: &str) -> Result<SuccessResponse<T>, StoreError>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = self.url(&path);
        let req = Request::builder()
            .method(Method::GET)
            .uri(url)
            .header("content-type", "application/json")
            .body(Default::default())
            .unwrap();
        let body = self.execute(req).await?;
        let data = serde_json::from_reader(body.reader())?;

        Ok(data)
    }

    #[allow(dead_code)]
    pub(crate) async fn post<T, S>(
        &self,
        path: &str,
        payload: S,
    ) -> Result<SuccessResponse<T>, StoreError>
    where
        T: serde::de::DeserializeOwned,
        S: serde::Serialize,
    {
        let url = self.url(&path);
        let payload_json = serde_json::to_string(&payload).unwrap().into_bytes();
        let req = Request::post(url)
            .header("content-type", "application/json")
            .body(Body::from(payload_json))
            .unwrap();
        let body = self.execute(req).await?;
        let data = serde_json::from_reader(body.reader())?;

        Ok(data)
    }

    #[allow(dead_code)]
    pub(crate) async fn put<T, S>(
        &self,
        path: &str,
        payload: S,
    ) -> Result<SuccessResponse<T>, StoreError>
    where
        T: serde::de::DeserializeOwned,
        S: serde::Serialize,
    {
        let url = self.url(&path);
        let payload_json = serde_json::to_string(&payload).unwrap().into_bytes();
        let req = Request::put(url)
            .header("content-type", "application/json")
            .body(Body::from(payload_json))
            .unwrap();
        let body = self.execute(req).await?;
        let data = serde_json::from_reader(body.reader())?;

        Ok(data)
    }

    #[allow(dead_code)]
    pub(crate) async fn delete<T, S>(&self, path: &str) -> Result<SuccessResponse<T>, StoreError>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = self.url(&path);
        let req = Request::delete(url)
            .header("content-type", "application/json")
            .body(Default::default())
            .unwrap();

        let body = self.execute(req).await?;
        let data = serde_json::from_reader(body.reader())?;

        Ok(data)
    }
}

impl FromStr for Database {
    type Err = HttpConfigError;

    fn from_str(connection_string: &str) -> Result<Self, HttpConfigError> {
        let config: Config = connection_string.parse::<Config>()?;
        let client = Client::new();

        Ok(Self {
            client: ClientWrapper::new(client),
            config,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ClientWrapper {
    client: Client<HttpConnector, Body>,
}

impl ClientWrapper {
    pub(crate) fn new(client: Client<HttpConnector, Body>) -> Self {
        Self { client }
    }
}

impl Deref for ClientWrapper {
    type Target = Client<HttpConnector, Body>;
    fn deref(&self) -> &Client<HttpConnector, Body> {
        &self.client
    }
}

impl DerefMut for ClientWrapper {
    fn deref_mut(&mut self) -> &mut Client<HttpConnector, Body> {
        &mut self.client
    }
}