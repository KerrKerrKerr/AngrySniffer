## Authentication Specification

Authentication can be provided via a connection string directly or from a file containing the string. This is used only for the initial connection.

### Connection String

The connection string follows this format:

```
http(s)://<ip>:<port>/<unique_id>/auth/<parameters>
```

-   `unique_id`: An MD5 hash of a passphrase that can be used only once.
-   `port`: The server port. Defaults to `2200`.

### Query Parameters

Parameters are separated by `&`.

-   `keep_alive`: `String`
    -   How long to keep the connection alive in seconds. Use `-1` for infinite.
    -   Useful for temporary access, after which the server will close the connection and reject further requests using the same `unique_id`.
-   `stream`: `bool`
    -   If `true`, streams the progress of the bruteforcing task.
    -   May be disabled for large jobs or for privacy concerns.

## Brute Force Request

To initiate a brute force attack, send a POST request to the following endpoint:

```
http(s)://<ip>:<port>/<unique_id>/brute/
```

**Request Body:**

The body of the request must contain:
-   A `.22000` hashcat hash file.
-   A password list or a ruleset.
-   A timeout value in seconds.
-   

response to auth:
 code 200 auth ok
 code 400 auth failed

response to brute:
 code 200 - started processing
 code 201 - put in queue
 code 400 - bad request (e.g., missing file, invalid parameters)

TODO!
