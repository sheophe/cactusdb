# 🌵 CactusDB

[![build](https://github.com/sheophe/cactusdb/actions/workflows/build.yml/badge.svg)](https://github.com/sheophe/cactusdb/actions/workflows/build.yml)

Fast highly available distributed transactional key-value database and message exchange system for low-bandwidth radio networks

Unlike many other NoSQL databases, CactusDB implements replication and distributed transactions using protocols based on UDP/IP multicast, making it a perfect choice for low-bandwidth wireless networks. In fact, CactusBD allows user to configure the network protocol used for communication between nodes in cluster.

Currently the following protocols are supported:
* PGM, EPGM and NORM (via ZeroMQ)
* DDS
* Aeron

Users connect to CactusDB using WebSockets.
