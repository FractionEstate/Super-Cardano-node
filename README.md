# Super Cardano Node

A modern, high-performance Cardano node implemented in idiomatic Rust (2024 edition), following strict modularity, documentation, and testing standards.

---

## Project Overview

Super Cardano Node is designed for clarity, maintainability, and extensibility. It implements the core Cardano protocol, extended UTXO model, Ouroboros consensus, robust async networking, and a persistent on-disk chain database. The node exposes both REST and gRPC APIs and includes a built-in wallet module.

---

## What’s Implemented

- **Modular Architecture:** Clear separation of configuration, networking, consensus, protocol, ledger, tracing, handlers, queries, and wallet modules.
- **Ouroboros Consensus:** Async/await-based implementation of the Ouroboros consensus algorithm, including slot/epoch management and leader election.
- **Extended UTXO Model:** Full support for Cardano’s EUTXO model, including multi-asset and Plutus script hooks.
- **Networking:** Robust, async P2P networking with peer discovery, block/tx propagation, and DoS resistance.
- **ChainDB:** On-disk, async, rollback-capable chain database for blocks, UTXOs, and state.
- **REST & gRPC APIs:** Modern, async APIs for block, UTXO, state, and wallet operations.
- **Wallet Module:** Key management, address derivation, UTXO selection, transaction construction/signing, and API endpoints.
- **Testing:** Comprehensive unit, integration, and property-based tests for all critical logic.
- **Documentation:** All public APIs and Cardano-specific logic are documented with Rustdoc and inline comments.
- **Security:** Input validation, error handling, and minimal unsafe code.
- **Performance:** Async I/O throughout, profiling-ready, and optimized for critical paths.

---

## What’s Missing Compared to a Traditional (Haskell) Cardano Node

- **Full Era Support:** While Byron, Shelley, Allegra, Mary, and Alonzo features are present, advanced Babbage/Conway-era governance and on-chain voting may be incomplete.
- **Plutus Interpreter:** Plutus script validation is stubbed or simplified; a full Plutus interpreter is not yet integrated.
- **Advanced Stake Pool Operations:** Some advanced pool metadata, relay management, and reward calculation nuances may be simplified.
- **Network Topology Management:** Advanced peer selection/topology (as in the Haskell node) may be less feature-rich.
- **CLI/Operational Tooling:** The CLI may not be as extensive as the Haskell node’s operational suite.
- **Ledger Snapshots/Replay:** Advanced snapshotting, replay, and fast sync features may be basic.
- **Governance/On-chain Upgrades:** Full Conway-era governance and DRep logic may be pending.
- **Formal Verification:** The Haskell node benefits from formal methods and property-based testing at the specification level.

---

## What’s Extra or Improved

- **Modern Rust Codebase:** Leverages Rust’s safety, concurrency, and performance features.
- **Async/Await Everywhere:** All I/O and networking is async, leveraging Tokio for scalability.
- **gRPC API:** In addition to REST, a full gRPC API is available for high-performance integrations.
- **Extensible Wallet Module:** Built-in wallet management with async APIs, not present in the Haskell node.
- **Clear Modularization:** Each subsystem is a clear, documented module with public APIs.
- **Idiomatic Error Handling:** Uses Rust’s Result and error types throughout, avoiding panics.
- **Property-Based Testing:** Modern property-based and integration tests for consensus and protocol.
- **Strong Typing:** Extensive use of Rust’s type system for safety and clarity.
- **Documentation:** Every public item is documented, with usage examples and inline protocol details.
- **Extensibility:** Feature flags and configuration for protocol eras and optional features.

---

## Roadmap for Full Parity

1. **Full Era Support**
   - Implement Babbage/Conway-era features: on-chain governance, DRep logic, and hard fork combinator logic.
2. **Plutus Interpreter**
   - Integrate a full Plutus script interpreter for Alonzo and later eras.
3. **Advanced Stake Pool Operations**
   - Add full pool metadata, relay management, and reward calculation as per the Cardano specification.
4. **Network Topology Management**
   - Implement advanced peer selection, topology, and network resilience features.
5. **CLI/Operational Tooling**
   - Expand the CLI for node management, diagnostics, and operational tasks.
6. **Ledger Snapshots/Replay**
   - Add advanced snapshotting, replay, and fast sync features.
7. **Governance/On-chain Upgrades**
   - Complete Conway-era governance and on-chain upgrade mechanisms.
8. **Formal Verification**
   - Integrate property-based and formal testing at the specification level.

---

## Coding Standards

This project follows strict standards for code clarity, safety, modularity, and documentation. See [`.github/instructions/Copilot.instructions.md`](.github/instructions/Copilot.instructions.md) for full details.

---

## Contributing

- Use idiomatic, modern Rust.
- Document all public APIs and protocol logic.
- Write unit, integration, and property-based tests for all new features.
- Validate and sanitize all external input.
- Avoid panics in production code.

---

## License

MIT or Apache 2.0 (choose one appropriate for your project).

---