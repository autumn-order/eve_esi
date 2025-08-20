# Contributing Guide

Contributions are always welcome, please read through the best practices section below to get an idea of the standards of code in this project and all projects under The Order of Autumn.

For an overview of the project, please refer to the [README](https://github.com/hyziri/eve_esi)

# Project Structure

This crate utilizes the following EVE Online APIs:
- [ESI API Documentation](https://developers.eveonline.com/api-explorer)
- [EVE SSO Documentation](https://developers.eveonline.com/docs/services/sso/)

## Overview

`eve_esi` is primarily divided into `endpoints` for all ESI related endpoints, `model` for ESI & oauth2 related models, and `oauth2` for all OAuth2 related functionality.

## Client

The EsiClient located in `client.rs` is the main client used to interact with the ESI API. It is configured using the `builder.rs` file which allows you to set up OAuth2 settings, base URLs, and other configurations.

## Errors

We use 2 main error enums, EsiError located in `error.rs` for general ESI related errors, and OAuthError located in `oauth2/error.rs` for OAuth2 related errors.

## Configuration

Default URLs and constants used throughout the crate are located in `constant.rs`. This includes base URLs for the ESI API & EVE OAuth2 endpoints as well as default settings for JWT key caching & refreshing used for authentication token validation.

`.env.example` is copied to `.env` and used with the `examples/sso` example to configure the EVE developer application values. It doesn't need to be set for the library itself.

## File Structure

The file structure of the `eve_esi` crate is as follows:

```plaintext
├── lib.rs       (The main entry point for the library)
├── client.rs    (The main client used to interact with the ESI API)
├── builder.rs   (The configuration for the client, including OAuth2 settings)
├── constant.rs  (Default URLs & constants used throughout the crate)
├── error.rs     (The error enum used throughout the crate)
├── esi.rs       (Helper functions for ESI requests)
├── model/       (All models used in the crate & ESI schemas)
│   ├── oauth2.rs    (OAuth2 related models)
│   ├── alliance.rs  (Alliance related models)
│   └── ... additional ESI models
├── oauth2/      (All OAuth2 related functionality)
│   ├── client.rs  (The client used to interact with the EVE OAuth2 API)
│   ├── error.rs   (The error enum for OAuth2 related errors)
│   ├── scope.rs   (The scope builder for creating a list of OAuth2 scopes)
│   ├── login.rs   (Method for initiating the login process)
│   ├── token.rs   (Token fetching & refreshing functionality)
│   └── jwk/       (Functionality for retrieving and caching jwt keys)
└── endpoints/
    ├── alliance.rs  (Alliance related endpoints)
    └── ... additional ESI endpoints
```

# Submitting Issues/Bugs

If you find a bug or issue with the code, please submit an issue to the repository. When submitting an issue, please include the following information:
- A clear and concise description of the issue
- Include how this issue can be reproduced with a code snippet, if applicable.

Be certain to review outstanding issues first to ensure the issue has not already been reported. If it has, please add any additional information of your own to that existing issue as that may help us come to a resolution faster.

FOR SECURITY ISSUES: Please see [SECURITY.md](https://github.com/hyziri/eve_esi/blob/main/SECURITY.md) for instructions. Do not create an issue about a security vulnerability and instead email us directly.

# Submitting Pull Requests

In order to submit a pull request, first you will need to fork the repository and clone it to your local machine. You'll make your commits to this forked repository.

- Prior to submitting the pull request run `cargo test` to ensure all tests pass.
- Additionally, please run `cargo tarpaulin` (`cargo install tarpaulin`) to ensure that the code you are submitting has sufficient test coverage.

When submitting your pull request, please be sure to document the features you have implement, any breaking changes which may require a major version bump, and notes that may be useful for other maintainers to know such as any known issues, bugs, or limitations of the code you are submitting.

Please be aware, any code submitted to this repository must fall under the MIT license.

## Workflow

When submitting a new ESI endpoint, please include the following:

1. ESI endpoint under the `endpoints/` directory named after the section category of the endpoint as stated in the [ESI API Documentation](https://developers.eveonline.com/api-explorer).
2. Any related schemas mirroring the [ESI API Documentation](https://developers.eveonline.com/api-explorer) documentation exactly under the `model/` directory which shares the same name as the section category of the related endpoint.
3. An integration test of the additional endpoint under the `tests/endpoint_category` folder.

The function name will mirror the title of the endpoint documented. For example, the List all alliances endpoint would be named `list_all_alliances` under the `endpoints/alliance.rs` file.

The schema name can be named something more concise as ESI docs use rather verbose naming schemes. For example, the `AlliancesAllianceIdGet` schema could simply be named `Alliance` under the `model/alliance.rs` file.

For example, if you were to add the `list_all_alliances` endpoint, you would create the following files:
- `src/endpoints/alliance.rs` for the endpoint implementation
- `src/model/alliance.rs` for the related schemas
- `tests/alliance/endpoint_name.rs` for the integration test of the endpoint

Don't forget to include:
- If it's a new file, module documentation denoted by `//!` at the top of the file
- Function documentation denoted by `///` above the function
- Inline comments denoted by `//` within the code if there are multiple steps
- Tests, be sure to include documentation denoted by `///` above the test function to explain the test setup and assertions. See `tests/alliance/get_alliance_information` for an example of this.

## Review Process

Upon submitting your pull request, one of the core maintainers of the repository will view your request.
- If there is any documentation, test coverage, or logging which is missing, we're happy to assist you with implementing it.
- Upon acceptance, your pull request will be merged into the `dev` branch and merged into main upon next version bump.

# Best Practices

When writing code for this project, we are meticulous in that we always include:
- Documentation & inline comments
- Logging
- Integration & unit testing

See `src/oauth2/jwt/task.rs` for an example of how these best practices are followed.
See `tests/alliance/get_alliance_information.rs` for an example of how to write integration tests for the endpoints.

Ultimately, these ensure that our code is maintainable, understandable, and reliable. It takes longer to write initially but saves a lot of headaches in the long run trying to walk through each step of a function just to figure out what it does or how it is implemented.

For new programmers, especially to Rust, this can be daunting. You are encouraged to contribute and put your best effort forward on tackling these additional best practices but we'd much prefer progress over perfection. Be certain to ask questions and ask for help if you need a further explanation on how to do something. Once you've done the best you can, submit a pull request and we'll work with you to implement the additional best practices before merging it into a release.

Additionally, you could consider using a tool such as [GitHub CoPilot](https://github.com/features/copilot) with your code editor which can help as a guide in writing documentation, unit tests, and explaining new concepts. You will just want to make absolute certain you double check everything it writes for correctness and consistency with the rest of the codebase.

As of August 17th, 2025, `Claude Sonnet 3.7` is the recommended model to use.

# Development Environment

Development environments for Rust projects is rather straightforward. You simply need a proper code editor & the Rust toolchain installed.

To get started with contributing to the `eve_esi` crate:
- You will need to install [rust](https://www.rust-lang.org/tools/install) then use `rustup default stable` or `rustup default nightly` to set the default toolchain to.
- You will need a code editor which supports `rust-analyzer` such as [Zed](https://zed.dev/) or [Visual Studio Code](https://code.visualstudio.com/).
- To start use `cargo test` to run the tests to ensure everything is working as expected.
