# Query String to Body

A simple service to convert query string to body.

This service basically:

- Reads **query parameters**:
  - `t` → response `Content-Type` (Defaults to `text/html; charset=utf-8`)
  - `b` → response body (Defaults to empty string)

The service is hosted in [Leapcell](https://leapcell.io/) services for free.

You can find it here: https://qs-to-body.leapcell.app/

## Example

[https://qs-to-body.leapcell.app/?b=%3Ch1%3EHello+World!%3C%2Fh1%3E](https://qs-to-body.leapcell.app/?b=%3Ch1%3EHello+World!%3C%2Fh1%3E)
