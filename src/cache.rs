use derivative::Derivative;
use http::{request, response};
use http_cache_semantics::{AfterResponse, BeforeRequest, CachePolicy, ResponseLike};
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::{Client, Request, RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

#[derive(Debug)]
pub enum CachedHttpResponse {
    Hit(HttpResponse),
    Miss(HttpResponse),
}

impl CachedHttpResponse {
    pub fn reponse(&self) -> &HttpResponse {
        match self {
            Self::Hit(resp) => resp,
            Self::Miss(resp) => resp,
        }
    }
}

/// A basic generic type that represents an HTTP response
#[derive(Derivative, Clone, Deserialize, Serialize)]
#[derivative(Debug)]
pub struct HttpResponse {
    /// HTTP response body
    #[derivative(Debug = "ignore")]
    pub body: Vec<u8>,
    /// HTTP response headers
    #[serde(with = "http_serde::header_map")]
    pub headers: http::HeaderMap,
    /// HTTP response status code
    #[serde(with = "http_serde::status_code")]
    pub status: http::StatusCode,
    /// HTTP response url
    pub url: Url,
    /// HTTP response version
    #[serde(with = "http_serde::version")]
    pub version: http::Version,
}

impl ResponseLike for HttpResponse {
    fn status(&self) -> http::StatusCode {
        self.status
    }
    fn headers(&self) -> &http::HeaderMap {
        &self.headers
    }
}

impl HttpResponse {
    /// Returns `http::response::Parts`
    pub fn parts(&self) -> response::Parts {
        let mut converted = response::Builder::new()
            .status(self.status)
            .body(())
            .unwrap();
        {
            let headers = converted.headers_mut();
            for (key, value) in &self.headers {
                headers.insert(key, value.clone());
            }
        }
        converted.into_parts().0
    }
    /// Update the headers from `http::response::Parts`
    pub fn update_headers(&mut self, parts: &response::Parts) {
        for (key, value) in parts.headers.iter() {
            self.headers.insert(key, value.clone());
        }
    }

    async fn from_reqwest(response: Response) -> Self {
        let headers = response.headers().to_owned();
        let status = response.status();
        let version = response.version();
        let url = response.url().to_owned();
        let body: Vec<_> = response.bytes().await.unwrap().to_vec();

        Self {
            body,
            headers,
            status,
            url,
            version,
        }
    }
}

fn update_request_parts(request: RequestBuilder, parts: request::Parts) -> RequestBuilder {
    request.headers(parts.headers)
}

pub async fn get_response_with_cache(
    request_builder: RequestBuilder,
    cache: &mut HashMap<String, (CachePolicy, HttpResponse)>,
    source: &str,
) -> CachedHttpResponse {
    let orig_request = request_builder.try_clone().unwrap().build().unwrap();

    // try to pull from cache if possible
    if let Some((policy, cached_response)) = cache.get_mut(&source.to_string()) {
        match policy.before_request(&orig_request, SystemTime::now()) {
            BeforeRequest::Fresh(parts) => {
                cached_response.update_headers(&parts);
                CachedHttpResponse::Hit(cached_response.clone())
            }
            BeforeRequest::Stale { request, matches } => {
                // update parts
                let request_builder = update_request_parts(request_builder, request);
                let orig_request = request_builder.try_clone().unwrap().build().unwrap();

                let response = request_builder.send().await.unwrap();
                let mut response = HttpResponse::from_reqwest(response).await;

                match policy.after_response(&orig_request, &response, SystemTime::now()) {
                    AfterResponse::NotModified(_, parts) => {
                        // 304
                        // use cached body, update headers from parts
                        response.update_headers(&parts);
                        CachedHttpResponse::Hit(cached_response.clone())
                    }
                    AfterResponse::Modified(policy, parts) => {
                        // 200
                        // still have to update headers
                        response.update_headers(&parts);
                        // update body in cache
                        cache.insert(source.to_string(), (policy, response));
                        CachedHttpResponse::Miss(cache.get(&source.to_string()).unwrap().1.clone())
                    }
                }
            }
        }
    } else {
        let response = request_builder.send().await.unwrap();
        let response = HttpResponse::from_reqwest(response).await;
        let cache_policy = CachePolicy::new(&orig_request, &response);
        let response = if cache_policy.is_storable() {
            println!("STORING IN CACHE");
            cache.insert(source.to_string(), (cache_policy, response));
            cache.get(&source.to_string()).unwrap().1.clone()
        } else {
            response
        };
        CachedHttpResponse::Miss(response)
    }
}
