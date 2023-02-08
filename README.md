# 🌵 CactusDB

[![build](https://github.com/sheophe/cactusdb/actions/workflows/build.yml/badge.svg)](https://github.com/sheophe/cactusdb/actions/workflows/build.yml)

Fast highly available distributed transactional key-value database and message exchange system for low-bandwidth radio networks.

Unlike many other NoSQL databases, CactusDB implements replication and distributed transactions using network protocols based on UDP/IP multicast, making it a perfect choice for low-bandwidth wireless networks.

Users connect to CactusDB using WebSockets, HTTP or gRPC interfaces. WebSockets interface provides pub-sub functionatity, while HTTP and gRPC interfaces are built on a more classical set-get paradigm. Because of this interface flexibility CactusDB can be configured to function purely as a message pub-sub system (user can configure database to store all the data in RAM instead of disk), or as a reliable brokerless message queue (if database is configured to persist on disk), or as a full-fledged key-value database (with HTTP interface for GET and PUT operations).

## Status
This project is currently in the earliest stages of development and does not actually work. This status message will be updated regularely until the first release. Afterwards, release changes will be described in changelogs.

## Acknowledgements

* Thanks [TiKV](https://github.com/tikv/tikv) for inspiring a large portion of the database-related code (transactions, caching, snapshots, etc)
* Thanks [RocksDB](https://github.com/facebook/rocksdb) for the powerful persisten key-value store
* Thanks [Aeron](https://github.com/real-logic/aeron) for the efficient reliable UDP multicast transport
