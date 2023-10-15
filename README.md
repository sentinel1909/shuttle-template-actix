# Actix-Web Template

A template for an [Actix-Web](https://actix.rs) server application, hosted on [shuttle](https://shuttle.rs)

This app has one pre-built endpoint, at `/health_check`, which returns a 200:OK status code and the JSON formatted message body "Healthy". There is also a "Not Found" endpoint which is registered as a default service. It will respond with HTTP 404 "Not Found" for any non-registered endpoints.

The tracing in this template is opinionated in the sense that it opts out of Shuttle's default global subscriber and leaves you the developer to choose your own path. A few basics are setup such that tracing outputs to stdout.
