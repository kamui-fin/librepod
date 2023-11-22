// HTTP caching (RFC 7234) to speed up fetching RSS feed results with Redis

use derivative::Derivative;
use http::{request, response};
use http_cache_semantics::{AfterResponse, BeforeRequest, CachePolicy, ResponseLike};

use reqwest::{RequestBuilder, Response};
use serde::{Deserialize, Serialize};

use std::time::SystemTime;
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
struct RedisCacheItem {
    cached_response: HttpResponse,
    policy: CachePolicy,
}

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
    con: &mut redis::aio::ConnectionManager,
    source: &str,
) -> CachedHttpResponse {
    let orig_request = request_builder.try_clone().unwrap().build().unwrap();

    // try to pull from cache if possible
    if let Ok(prev_cached_item_json) = redis::cmd("GET")
        .arg(source)
        .query_async::<_, String>(con)
        .await
    {
        let RedisCacheItem {
            policy,
            mut cached_response,
        } = serde_json::from_str(&prev_cached_item_json).unwrap();

        match policy.before_request(&orig_request, SystemTime::now()) {
            BeforeRequest::Fresh(parts) => {
                cached_response.update_headers(&parts);
                CachedHttpResponse::Hit(cached_response.clone())
            }
            BeforeRequest::Stale {
                request,
                matches: _,
            } => {
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
                        let cache_item = RedisCacheItem {
                            policy,
                            cached_response: response,
                        };
                        let cache_item_json = serde_json::to_string(&cache_item).unwrap();
                        let _result = redis::cmd("SET")
                            .arg(source)
                            .arg(cache_item_json)
                            .query_async::<_, ()>(con)
                            .await;
                        CachedHttpResponse::Miss(cache_item.cached_response)
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

            let cache_item = RedisCacheItem {
                policy: cache_policy,
                cached_response: response,
            };
            let cache_item_json = serde_json::to_string(&cache_item).unwrap();
            let _result = redis::cmd("SET")
                .arg(source)
                .arg(cache_item_json)
                .query_async::<_, ()>(con)
                .await;

            cache_item.cached_response
        } else {
            response
        };
        CachedHttpResponse::Miss(response)
    }
}
