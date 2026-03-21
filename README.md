# Solana Blockchain Parser

A Rust CLI tool that queries the Solana mainnet blockchain via JSON-RPC and parses on-chain data for a given wallet address.

## What it does

- Fetches account info for a wallet (balance, owner program, executable status)
- Fetches current epoch info (slot, epoch number, progress)
- Fetches recent network performance samples (TPS)
- Fetches transaction signatures for a wallet address
- Fetches and parses the most recent transaction for that wallet

## Project structure

```
src/
├── main.rs                     - Entry point, calls all endpoints
├── blockchain_info.rs          - All RPC request functions (generic send_request)
├── account_info.rs             - Structs for getAccountInfo response
├── epoch_info.rs               - Structs for getEpochInfo response
├── recent_performance_samples.rs - Structs for getRecentPerformanceSamples response
├── signatures_for_address.rs   - Structs for getSignaturesForAddress response
└── transaction.rs              - Structs for getTransaction response
```

## Setup

1. Create a `.env` file in the project root:
```
WALLET=your_solana_wallet_address
API_KEY=your_rpc_api_key
```

2. Run:
```
cargo run
```

## Key concepts

- **JSON-RPC** — all Solana data is fetched by sending POST requests with a `method` and `params` to the RPC node
- **Generic `send_request<T>`** — one function handles all requests; the caller decides what type to deserialize into
- **Shared `reqwest::Client`** — created once in `main` and passed by reference to reuse the connection pool
- **`Option<T>`** — used throughout because Solana returns `null` for many fields on unconfirmed or empty accounts

## About the Developer

Jillian is a Rust and Solana developer with a background in networking and network security, including firewalls and infrastructure. She is building on-chain tooling to deepen her understanding of the Solana blockchain at a systems level — combining low-level network knowledge with modern async Rust to parse and interact with live blockchain data.

## Dependencies

- `reqwest` — async HTTP client
- `tokio` — async runtime
- `serde` / `serde_json` — JSON serialization and deserialization
- `dotenv` — loads environment variables from `.env`
