// If the requested path is a complete or partial match, the app will return
// a synthetic redirect response according to some parameters defined in the dictionary.
//   - Keys can have a hostname prefix or * suffix (* must be after /).
//   - "keep_query" field in values indicate whether the query string should be preserved in the responses or not. 
//
// Exmple redirect param definitions:
//   "/test-page-1/": { "status": 301, "keep_query": true, "path": "/destination1" }
//   "www.example.com/foo/*": { "status": 307, "keep_query": true, "path": "/dst1" }
//
use fastly::http::header;
use fastly::http::Url;
use fastly::{Dictionary, Error, Request, Response};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RedirectParams {
    status: u16,
    keep_query: bool,
    path: String,
}

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    let url = req.get_url();
    // Generate a redirect response if a matching redirect entry is found.
    if let Some(p) = lookup_redirects(url) {
        let params = serde_json::from_str::<RedirectParams>(&p).unwrap();

        // Build the location header value.
        let mut location_value = format!(
            "{}://{}{}",
            url.scheme(),
            url.host_str().expect("Host header is present"),
            params.path,
        );
        if params.keep_query && url.query().is_some() {
            location_value.push_str("?");
            location_value.push_str(url.query().expect("Query string is present"));
        }

        // Build a redirect response.
        return Ok(
            Response::from_status(params.status).with_header(header::LOCATION, location_value)
        );
    }

    // Redirect entry not found. sending the request as-is.
    Ok(req.send("origin_0")?)
}

fn lookup_redirects(url: &Url) -> Option<String> {
    let redirects = Dictionary::open("redirects");

    // (1) Look up with host + path
    let mut key = String::new();
    key.push_str(url.host_str()?);
    key.push_str(url.path());
    if let Some(params) = redirects.get(key.as_str()) {
        return Some(params);
    }

    // (2) Look up with path.
    key.clear();
    key.push_str(url.path());
    if let Some(params) = redirects.get(key.as_str()) {
        return Some(params);
    }

    // (3) Look up with host + path + wildcard.
    key.clear();
    key.push_str(url.host_str()?);
    // Add requested path, remove trailing slash if present.
    key.push_str(url.path().trim_end_matches("/"));

    // Perform two rounds of wildcard lookups.
    // One for "(3) host + path + wildcard", another for "(4) path + wildcard".
    for _ in 0..2 {
        // Wildcard lookup is done recursively until all directories have had a chance to match on a wildcard.
        while key.contains('/') {
            key.push_str("/*");
            if let Some(params) = redirects.get(&key) {
                return Some(params);
            }
            // Remove "/*".
            key.truncate(key.len() - 2);
            // Remove the right most path segment.
            if let Some(n) = key.rfind('/') {
                key.truncate(n);
            }
        }
        // (4) Look up with path + wildcard.
        key.clear();
        key.push_str(url.path().trim_end_matches("/"));
    }
    // Redirect not found.
    None
}
