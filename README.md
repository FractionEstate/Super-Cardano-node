# 🚀 Super Cardano Node

**A next-generation, high-performance Cardano node written in idiomatic Rust (2024 edition).  
Built for clarity, modularity, and protocol excellence.**

---

## 🌟 Vision

Super Cardano Node is more than a blockchain node—it's a showcase of modern Rust engineering, protocol rigor, and extensibility.  
Our mission: **deliver a robust, secure, and developer-friendly Cardano node that sets new standards for clarity, safety, and performance.**

---

## 🏗️ Architecture at a Glance

- **Idiomatic Rust:** Leveraging Rust's safety, concurrency, and expressive type system.
- **Strict Modularity:** Each subsystem—configuration, networking, consensus, protocol, ledger, tracing, handlers, queries, wallet—is a clear, documented module.
- **Async/Await Everywhere:** All I/O and networking is async, powered by Tokio for massive scalability.
- **Multi-Era Protocol:** Supports Byron, Shelley, Allegra, Mary, Alonzo, Babbage, and Conway eras, with a hard fork combinator for seamless upgrades.
- **Ouroboros Consensus:** Implements Praos and BFT, with extensible hooks for future consensus research.
- **Extended UTXO Model:** Full support for Cardano's EUTXO, multi-asset, and Plutus script hooks.
- **Persistent ChainDB:** On-disk, async, rollback-capable database for blocks, UTXOs, and state.
- **Modern APIs:** Exposes both REST (Axum) and gRPC (Tonic) APIs for all node, chain, and wallet operations.
- **Integrated Wallet:** Key management, address derivation, UTXO selection, transaction construction/signing, and API endpoints.
- **Comprehensive Testing:** Unit, integration, and property-based tests for all critical logic.
- **First-Class Documentation:** Every public API and protocol detail is documented with Rustdoc and inline comments.

---

## 🗂️ Cardano Eras Overview

| Era     | Description                                                                 | Main Features Introduced                                  |
|---------|-----------------------------------------------------------------------------|-----------------------------------------------------------|
| Byron   | Cardano's original bootstrap era                                            | Classic UTXO, Ouroboros Classic, basic transactions       |
| Shelley | Decentralization, staking, and delegation                                  | Ouroboros Praos, staking, delegation, rewards             |
| Allegra | Timelocks and multi-asset groundwork                                       | Timelock scripts, multi-asset groundwork                  |
| Mary    | Native multi-asset support                                                 | Multi-asset ledger, user-defined tokens                   |
| Alonzo  | Smart contracts via Plutus                                                 | Plutus scripts, smart contracts, script validation        |
| Babbage | Plutus V2, reference scripts, inline datums, reference inputs              | Improved smart contracts, new ledger features             |
| Conway  | On-chain governance, advanced voting, protocol upgrades                    | Voltaire governance, DRep voting, protocol upgrades       |

---

## 🔥 Why Super Cardano Node?

- **Modern Rust Codebase:** Clean, idiomatic, and future-proof.
- **Protocol Correctness:** Implements Cardano’s extended UTXO and Ouroboros consensus with precision.
- **Performance:** Async I/O, lock-free design, and profiling-ready for critical paths.
- **Security:** Input validation, error handling, and minimal unsafe code.
- **Extensibility:** Feature flags, configuration, and modular APIs for rapid evolution.
- **Developer Experience:** Clear APIs, exhaustive docs, and a welcoming codebase.

---

## 🚦 What’s Implemented

- **Full Modular Architecture:** Each subsystem is a clear, documented module.
- **Ouroboros Consensus:** Async/await-based Praos and BFT, slot/epoch management, leader election.
- **Multi-Era Protocol:** Byron, Shelley, Allegra, Mary, Alonzo, Babbage, Conway (with hard fork combinator).
- **EUTXO Model:** Multi-asset, Plutus script hooks, and extensible transaction types.
- **Networking:** Robust async P2P, peer discovery, block/tx propagation, DoS resistance.
- **ChainDB:** On-disk, async, rollback-capable database for blocks, UTXOs, and state.
- **REST & gRPC APIs:** Modern, async APIs for all node, chain, and wallet operations.
- **Wallet Module:** Key management, address derivation, UTXO selection, transaction construction/signing.
- **Testing:** Unit, integration, and property-based tests for all critical logic.
- **Documentation:** All public APIs and Cardano-specific logic are documented.
- **Security & Performance:** Input validation, error handling, and profiling-ready.

---

## 🔍 Technical Implementation Details

### Core Architecture

