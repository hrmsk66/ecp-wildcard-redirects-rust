# Wildcard redirects at the edge (with Rust SDK)

This example assumes that redirection parameters are defined in the dictionary in the format like below.
- Keys can have a hostname prefix or * suffix (* must be after /).
- "keep_query" field in values indicate whether the query string should be preserved in redirect responses or not. 

```
{
  "/test-page-1/": {
    "status": 301,
    "keep_query": true,
    "path": "/destination1"
  },
  "/city/boston/*": {
    "status": 302,
    "keep_query": false,
    "path": "/boston-massachusetts"
  },
  "www.example.com/foo/*": {
    "status": 307,
    "keep_query": true,
    "path": "/dst1"
  }
}
```
