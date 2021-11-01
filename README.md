# Wildcard redirects at the edge (with Rust SDK)

If the requested path is a complete or partial match, the app will return a synthetic redirect response according to some parameters defined in the dictionary.
  - Keys can have a hostname prefix or * suffix (* must be after /).
  - "keep_query" field in values indicate whether the query string should be preserved in the responses or not. 

Exmple redirect param definitions:

```
"/test-page-1/": { "status": 301, "keep_query": true, "path": "/destination1" }
"www.example.com/foo/*": { "status": 307, "keep_query": true, "path": "/dst1" }
```
