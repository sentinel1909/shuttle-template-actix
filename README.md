# Actix-Web API Template

A template for an [Actix-Web](https://actix.rs) server application, which can be hosted on [shuttle](https://shuttle.rs). This template provides a starting point for building out a full Actix Web API.

This app has one pre-built endpoint, at `/api/health_check`, which returns a 200:OK status code and the JSON formatted message body "Healthy". There is also a "Not Found" endpoint which is registered as a default service. It will respond with HTTP 404 "Not Found" for any non-registered endpoints.

CORS is also implemented, with the default settings being permissive.

The tracing in this template is opinionated in the sense that it opts out of Shuttle's default global subscriber and leaves you the developer to choose your own path. A few basics are setup such that tracing outputs to stdout.