The Super Cardano Node is structured around a **multi-crate** approach within a single repository, leveraging Rust's **Cargo** for seamless integration and management. The architecture is divided into several key components:

- **`core`:** Contains the fundamental building blocks of the node, including the extended UTXO model, transaction processing, and the Ouroboros consensus algorithm.
- **`network`:** Manages all aspects of peer-to-peer networking, including connection handling, message passing, and network discovery.
- **`chain`:** Implements the ChainDB for block and state storage, utilizing an append-only log structure for efficient and secure data management.
- **`wallet`:** Provides wallet functionality, including key management, address derivation, and transaction creation and signing.
- **`api`:** Exposes REST and gRPC APIs for external interaction with the node, allowing for comprehensive control and monitoring.
- **`cli`:** A command-line interface for node management, providing commands for all major functionalities.

### Detailed Component Overview

- **Idiomatic Rust:** The codebase is written in **Rust 2024 edition**, adhering to the latest language features and best practices. This ensures maximum performance, safety, and concurrency.
- **Strict Modularity:** Each component is developed as a separate module with clear boundaries and responsibilities, facilitating easier maintenance and evolution.
- **Async/Await Everywhere:** The entire I/O stack is asynchronous, using Rust's `async`/`await` syntax for non-blocking operations, backed by the **Tokio** runtime.
- **Multi-Era Protocol Support:** The node natively understands and can switch between different Cardano eras (Byron, Shelley, Allegra, Mary, Alonzo, Babbage, Conway), with a hard fork combinator seamlessly integrating changes.
- **Ouroboros Consensus:** Implements the **Praos** and **BFT** variants of the Ouroboros consensus algorithm, with hooks for future extensibility.
- **Extended UTXO Model:** Full support for Cardano's EUTXO model, including multi-asset support and Plutus script integration.
- **Persistent ChainDB:** Utilizes an on-disk, asynchronous database for storing blockchain data, with support for rollbacks and fast sync.
- **Modern APIs:** Provides both RESTful and gRPC APIs, designed with **Axum** and **Tonic**, respectively, for modern, efficient web services.
- **Integrated Wallet:** A complete wallet solution is integrated into the node, supporting all wallet-related operations and API endpoints.
- **Comprehensive Testing:** The project includes an extensive suite of tests, using Rust's built-in test framework, covering unit, integration, and property-based tests.
- **First-Class Documentation:** Documentation is generated using Rustdoc, with an emphasis on clarity and completeness, ensuring all public APIs and complex logic are well explained.

---

## 🧭 Roadmap

1. **Full Era Support:**  
   Implement Babbage/Conway-era governance, on-chain voting, and advanced hard fork logic.
2. **Plutus Interpreter:**  
   Integrate a full Plutus script interpreter for Alonzo and later eras.
3. **Advanced Stake Pool Operations:**  
   Add full pool metadata, relay management, and reward calculation as per the Cardano specification.
4. **Network Topology Management:**  
   Implement advanced peer selection, topology, and network resilience features.
5. **CLI/Operational Tooling:**  
   Expand the CLI for node management, diagnostics, and operational tasks.
6. **Ledger Snapshots/Replay:**  
   Add advanced snapshotting, replay, and fast sync features.
7. **Governance/On-chain Upgrades:**  
   Complete Conway-era governance and on-chain upgrade mechanisms.
8. **Formal Verification:**  
   Integrate property-based and formal testing at the specification level.

---

## 📚 Documentation & Standards

- **Every public item is documented.**
- **Cardano-specific logic and protocol details are explained inline.**
- **See [`Copilot.instructions.md`](.github/instructions/Copilot.instructions.md) for coding standards and domain knowledge.**

---

## 🧪 Testing

- **Unit and integration tests** for all critical logic.
- **Property-based testing** for consensus and protocol code.
- **Run all tests:**  
  ```sh
  cargo test
  ```

---

## 🤝 Contributing

We welcome contributions from the Cardano and Rust communities!

- **Use idiomatic, modern Rust.**
- **Document all public APIs and protocol logic.**
- **Write unit, integration, and property-based tests for all new features.**
- **Validate and sanitize all external input.**
- **Avoid panics in production code.**
- **Read our [coding standards](.github/instructions/Copilot.instructions.md) before submitting PRs.**

---

## ⚖️ License

MIT or Apache 2.0 (choose the one appropriate for your project).

---

## 💬 Get Involved

- **Questions?** Open an issue or join the discussion.
- **Ideas?** File a feature request or start a conversation.
- **Found a bug?** Please report it with a minimal reproduction.

---

## 🚀 Let’s build the future of Cardano together, in Rust.

---