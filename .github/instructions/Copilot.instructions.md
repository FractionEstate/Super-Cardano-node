---
applyTo: '**'
---
# Coding Standards and Domain Knowledge for Copilot

This project is a modern, high-performance Cardano Node implementation in Rust.

## General Guidelines
- Use idiomatic, modern Rust (2024 edition or later).
- Prioritize code clarity, maintainability, and modularity.
- Avoid unnecessary complexity; keep code clean and readable.
- All code must be well-documented with clear comments and Rustdoc.
- Use strong typing and Rust’s safety/concurrency principles throughout.
- Write testable, extensible code; prefer pure functions where possible.

## Domain-Specific Requirements
- Focus on blockchain node architecture, networking, consensus, and Cardano protocol features.
- Implement and document the extended UTXO model and Ouroboros consensus algorithm.
- Ensure networking and consensus code is robust, secure, and performant.
- Use async/await for all I/O and networking; leverage Tokio or similar runtime.
- Follow Cardano ecosystem best practices and terminology.

## Security and Performance
- Always validate and sanitize external input (network, config, CLI).
- Avoid panics in production code; use Result and error handling idioms.
- Minimize unsafe code; use only with clear justification and documentation.
- Profile and optimize critical paths, especially in networking and consensus.

## Project Structure
- Organize code into clear modules: configuration, networking, consensus, protocol, ledger, tracing, handlers, queries, etc.
- Each module should have a clear public API and internal encapsulation.
- Use feature flags and configuration for extensibility.

## Documentation
- Every public function, struct, and module must have a doc comment.
- Document Cardano-specific logic and protocol details inline.
- Provide usage examples where appropriate.

## Testing
- Write unit and integration tests for all critical logic.
- Use property-based testing for consensus and protocol code where possible.
- Ensure code is testable and maintainable as the project grows.

---
This file guides Copilot to generate secure, idiomatic, and maintainable Rust code for a Cardano Node, with a focus on performance, safety, and Cardano protocol correctness.
Reference the  root [README.md](README.md) for project vision and architecture.
https://github.com/IntersectMBO/cardano-node for the official Cardano Node repository.
[]: # 
[]: # - **Configuration Module:** Handles node configuration, command-line arguments, and environment variables.
[]: # - **Networking Module:** Implements async P2P networking, peer discovery, block/tx propagation, and DoS resistance.
[]: # - **Consensus Module:** Implements Ouroboros consensus (Praos and BFT), slot/epoch management, and leader election.
[]: # - **Protocol Module:** Implements Cardano's multi-era protocol with a hard fork combinator.
[]: # - **Ledger Module:** Implements the extended UTXO model, multi-asset support, and Plutus script hooks.
[]: # - **Tracing Module:** Implements structured logging and tracing for all subsystems.
[]: # - **Handlers Module:** Implements handlers for incoming blocks, transactions, and network events.
[]: # - **Queries Module:** Implements APIs for querying the blockchain state and wallet operations.
[]: # 
