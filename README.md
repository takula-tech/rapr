<div style="text-align: center"><img src="./assets/logo.svg" height="120px">
<h2>Building Performant, Secure and Reliable Distributed System</h2>
</div>

[![Docker Pulls][docker-badge]][docker-url] [![Build Status][actions-badge]][actions-url] [![Test Status][e2e-badge]][e2e-url] [![Code Coverage][codecov-badge]][codecov-url] [![License: Apache 2.0][apache-badge]][apache-url] [![Good First Issues][gfi-badge]][gfi-url] [![discord][discord-badge]][discord-url] [![YouTube][youtube-badge]][youtube-link] [![Bluesky][bluesky-badge]][bluesky-link] [![X/Twitter][x-badge]][x-link]

[docker-badge]: https://img.shields.io/docker/pulls/raprio/raprd?style=flat&logo=docker
[docker-url]: https://hub.docker.com/r/takulatech/rapr
[apache-badge]: https://img.shields.io/github/license/rapr/rapr?style=flat&label=License&logo=github
[apache-url]: https://github.com/takula-tech/rapr/blob/master/LICENSE
[actions-badge]: https://github.com/takula-tech/rapr/workflows/rapr/badge.svg
[actions-url]: https://github.com/takula-tech/rapr/actions?workflow=rapr
[e2e-badge]: https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/rapr-bot/14e974e8fd6c6eab03a2475beb1d547a/raw/rapr-test-badge.json
[e2e-url]: https://github.com/takula-tech/rapr/actions?workflow=rapr-test&event=schedule
[codecov-badge]: https://codecov.io/gh/takula-tech/rapr/branch/master/graph/badge.svg
[codecov-url]: https://codecov.io/gh/takula-tech/rapr
[gfi-badge]:https://img.shields.io/github/issues-search/takula-tech/rapr?query=type%3Aissue%20is%3Aopen%20label%3A%22good%20first%20issue%22&label=Good%20first%20issues&style=flat&logo=github
[gfi-url]:https://github.com/takula-tech/rapr/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22
[discord-badge]: https://img.shields.io/discord/1335036405788971020.svg?label=&logo=discord&logoColor=ffffff&color=7389D8
[discord-url]: https://discord.gg/3jq8js8u
[youtube-badge]:https://img.shields.io/youtube/channel/views/xxx?style=flat&label=YouTube%20views&logo=youtube
[youtube-link]:https://youtube.com/@takulatechdev
[bluesky-badge]:https://img.shields.io/badge/Follow-%40takulatechdev.bsky.social-0056A1?logo=bluesky
[bluesky-link]:https://bsky.app/profile/takulatechdev.bsky.social
[x-badge]:https://img.shields.io/twitter/follow/takulatechdev?logo=x&style=flat
[x-link]:https://twitter.com/takulatechdev
[Dapr]:https://github.com/dapr/dapr

Rapr is inspired by [Dapr] and aims to provide a set of integrated APIs with built-in best practices and patterns to build distributed applications. Rapr increases your developer productivity with out-of-the-box features such as workflow, pub/sub, state management, secret stores, external configuration, bindings, actors, distributed lock, and cryptography.  

You benefit from the built-in security, reliability, and observability capabilities, so you don't need to write boilerplate code to achieve production-ready applications.

Compared with [Dapr], Rapr has the following advantages:

- **New Clustering Build Block**
  Gossip-based cluster formation, node membership and failure detection

- **Enhanced Performance**
  - Rapr is built with performance in mind with `RUST` language, providing low-latency and high-throughput capabilities for distributed applications.
  - Rapr uses `DPKT` to provide ultra-low network latency and super-high throughput.

- **Enhanced Pubsub Build Block**
  Rapr extends the pub/sub capabilities of [Dapr] with not only intra-cluster service but also support large scale of subscribers outside of cluster

- **Enhanced Actor Build Block**
  Rapr extends the actor capabilities of [Dapr] with new features:
  - `Actor Placement`:
    - a host where a specific actor locates
    - a host with least actors
    - a random host
  - `Actor Migration`: User can trigger migration programmatically from codes. For latency-sensitive applications, user can trigger live migration without dropping requests or losing in-memory state, requiring no read/write operations of database.
  - `Rebalance with custom strategy`: rebalance the actors when new host joins or leaves the cluster (shutdown, crash or upgrade) based on predefined or use-defined strategy.
  - `Transaction`: allows multiple operations to ve executed as a whole across many actors.
