# NebulaFS

NebulaFS is a distributed, versioned file system built from scratch to explore advanced systems concepts including:

- 🗃️ Raft-based consensus and log replication
- 🧩 Immutable, content-addressed storage (Git/IPFS-inspired)
- 🔐 WASM-based smart modules for file validation and processing
- 🌍 P2P networking for decentralized node discovery and sync
- ⚙️ Edge-capable deployments (Raspberry Pi, cloud, etc.)

---

## 🧱 Core Components and Build Plan

| Module                | Description                                                        | Status                                                                             |
| --------------------- | ------------------------------------------------------------------ | ---------------------------------------------------------------------------------- |
| `nebula-raft`         | Raft consensus engine for node coordination and log replication    | ✅ Completed: Workspace & crate setup<br>🚧 In Progress: Follower & election logic |
| `nebula-core`         | Versioned file storage engine with chunking, diffs, and Merkle DAG | ⏳ Not started                                                                     |
| `nebula-wasm-runtime` | WASM runtime to securely execute user-defined file validators      | ⏳ Not started                                                                     |
| `nebula-p2p`          | Peer-to-peer networking layer for node discovery and gossip sync   | ⏳ Not started                                                                     |
| `nebula-cli`          | CLI tool for interacting with nodes (upload, fetch, validate)      | ⏳ Not started                                                                     |
| `nebula-node`         | Full daemon that composes all modules and runs a NebulaFS node     | 🚧 Initial wiring in progress                                                      |
| `dashboard`           | Web interface for viewing node state, file logs, and WASM output   | ⏳ Optional / Future                                                               |
| `deploy/`             | Scripts and tools for local + edge deployment (Docker, Pi, etc.)   | ⏳ In setup planning                                                               |

---

## 📘 Documentation

All architecture decisions, dev journals, and design notes are in the [**Nebula Docs** repo](https://github.com/otobongfp/nebulaDocs).

---

## 🛠 How to Run (for now)

```bash
cargo run -p nebula-node

```
