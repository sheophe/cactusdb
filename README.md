# 🌵 CactusDB

[![build](https://github.com/sheophe/cactusdb/actions/workflows/build.yml/badge.svg)](https://github.com/sheophe/cactusdb/actions/workflows/build.yml)

Fast highly available distributed transactional key-value database and message exchange system for low-bandwidth radio networks

Unlike many other NoSQL databases, CactusDB implements replication and distributed transactions using protocols based on UDP/IP multicast, making it a perfect choice for low-bandwidth wireless networks. In fact, CactusBD allows user to configure the network protocol used for communication between nodes in cluster.

Currently the following protocols are supported:
* PGM, EPGM and NORM (via ZeroMQ)
* DDS
* Aeron

Users connect to CactusDB using WebSockets or HTTP interfaces. WebSockets interface provides pub-sub functionatity, while HTTP interface is a more classical get-put interface. Because of this interface flexibility CactusDB can be configured to function purely as a message pub-sub system (user can configure database to store all the data in RAM instead of disk), or as a reliable brokerless message queue (if database is configured to persist on disk), or as a full-fledged key-value database (with HTTP interface for GET and PUT operations).
