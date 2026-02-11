# Contributing Guide

Contributions are always welcome, please read through the best practices section below to get an idea of the standards of code in this project and all projects under The Order of Autumn.

# Project Structure

This crate utilizes the following EVE Online APIs:
- [ESI API Documentation](https://developers.eveonline.com/api-explorer)
- [EVE SSO Documentation](https://developers.eveonline.com/docs/services/sso/)

## Overview

`eve_esi` is primarily divided into `endpoints` for all ESI related endpoints, `model` for ESI & oauth2 related models, and `oauth2` for all OAuth2 related functionality.

## Client

The Client located in `client.rs` is the main client used to interact with the ESI API. It is configured using the `builder.rs` & `config.rs` files which allow you to set up OAuth2 settings, base URLs, and other configurations.

## Errors

We use 3 main error enums located in the `error/` module: `Error` for general runtime errors, `ConfigError` for configuration errors, and `OAuthError` for OAuth2 related errors.

## Configuration

Default URLs and constants used throughout the crate are located in `constant.rs`. This includes base URLs for the ESI API & EVE OAuth2 endpoints as well as default settings for JWT key caching & refreshing used for authentication token validation.

The defaults can be overridden using the `Config` located in `config.rs`

`.env.example` is copied to `.env` and used with the `examples/sso` example to configure the EVE developer application values. It doesn't need to be set for the library itself.

# Submitting Issues/Bugs

If you find a bug or issue with the code, please submit an issue to the repository. When submitting an issue, please include the following information:
- A clear and concise description of the issue
- Include how this issue can be reproduced with a code snippet, if applicable.

Be certain to review outstanding issues first to ensure the issue has not already been reported. If it has, please add any additional information of your own to that existing issue as that may help us come to a resolution faster.

FOR SECURITY ISSUES: Please see [SECURITY.md](https://github.com/hyziri/eve_esi/blob/main/SECURITY.md) for instructions. Do not create an issue about a security vulnerability and instead email us directly.

# Submitting Pull Requests

When submitting your pull request, please be sure to document the features you have implement, any breaking changes which may require a major version bump, and notes that may be useful for other maintainers to know such as any known issues, bugs, or limitations of the code you are submitting.

Prior to submitting a pull request, check the following:
- `cargo test`: Ensure all tests pass
- `cargo llvm-cov` (`cargo install cargo-llvm-cov): Ensure the changes you make are properly covered by tests (See <https://github.com/taiki-e/cargo-llvm-cov>)
- `cargo doc --open`: Check for any documentation warnings such as broken links and everything is correctly formatted

To submit a pull request, do the following:

1. Fork this repository
2. Clone it to your machine
3. Checkout a new branch for your feature
4. Commit changes
5. Create a pull request on this repository with a summary of your feature & changes

Please be aware, any code submitted to this repository must fall under the MIT license.

## Development Environment

Development environments for Rust projects is rather straightforward. You simply need a proper code editor & the Rust toolchain installed.

To get started with contributing to the `eve_esi` crate:
- You will need to install [rust](https://www.rust-lang.org/tools/install) then use `rustup default stable` or `rustup default nightly` to set the default toolchain to.
- You will need a code editor which supports `rust-analyzer` such as [Zed](https://zed.dev/) or [Visual Studio Code](https://code.visualstudio.com/).
- To start use `cargo test` to run the tests to ensure everything is working as expected.

## Workflow

When submitting a new ESI endpoint, please include the following:

1. ESI endpoint under the `endpoints/` directory named after the section category of the endpoint as stated in the [ESI API Documentation](https://developers.eveonline.com/api-explorer).
2. Any related schemas mirroring the [ESI API Documentation](https://developers.eveonline.com/api-explorer) documentation exactly under the `model/` directory which shares the same name as the section category of the related endpoint.
3. An integration test of the additional endpoint under the `tests/endpoint_category` folder.
4. A unit test if any internal functionality needs to be tested.

The function name will mirror the title of the endpoint documented. For example, the List all alliances endpoint would be named `list_all_alliances` under the `endpoints/alliance.rs` file.

The schema name can be named something more concise as ESI docs use rather verbose naming schemes. For example, the `AlliancesAllianceIdGet` schema could simply be named `Alliance` under the `model/alliance.rs` file.

For example, if you were to add the `list_all_alliances` endpoint, you would create the following files:
- `src/endpoints/alliance.rs` for the endpoint implementation
- `src/model/alliance.rs` for the related schemas
- `tests/alliance/endpoint_name.rs` for the integration test of the endpoint for the public facing methods
- Unit tests in the same file `src/endpoints/alliance.rs` under `mod list_alliances_tests {}` for internal functionality

Don't forget to include:
- If it's a new file, module documentation denoted by `//!` at the top of the file
- Function documentation denoted by `///` above the function
- Inline comments denoted by `//` within the code if there are multiple steps
- Tests, be sure to include documentation denoted by `///` above the test function to explain the test setup and assertions. See `tests/alliance/get_alliance_information` for an example of this.

## Review Process

Upon submitting your pull request, one of the core maintainers of the repository will view your request.
- If there is any documentation, test coverage, or logging which is missing, we're happy to assist you with implementing it.
- Upon acceptance, your pull request will be merged into the `dev` branch and merged into main upon next version bump.
