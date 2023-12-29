# Changelog

## [0.4.0](https://github.com/nvim-neorg/norgopolis-client/compare/v0.3.0...v0.4.0) (2023-12-29)


### Features

* add ability to spawn the norgopolis server behind feature flag ([b9b1ca9](https://github.com/nvim-neorg/norgopolis-client/commit/b9b1ca9f22ffaf823935e88681ae45dae7bcd3be))


### Bug Fixes

* disable `autostart` feature by default as per rust guidelines ([7d58b06](https://github.com/nvim-neorg/norgopolis-client/commit/7d58b063d1d80c11b3575a1dd61f2f64f755c6ca))

## [0.3.0](https://github.com/nvim-neorg/norgopolis-client/compare/v0.2.1...v0.3.0) (2023-09-23)


### Features

* add `invoke_raw_callback` function ([ef17e75](https://github.com/nvim-neorg/norgopolis-client/commit/ef17e7590bfc591132b5b043b9e0809e3ee5e030))

## [0.2.1](https://github.com/nvim-neorg/norgopolis-client/compare/v0.2.0...v0.2.1) (2023-09-07)


### Bug Fixes

* **ci:** install protobuf compiler before executing cargo publish ([33bc405](https://github.com/nvim-neorg/norgopolis-client/commit/33bc4059803be0507cab74bdb0f3a393878793d6))
* **ci:** use `sudo` on apt commands ([41f310d](https://github.com/nvim-neorg/norgopolis-client/commit/41f310d74f7367897a6b2dde90525fb68a65a752))
* update now-outdated `norgopolis-protos` dependency ([c71c87d](https://github.com/nvim-neorg/norgopolis-client/commit/c71c87d6236fb356edeb18519769f560f17633f2))

## 0.2.0 (2023-09-06)


### ⚠ BREAKING CHANGES

* move the client to a separate repository

### Features

* move the client to a separate repository ([ce833f4](https://github.com/nvim-neorg/norgopolis-client/commit/ce833f4e70b7b6a872a82cf041f4cc39331c93c1))


### Bug Fixes

* expose MessagePack as a type ([fd048bf](https://github.com/nvim-neorg/norgopolis-client/commit/fd048bf1536de5161708906fb6702b1f579031be))
* switch to `norgopolis-protos` ([0970e96](https://github.com/nvim-neorg/norgopolis-client/commit/0970e96b5d2b8dd20db13192830b4cc1548460f2))


### Miscellaneous Chores

* release 0.2.0 ([5a0f315](https://github.com/nvim-neorg/norgopolis-client/commit/5a0f315524ae8466b9909dccfe173da428390d40))
