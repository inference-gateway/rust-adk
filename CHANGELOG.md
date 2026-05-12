# Changelog

All notable changes to this project will be documented in this file.

## [0.4.0](https://github.com/inference-gateway/rust-adk/compare/0.3.1...0.4.0) (2026-05-12)

### ✨ Features

* **protocol:** Implement tasks/resubscribe and agent/getAuthenticatedExtendedCard ([#24](https://github.com/inference-gateway/rust-adk/issues/24)) ([cf36ad1](https://github.com/inference-gateway/rust-adk/commit/cf36ad1fd9a2f4113d5493dc976820f79d852d4d)), closes [#22](https://github.com/inference-gateway/rust-adk/issues/22)
* **server:** Enforce authentication on agent/getAuthenticatedExtendedCard and wire up AuthConfig ([#27](https://github.com/inference-gateway/rust-adk/issues/27)) ([5efa830](https://github.com/inference-gateway/rust-adk/commit/5efa830d8d097346790caa2b1f20cf6bf938e80d)), closes [#25](https://github.com/inference-gateway/rust-adk/issues/25)
* **server:** Wire up TlsConfig for TLS termination and mTLS client certs ([#32](https://github.com/inference-gateway/rust-adk/issues/32)) ([44652b5](https://github.com/inference-gateway/rust-adk/commit/44652b57403ab7b8362e4f4cf01a32b0fe180d0f))

### 📚 Documentation

* **examples:** Add tasks/resubscribe and agent/getAuthenticatedExtendedCard examples ([#26](https://github.com/inference-gateway/rust-adk/issues/26)) ([9d4714c](https://github.com/inference-gateway/rust-adk/commit/9d4714c2654a0c1f941cda774d9390a0a571a0b0)), closes [#23](https://github.com/inference-gateway/rust-adk/issues/23)

## [0.3.1](https://github.com/inference-gateway/rust-adk/compare/0.3.0...0.3.1) (2026-05-12)

### ♻️ Improvements

* **server:** Drop hardcoded sdk_version from health response ([2481af3](https://github.com/inference-gateway/rust-adk/commit/2481af3384fcac1fd5ac06e37b575339c5cfbbcb))

### 🐛 Bug Fixes

* **docs:** Remove emojis to fix links on crates.io ([9ca3508](https://github.com/inference-gateway/rust-adk/commit/9ca350838a8465d3cd134529bcfb21473c6bbb3d))

### 📚 Documentation

* Use better wording instead of just Docker support call it OCI compliant ([e3ed821](https://github.com/inference-gateway/rust-adk/commit/e3ed8212d90439acab46f0afe017674e85f7cba6))

## [0.3.0](https://github.com/inference-gateway/rust-adk/compare/0.2.5...0.3.0) (2026-05-12)

### ✨ Features

* **server:** Implement A2A JSON-RPC method dispatch ([#20](https://github.com/inference-gateway/rust-adk/issues/20)) ([67c5fcf](https://github.com/inference-gateway/rust-adk/commit/67c5fcf76d3d0eb3102b5cfeeee07424626f11af))
* Streaming, queue + Redis storage, and per-method A2A examples ([#21](https://github.com/inference-gateway/rust-adk/issues/21)) ([9b94bbe](https://github.com/inference-gateway/rust-adk/commit/9b94bbeaa660567cff3da6ecfc791eb058deab3d)), closes [#15](https://github.com/inference-gateway/rust-adk/issues/15)

### 👷 CI

* Bump the version of actions/create-github-app-token ([32c9f49](https://github.com/inference-gateway/rust-adk/commit/32c9f492ee7fa5f3f56084793a80585b6ddc4434))

## [0.2.5](https://github.com/inference-gateway/rust-adk/compare/0.2.4...0.2.5) (2026-05-11)

### 👷 CI

* Use crates.io trusted publishing via OIDC ([0a01d6e](https://github.com/inference-gateway/rust-adk/commit/0a01d6eaa2f0aa9499bc6e08567e71d3578fc20f))

## [0.2.4](https://github.com/inference-gateway/rust-adk/compare/0.2.3...0.2.4) (2026-05-11)

### ♻️ Improvements

* Regenerate A2A types from canonical schema ([#18](https://github.com/inference-gateway/rust-adk/issues/18)) ([dd9b569](https://github.com/inference-gateway/rust-adk/commit/dd9b5690f9e64c16b2f459e77ae6426213bfd3d1)), closes [#14](https://github.com/inference-gateway/rust-adk/issues/14)
* Remove outdated copilot-instructions.md file ([89afde5](https://github.com/inference-gateway/rust-adk/commit/89afde521301eed48a66aa92ac73a464d0ce82c0))
* Restructure examples into self-contained server/client scenarios ([#19](https://github.com/inference-gateway/rust-adk/issues/19)) ([95149e6](https://github.com/inference-gateway/rust-adk/commit/95149e654dd71f51dcf30dff198e7818d2e78883)), closes [#18](https://github.com/inference-gateway/rust-adk/issues/18)

### 👷 CI

* Bump actions to their latest ([5111cd0](https://github.com/inference-gateway/rust-adk/commit/5111cd04b85a463b37dd3e61bed9d7ba30f8873d))
* Enable display report for Claude Code action ([6b2e22d](https://github.com/inference-gateway/rust-adk/commit/6b2e22d4ae008b55033b23e9203230e52f6d4e5b))

### 🔧 Miscellaneous

* Remove outdated issue templates for bug reports, feature requests, and refactor requests ([f37489f](https://github.com/inference-gateway/rust-adk/commit/f37489f1cb4992a3a1ce5988ac3f03bcced4308e))
* Upgrade inference-gateway-sdk to 0.13.3 and switch to Flox ([73183e2](https://github.com/inference-gateway/rust-adk/commit/73183e24475df297f565786b3d4808f3a4de6165))

## [0.2.3](https://github.com/inference-gateway/rust-adk/compare/0.2.2...0.2.3) (2026-05-07)

### ♻️ Improvements

* Improve clippy linting by adding collapsible_if allows in config and server files ([a577f43](https://github.com/inference-gateway/rust-adk/commit/a577f439c4b57cd1c7808e40886577a25183d92d))

### 🐛 Bug Fixes

* **ci:** Fix linting errors in test files ([#11](https://github.com/inference-gateway/rust-adk/issues/11)) ([1098987](https://github.com/inference-gateway/rust-adk/commit/109898716feae3607cdc686a0948600436fd2e7f))

### 👷 CI

* Remove claude-code review ([7cecb94](https://github.com/inference-gateway/rust-adk/commit/7cecb941e29d27bf49ea3c5c5584392e2db518ad))
* Update system prompt and allowed tools in Claude workflow ([5cdc50f](https://github.com/inference-gateway/rust-adk/commit/5cdc50f2d288aae012538b33579f0b8a04e5f0c8))

### 🔧 Miscellaneous

* Add issue templates for bug reports, feature requests, and refactor requests ([fb229dd](https://github.com/inference-gateway/rust-adk/commit/fb229dd89423657586386989fb24ce114107b21f))
* Run formatting ([9ce0fb0](https://github.com/inference-gateway/rust-adk/commit/9ce0fb09a28f3eb064a1ef1874c6727a14a96f88))

## [0.2.2](https://github.com/inference-gateway/rust-adk/compare/0.2.1...0.2.2) (2025-07-30)

### 👷 CI

* **workflow:** Update actions/checkout version to v4.2.2 ([16c4a78](https://github.com/inference-gateway/rust-adk/commit/16c4a7888714b5ca7490df56d932a22fb7a372c2))

### 📚 Documentation

* Add contributing guidelines for Rust ADK ([6ca000f](https://github.com/inference-gateway/rust-adk/commit/6ca000fcfca192d15e627ba5f6523597096b3ff9))

## [0.2.1](https://github.com/inference-gateway/rust-adk/compare/0.2.0...0.2.1) (2025-07-30)

### 👷 CI

* **release:** Update GitHub Actions workflow for improved token handling and configuration ([c69747a](https://github.com/inference-gateway/rust-adk/commit/c69747ab2ba3afe4effca48388ac46f06a53e883))

## [0.2.0](https://github.com/inference-gateway/rust-adk/compare/0.1.3...0.2.0) (2025-07-30)

### ✨ Features

* **server:** Add AgentCardOverrides for customizable agent card fields ([6382964](https://github.com/inference-gateway/rust-adk/commit/6382964e721ba240db48fd8b04f54b9fa059e4cb))

### 🐛 Bug Fixes

* **server:** Update agent card loading to accept optional parameters ([4fb5354](https://github.com/inference-gateway/rust-adk/commit/4fb53549e2ded1cc92beabb990d0f7de6e98d7e3))

### ✅ Miscellaneous

* **a2a_server:** Add  validation suite for A2A server functionality ([d9aafbb](https://github.com/inference-gateway/rust-adk/commit/d9aafbb64d723f16e55c8dfd624f08656676ecd9))

## [0.1.3](https://github.com/inference-gateway/rust-adk/compare/0.1.2...0.1.3) (2025-07-30)

### ♻️ Improvements

* **package:** Remove redundant keyword from Cargo.toml ([d38394f](https://github.com/inference-gateway/rust-adk/commit/d38394f7be5aaf2e302881f537edb5fc113e76ed))

## [0.1.2](https://github.com/inference-gateway/rust-adk/compare/0.1.1...0.1.2) (2025-07-30)

### ♻️ Improvements

* **package:** Add metadata fields to Cargo.toml ([d4bbfca](https://github.com/inference-gateway/rust-adk/commit/d4bbfca48ede6bb1f12ebc87970c4a7afdbe3773))

## [0.1.1](https://github.com/inference-gateway/rust-adk/compare/0.1.0...0.1.1) (2025-07-30)

### ♻️ Improvements

* **examples:** Add A2A server example with toolbox functionality ([791a01b](https://github.com/inference-gateway/rust-adk/commit/791a01b85c458e3c399bda9547aa014940c0db18))
