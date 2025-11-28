# Query String to Body

A simple service to convert query string to body.

This service basically:

- Reads **query parameters**:
  - `t` → response `Content-Type` (Defaults to `text/html; charset=utf-8`)
  - `b` → response body (Defaults to empty string)

If `Content-Type` is anything other than `text/*`, `b` should be sent as `url_encode(base64_encode(b))`. It means `b` will be url decoded and then base64 decoded.

The service is hosted in [Leapcell](https://leapcell.io/) services for free.

You can find it here: [https://qs-to-body.leapcell.app/](https://qs-to-body.leapcell.app/)

## Example

[https://qs-to-body.leapcell.app/?b=%3Ch1%3EHello+World!%3C%2Fh1%3E](https://qs-to-body.leapcell.app/?b=%3Ch1%3EHello+World!%3C%2Fh1%3E)

## Limitations

- Unfortunately, to avoid misuse, you are not allowed to have a link to this page in your pages. Either as sourc of a js script or as an image or anything else.
- There is a limitation for the body, but it depends on the Leapcell server or any other server you run the code on.
