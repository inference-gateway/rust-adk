# Changelog

All notable changes to this project will be documented in this file.

## [0.5.0](https://github.com/inference-gateway/rust-adk/compare/0.4.3...0.5.0) (2026-06-18)

### ✨ Features

* **artifacts:** Add Artifacts Server support with filesystem backend ([#34](https://github.com/inference-gateway/rust-adk/issues/34)) ([2ed5e9d](https://github.com/inference-gateway/rust-adk/commit/2ed5e9d1caae61ea9d101bd4658cba1a962692ba)), closes [#33](https://github.com/inference-gateway/rust-adk/issues/33)
* attach token usage and execution stats to terminal tasks ([#49](https://github.com/inference-gateway/rust-adk/issues/49)) ([7bae730](https://github.com/inference-gateway/rust-adk/commit/7bae730f895c259a8036cd5a56cdd2fcaa86419d)), closes [#44](https://github.com/inference-gateway/rust-adk/issues/44)

### 👷 CI

* add permissions section to CI workflow ([d109707](https://github.com/inference-gateway/rust-adk/commit/d109707e1151df46ec01a3fa9d5671b480ee1577))
* centralize claude.yml via reusable workflow ([#46](https://github.com/inference-gateway/rust-adk/issues/46)) ([d56183a](https://github.com/inference-gateway/rust-adk/commit/d56183a193261144d9be64c7170c199f5ce967ac))
* centralize claude.yml via reusable workflow ([#50](https://github.com/inference-gateway/rust-adk/issues/50)) ([a29f521](https://github.com/inference-gateway/rust-adk/commit/a29f521724874d72b9ef5a4de428c894f1baede9))
* centralize claude.yml via reusable workflow ([#51](https://github.com/inference-gateway/rust-adk/issues/51)) ([1324389](https://github.com/inference-gateway/rust-adk/commit/1324389e96effc26720f15e618eb69fd2ee0cd00))
* centralize claude.yml via reusable workflow ([#68](https://github.com/inference-gateway/rust-adk/issues/68)) ([8a9cbeb](https://github.com/inference-gateway/rust-adk/commit/8a9cbebb00bf8438f80944e399f9eb62bf77d068))
* centralize infer.yml + bump infer CLI and sync .infer config ([#54](https://github.com/inference-gateway/rust-adk/issues/54)) ([09c2e81](https://github.com/inference-gateway/rust-adk/commit/09c2e81988dc1892c10d6b64b0e3c30e800b0f89))
* centralize infer.yml + sync .infer config ([#53](https://github.com/inference-gateway/rust-adk/issues/53)) ([92a5ad7](https://github.com/inference-gateway/rust-adk/commit/92a5ad70267488f0cc098022a3fc105e2ed3ef78))
* centralize infer.yml via reusable workflow ([#52](https://github.com/inference-gateway/rust-adk/issues/52)) ([c891244](https://github.com/inference-gateway/rust-adk/commit/c89124430fe9e755e96749cb7c3aa38096ba15b0))
* **claude:** Add maintainer skill ([ee20052](https://github.com/inference-gateway/rust-adk/commit/ee20052fcb2509382d49911b66dc21c1002c23b0))
* **claude:** change effort to max ([7d9fe3b](https://github.com/inference-gateway/rust-adk/commit/7d9fe3b06e05df23d31e2f7b34acb981c0adb253))
* **claude:** download all maintainer skill assets ([7184c6c](https://github.com/inference-gateway/rust-adk/commit/7184c6cd02369e613399eed9a9afc79e20a5b73d))
* **claude:** remove system prompt - use default community maintained prompt ([c4868b0](https://github.com/inference-gateway/rust-adk/commit/c4868b00f9324218aa62bb4b572ddf5fea8ae20b))
* **claude:** standardize workflow + task-based branch prefix ([df22f98](https://github.com/inference-gateway/rust-adk/commit/df22f989d63f0168dee38903846321da6939051d))
* **deps:** Bump anthropics/claude-code-action  v1.0.131 -> v1.0.133 ([30220c1](https://github.com/inference-gateway/rust-adk/commit/30220c1ee7d5264e7e091eeb4a525805bc049d58))
* **deps:** Update Claude Code Action to version 1.0.131 ([f186def](https://github.com/inference-gateway/rust-adk/commit/f186deffa65dd41e5fd80a7af50f1d66fa792573))
* **deps:** Update claude-code-action to version 1.0.130 ([97d163d](https://github.com/inference-gateway/rust-adk/commit/97d163d600b3c8819635e7d2f941e012a54dcd9f))
* **deps:** upgrade actions/checkout from v6.0.3 to v7.0.0 across workflows ([d2fdd0b](https://github.com/inference-gateway/rust-adk/commit/d2fdd0b1d4ff252d050ccf795248368ad3d65bdf))
* **infer:** centralize infer.yml + bump infer CLI and sync .infer config ([#55](https://github.com/inference-gateway/rust-adk/issues/55)) ([9a07b00](https://github.com/inference-gateway/rust-adk/commit/9a07b00a3fbfd09977f0c33bedafddf1eb23e123))
* Update task installation method in Claude workflow ([d82e33d](https://github.com/inference-gateway/rust-adk/commit/d82e33d4df792cdf9ac5403d9cfab6c8625e5959))

### 📚 Documentation

* Generate AGENTS.md with codex ([42efd28](https://github.com/inference-gateway/rust-adk/commit/42efd281bd28e46168129a05d3f4f8e1beb65e8d))
* Regenerate CLAUDE.md ([9e6dcb3](https://github.com/inference-gateway/rust-adk/commit/9e6dcb35800bacd673aa46f59535e60256a4c808))

### 🔧 Miscellaneous

* **deps:** Add codex and bump infer CLI ([58beffc](https://github.com/inference-gateway/rust-adk/commit/58beffc689cf4af7ae48d1f8b68afdf390559d45))
* **deps:** bump claude-code 2.1.148 -> 2.1.158 ([#57](https://github.com/inference-gateway/rust-adk/issues/57)) ([927f57c](https://github.com/inference-gateway/rust-adk/commit/927f57c62f6ad0b1d01595c9063d68902f950053))
* **deps:** bump claude-code 2.1.158 -> 2.1.161, claude-code-action v1.0.133 -> v1.0.135 ([#65](https://github.com/inference-gateway/rust-adk/issues/65)) ([7c8fad3](https://github.com/inference-gateway/rust-adk/commit/7c8fad3e54b37569ff1a95a0c2509c0059564b1c))
* **deps:** bump claude-code 2.1.161 -> 2.1.170, claude-code-action v1.0.135 -> v1.0.142 ([#70](https://github.com/inference-gateway/rust-adk/issues/70)) ([82161f1](https://github.com/inference-gateway/rust-adk/commit/82161f15c800f7a2ae8400cc3ed58d8246c0b47c))
* **deps:** bump claude-code 2.1.170 -> 2.1.177, claude-code-action v1.0.142 -> v1.0.150 ([#72](https://github.com/inference-gateway/rust-adk/issues/72)) ([8e38e7c](https://github.com/inference-gateway/rust-adk/commit/8e38e7c5392c1f00622ad1351a741fd99e1b1461))
* **deps:** bump codex 0.133.0 -> 0.135.0 ([#61](https://github.com/inference-gateway/rust-adk/issues/61)) ([33f3996](https://github.com/inference-gateway/rust-adk/commit/33f399680b5277dccaf225a316336c86fa336f7f))
* **deps:** Bump dev dependencies ([e91c98d](https://github.com/inference-gateway/rust-adk/commit/e91c98de7f6f6c2c834d118a7dcd15db2d8c7259))
* **deps:** bump infer CLI v0.117.0 -> v0.117.1, infer-action v0.9.1 -> v0.11.1 ([#56](https://github.com/inference-gateway/rust-adk/issues/56)) ([57f79a7](https://github.com/inference-gateway/rust-adk/commit/57f79a77b6203cc37a448bbb0701ea67ae526167))
* **deps:** bump infer CLI v0.117.1 -> v0.119.0, infer-action v0.11.2 -> v0.11.4 ([#62](https://github.com/inference-gateway/rust-adk/issues/62)) ([faaf51d](https://github.com/inference-gateway/rust-adk/commit/faaf51d6769be495fc22abc36a2eefa8e1351435))
* **deps:** bump infer CLI v0.119.0 -> v0.120.0, infer-action v0.11.4 -> v0.11.6 ([#63](https://github.com/inference-gateway/rust-adk/issues/63)) ([b42ef94](https://github.com/inference-gateway/rust-adk/commit/b42ef949603a408980157a502faa52833333e799))
* **deps:** bump infer CLI v0.120.0 -> v0.120.1, infer-action v0.11.6 -> v0.11.7 ([#64](https://github.com/inference-gateway/rust-adk/issues/64)) ([3bc28ab](https://github.com/inference-gateway/rust-adk/commit/3bc28abbf82c2980c823e3cc87fe37de1ca05218))
* **deps:** bump infer CLI v0.120.1 -> v0.121.0 ([#66](https://github.com/inference-gateway/rust-adk/issues/66)) ([4fa71af](https://github.com/inference-gateway/rust-adk/commit/4fa71af7600b945f31cbbc462823cc56dc8dfab7))
* **deps:** bump infer CLI v0.121.0 -> v0.121.1, infer-action v0.12.1 -> v0.13.1 ([#71](https://github.com/inference-gateway/rust-adk/issues/71)) ([6058f98](https://github.com/inference-gateway/rust-adk/commit/6058f980cc73857b0dd831b14ded9cb9a815cded))
* **deps:** bump infer-action v0.11.1 -> v0.11.2 ([#60](https://github.com/inference-gateway/rust-adk/issues/60)) ([8e6ae99](https://github.com/inference-gateway/rust-adk/commit/8e6ae999fd8c5c0d5de01da72cef30d559b68553))
* **deps:** bump infer-action v0.11.7 -> v0.12.1 ([#67](https://github.com/inference-gateway/rust-adk/issues/67)) ([ca41699](https://github.com/inference-gateway/rust-adk/commit/ca41699df23abb0b741b2025170b5b62a9878bbb))
* **deps:** Bump inference-gateway/.github/.github/workflows/claude.yml ([#47](https://github.com/inference-gateway/rust-adk/issues/47)) ([5f6af83](https://github.com/inference-gateway/rust-adk/commit/5f6af8367f0d323425ef434ff65d618a5e799e90))
* **deps:** bump the cargo group with 2 updates ([#69](https://github.com/inference-gateway/rust-adk/issues/69)) ([2d8b1a2](https://github.com/inference-gateway/rust-adk/commit/2d8b1a200441455361e7ec122a665167a2f3153e))
* **deps:** bump the cargo group with 2 updates ([#74](https://github.com/inference-gateway/rust-adk/issues/74)) ([0619744](https://github.com/inference-gateway/rust-adk/commit/0619744d2f7a3c8f3f8b092979055d6204d09476))
* **deps:** bump the cargo group with 3 updates ([#48](https://github.com/inference-gateway/rust-adk/issues/48)) ([c6247a8](https://github.com/inference-gateway/rust-adk/commit/c6247a865c8af3aa0a8e709827475e80d3d117d2))
* **deps:** Bump the cargo-minor-and-patch group with 3 updates ([#45](https://github.com/inference-gateway/rust-adk/issues/45)) ([118ffb9](https://github.com/inference-gateway/rust-adk/commit/118ffb98e1b6ef18e922f1a0460601632bf6df1a))
* **deps:** bump the github-actions group with 2 updates ([#59](https://github.com/inference-gateway/rust-adk/issues/59)) ([8a215b0](https://github.com/inference-gateway/rust-adk/commit/8a215b05731f2f5928c78766d09fd5f0e44b0814))
* **deps:** bump the github-actions group with 2 updates ([#73](https://github.com/inference-gateway/rust-adk/issues/73)) ([9ea8c59](https://github.com/inference-gateway/rust-adk/commit/9ea8c59295a981e0b754b1a886915f396ce6bdf2))
* **deps:** Update claude-code version to 2.1.141 and infer.flake to v0.109.11 ([a296491](https://github.com/inference-gateway/rust-adk/commit/a296491dc4cf1a9fb4c367d737b0538c294f85e5))
* **deps:** update schema version from 1.12.0 to 1.13.0 in manifest.toml ([86cb6ec](https://github.com/inference-gateway/rust-adk/commit/86cb6ec5c99d3fddf2e8cb2213cf4672a7fbaba5))
* **docs:** Generate AGENTS.md file ([c09beb0](https://github.com/inference-gateway/rust-adk/commit/c09beb064ba0852728a14a1917e3e4db1e399f32))
* **docs:** Generate CLAUDE.md file ([d8e68c2](https://github.com/inference-gateway/rust-adk/commit/d8e68c2bd62a6994bc284898e29df4e29102fceb))
* **docs:** Remove AGENTS.md and CLAUDE.md ([984a603](https://github.com/inference-gateway/rust-adk/commit/984a603c106a3fad52b528709974cda74682c101))
* **flox:** add missing manifest.lock file ([e4c4d05](https://github.com/inference-gateway/rust-adk/commit/e4c4d051b6698105102b93ace712d0c77f039342))
* **flox:** Bump schema version ([483b53c](https://github.com/inference-gateway/rust-adk/commit/483b53ce058c077f369dc178b960f02d75874141))
* **license:** Update license to Apache 2.0 ([adfac0a](https://github.com/inference-gateway/rust-adk/commit/adfac0afcaa24e250e81ec292d2d6669c1f39777))
* Regenerate infer CLI configurations ([9fb2962](https://github.com/inference-gateway/rust-adk/commit/9fb29624d5255f029155f3523f3e6cdf8312a223))
* Replace em dahses with regular dashes ([602fc14](https://github.com/inference-gateway/rust-adk/commit/602fc146aa51735a28f97c589891d8d868ce82a5))
* Small fix nit-picking ([a065ef2](https://github.com/inference-gateway/rust-adk/commit/a065ef289ccf109c6487b203b39d4d5500fe38d0))

## [0.4.3](https://github.com/inference-gateway/rust-adk/compare/0.4.2...0.4.3) (2026-05-18)

### ♻️ Improvements

* **docs:** Some cleanups - replace emdashes and remove overwehlming docs ([a1915e9](https://github.com/inference-gateway/rust-adk/commit/a1915e90480bdbcd99dedb219c1cc2dd8f97cc82))

## [0.4.2](https://github.com/inference-gateway/rust-adk/compare/0.4.1...0.4.2) (2026-05-18)

### ♻️ Improvements

* **docs:** Re-word production-ready with enterprise-ready ([8df2150](https://github.com/inference-gateway/rust-adk/commit/8df21505d2b02a583c931c87a81fcb22eac7203c))

### 📚 Documentation

* **readme:** Cleanup - remove table of content entry ([6117976](https://github.com/inference-gateway/rust-adk/commit/6117976c3c0ec3147561b16827fa418f9e05fb13))

## [0.4.1](https://github.com/inference-gateway/rust-adk/compare/0.4.0...0.4.1) (2026-05-18)

### ♻️ Improvements

* Load Config via envy, drop env reads from the library ([#42](https://github.com/inference-gateway/rust-adk/issues/42)) ([43b2b84](https://github.com/inference-gateway/rust-adk/commit/43b2b8479c0c0166f5d4bde8a56dbc02d67a54c2))

### 🐛 Bug Fixes

* **dependabot:** Drop duplicated deps scope from commit prefix ([50f37fb](https://github.com/inference-gateway/rust-adk/commit/50f37fb32c6365808d0a8d2b6073f29d0e7e3720))
* **dependabot:** Remove versioning-strategy ([d8cb4a1](https://github.com/inference-gateway/rust-adk/commit/d8cb4a1b300afd7559b6113d2e602933ce1b4284))

### 👷 CI

* **deps:** Bump anthropics/claude-code-action in the actions-all group ([#43](https://github.com/inference-gateway/rust-adk/issues/43)) ([e73be5e](https://github.com/inference-gateway/rust-adk/commit/e73be5ecaf1ac4950a4235cc54093098ca7b9cc5))
* **deps:** bump the actions-all group across 1 directory with 2 updates ([#35](https://github.com/inference-gateway/rust-adk/issues/35)) ([e13d384](https://github.com/inference-gateway/rust-adk/commit/e13d384ee8df9945573bb479fb209bafd1c64e48))

### 📚 Documentation

* **readme:** Update stale code snippets and env-var docs ([07a042c](https://github.com/inference-gateway/rust-adk/commit/07a042cf428bf7ed32070c324c9feee5fd7d13e2))

### 🔧 Miscellaneous

* Add dependabot config ([ceeaee2](https://github.com/inference-gateway/rust-adk/commit/ceeaee2a95f2d32b1297bf27b1ebd7322a6bbe58))
* Add infer CLI for development ([#41](https://github.com/inference-gateway/rust-adk/issues/41)) ([5bc8d9a](https://github.com/inference-gateway/rust-adk/commit/5bc8d9a54907ee8be1c8e558bdd376c6fef0f34d))
* **deps:** Bump axum-server from 0.7.3 to 0.8.0 ([#39](https://github.com/inference-gateway/rust-adk/issues/39)) ([7487b03](https://github.com/inference-gateway/rust-adk/commit/7487b03340b1e066f7a036deb9166800b5b5f205))
* **deps:** Bump rcgen from 0.13.2 to 0.14.8 ([#40](https://github.com/inference-gateway/rust-adk/issues/40)) ([7ec84a3](https://github.com/inference-gateway/rust-adk/commit/7ec84a369878c19e83b5ce4c9ecad39f3fbd361c))
* **deps:** Bump redis from 0.27.6 to 1.2.1 ([#38](https://github.com/inference-gateway/rust-adk/issues/38)) ([16f37b7](https://github.com/inference-gateway/rust-adk/commit/16f37b7275708cc6460857f863c2841cbdb24338))
* **deps:** bump the cargo-minor-and-patch group across 1 directory with 3 updates ([#36](https://github.com/inference-gateway/rust-adk/issues/36)) ([efc2bc9](https://github.com/inference-gateway/rust-adk/commit/efc2bc96689cc6e328ebd97d36ebda84e29fc19c))
* **deps:** Bump x509-parser from 0.16.0 to 0.18.1 ([#37](https://github.com/inference-gateway/rust-adk/issues/37)) ([deca3e2](https://github.com/inference-gateway/rust-adk/commit/deca3e2fa59d19c0f0d481511970c81265ee6ebc))

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
