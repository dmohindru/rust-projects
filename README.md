# Rust Learning Journey - 10 Professional Projects

A progressive roadmap of Rust projects designed to build expertise from fundamentals to advanced systems programming. Each project increases in complexity while reinforcing core Rust concepts through practical, professional-grade applications.

## Learning Philosophy

This roadmap follows a hands-on approach to learning Rust, moving from reading about concepts to implementing them in real-world scenarios. Each project is designed to:

- Build upon previous concepts while introducing new ones
- Mirror professional development practices
- Include comprehensive testing (TDD approach)
- Use external libraries and modern Rust ecosystem
- Provide 2-4 weeks of engaging development work

**Prerequisites:**

- Completed "The Rust Programming Language" book
- Experience with backend/frontend development in other languages
- Familiarity with concepts like HTTP, databases, and concurrent programming

---

## Project 1: Command-Line Task Manager with SQLite

**Core Concepts:** Basic syntax, error handling, SQLite integration

Build a CLI task management tool that persists data to SQLite. Users can add, list, update, and delete tasks with priorities and due dates.

**Key Libraries:**

- `clap` for CLI argument parsing
- `rusqlite` for SQLite database operations
- `chrono` for date/time handling
- `serde` for serialization

**TDD Focus:**

- Unit tests for task CRUD operations
- Integration tests for CLI commands
- Mock database layer for testing business logic

**Learning Outcomes:**

- Rust project structure and Cargo
- Error handling with `Result<T, E>`
- Basic database operations
- Command-line interface design

---

## Project 2: HTTP REST API for Expense Tracking

**Duration:** 2 weeks  
**Core Concepts:** HTTP servers, JSON handling, async programming basics

Create a REST API for tracking personal expenses with categories, amounts, and dates. Include endpoints for CRUD operations and basic reporting.

**Key Libraries:**

- `axum` or `warp` for HTTP server
- `tokio` for async runtime
- `serde_json` for JSON serialization
- `sqlx` for async database operations
- `uuid` for unique identifiers

**TDD Focus:**

- HTTP endpoint testing with test clients
- Mock database repositories
- JSON serialization/deserialization tests
- Error response validation

**Learning Outcomes:**

- Async/await programming model
- HTTP request/response handling
- JSON APIs in Rust
- Basic project architecture patterns

---

## Project 3: Multi-threaded Web Scraper with Rate Limiting

**Duration:** 2-3 weeks  
**Core Concepts:** Concurrency, thread pools, network programming

Build a configurable web scraper that can crawl multiple websites concurrently while respecting rate limits and robots.txt files.

**Key Libraries:**

- `reqwest` for HTTP client
- `tokio` for async concurrency
- `scraper` for HTML parsing
- `governor` for rate limiting
- `tracing` for structured logging

**TDD Focus:**

- Mock HTTP responses for testing
- Concurrency testing with deterministic scenarios
- Rate limiting verification
- Parser unit tests with sample HTML

**Learning Outcomes:**

- Advanced concurrency patterns
- HTTP client programming
- HTML parsing and data extraction
- Rate limiting and ethical scraping

---

## Project 4: Real-time Chat Server with WebSockets

**Duration:** 2-3 weeks  
**Core Concepts:** WebSockets, message broadcasting, connection management

Develop a chat server supporting multiple rooms, user authentication, and message persistence. Include features like typing indicators and user presence.

**Key Libraries:**

- `axum` with WebSocket support
- `tokio-tungstenite` for WebSocket handling
- `dashmap` for concurrent collections
- `jwt` for authentication tokens
- `redis` for session storage (or in-memory alternative)

**TDD Focus:**

- WebSocket connection lifecycle tests
- Message broadcasting verification
- Authentication flow testing
- Concurrent user session management

**Learning Outcomes:**

- Real-time communication protocols
- Connection state management
- Authentication and authorization
- Broadcasting patterns in Rust

---

## Project 5: File Processing Pipeline with Work Queues

**Duration:** 2-3 weeks  
**Core Concepts:** Channel-based communication, worker pools, file I/O

Create a system that processes uploaded files (CSV, JSON, images) through configurable transformation pipelines using worker threads and job queues.

**Key Libraries:**

- `crossbeam` for channels and concurrency
- `rayon` for data parallelism
- `csv` and `serde` for data processing
- `image` for image manipulation
- `thiserror` for custom error types

**TDD Focus:**

- Pipeline stage unit tests
- Worker pool behavior verification
- File processing integration tests
- Error propagation and recovery testing

**Learning Outcomes:**

- Channel-based architecture
- Worker pool patterns
- File I/O and processing
- Error handling in concurrent systems

---

## Project 6: Distributed Cache System with Network Protocol

**Duration:** 3 weeks  
**Core Concepts:** Custom network protocols, serialization, distributed systems basics

Build a distributed key-value cache similar to Redis, with custom binary protocol, persistence, and basic clustering support.

**Key Libraries:**

- `tokio` for async networking
- `bincode` for binary serialization
- `dashmap` for concurrent hash maps
- `clap` for configuration
- `tracing` for distributed logging

**TDD Focus:**

- Protocol serialization/deserialization tests
- Network message handling verification
- Cache consistency testing
- Cluster node communication mocking

**Learning Outcomes:**

- Custom network protocol design
- Binary serialization strategies
- Distributed system challenges
- Performance optimization techniques

---

## Project 7: Microservices API Gateway with Load Balancing

**Duration:** 3 weeks  
**Core Concepts:** Service discovery, load balancing, middleware chains

Develop an API gateway that routes requests to backend services with features like authentication, rate limiting, request/response transformation, and health checking.

**Key Libraries:**

- `hyper` for low-level HTTP handling
- `tower` for middleware composition
- `consul` or `etcd` client for service discovery
- `metrics` for observability
- `config` for configuration management

**TDD Focus:**

- Load balancing algorithm tests
- Middleware chain composition verification
- Service health checking mocks
- End-to-end routing integration tests

**Learning Outcomes:**

- Middleware patterns and composition
- Service discovery patterns
- Load balancing strategies
- Production-ready HTTP services

---

## Project 8: Event-Driven Microservices with Message Queues

**Duration:** 3-4 weeks  
**Core Concepts:** Event sourcing, message queues, eventual consistency

Build a e-commerce order processing system with separate services for inventory, payment, and shipping, communicating through events.

**Key Libraries:**

- `lapin` for RabbitMQ integration
- `sqlx` with PostgreSQL
- `serde` for event serialization
- `uuid` for correlation IDs
- `opentelemetry` for distributed tracing

**TDD Focus:**

- Event handler unit tests with mock dependencies
- Saga pattern testing for distributed transactions
- Message queue integration tests
- End-to-end business process verification

**Learning Outcomes:**

- Event-driven architecture patterns
- Distributed transaction management
- Message queue integration
- Microservices communication patterns

---

## Project 9: High-Performance Metrics Collection System

**Duration:** 3-4 weeks  
**Core Concepts:** Performance optimization, memory management, time-series data

Create a system like Prometheus that collects, aggregates, and queries time-series metrics data with high throughput and low latency requirements.

**Key Libraries:**

- `tokio` for async I/O
- `parking_lot` for high-performance synchronization
- `byteorder` for binary data handling
- `lz4` for compression
- `criterion` for benchmarking

**TDD Focus:**

- Performance benchmarking with Criterion
- Memory usage testing and profiling
- Concurrent data structure verification
- Query engine correctness tests

**Learning Outcomes:**

- Performance optimization techniques
- Memory-efficient data structures
- Time-series database concepts
- Benchmarking and profiling in Rust

---

## Project 10: Container Orchestration Mini-Platform

**Duration:** 4 weeks  
**Core Concepts:** System programming, process management, advanced networking

Build a simplified container orchestration system that can deploy, scale, and manage containerized applications across multiple nodes.

**Key Libraries:**

- `nix` for system calls
- `bollard` for Docker API integration
- `k8s-openapi` for Kubernetes-compatible APIs
- `etcd-client` for distributed configuration
- `prometheus` for metrics export

**TDD Focus:**

- System integration tests with Docker containers
- Distributed state management testing
- API compatibility verification
- Failure scenario simulation and recovery

**Learning Outcomes:**

- System-level programming in Rust
- Container runtime integration
- Distributed system coordination
- Production deployment patterns

---

## General Testing Strategy for All Projects

### Unit Testing Approach

- Use `#[cfg(test)]` modules for unit tests
- Mock external dependencies with traits
- Test error conditions and edge cases
- Aim for >80% code coverage

### Integration Testing Structure

```rust
// tests/integration_test.rs
use your_project::*;

#[tokio::test]
async fn test_end_to_end_workflow() {
    // Integration test implementation
}
```

### Mocking Strategy

- Use `mockall` crate for mock generation
- Create trait abstractions for external dependencies
- Test both happy path and error scenarios

### Test Organization

```
project/
├── src/
├── tests/           # Integration tests
├── benches/         # Performance benchmarks
└── examples/        # Usage examples
```

## Progression Tips

1. **Start Simple**: Begin each project with core functionality, then add features
2. **Focus on Ownership**: Pay attention to how data moves through your system
3. **Error Handling**: Design comprehensive error types early
4. **Documentation**: Write docs as you code - it helps clarify your thinking
5. **Performance**: Profile and benchmark critical paths
6. **Community**: Engage with Rust community for code reviews and feedback

## Progress Tracking

- [ ] Project 1: Task Manager CLI
- [ ] Project 2: REST API for Expenses
- [ ] Project 3: Multi-threaded Web Scraper
- [ ] Project 4: Real-time Chat Server
- [ ] Project 5: File Processing Pipeline
- [ ] Project 6: Distributed Cache System
- [ ] Project 7: API Gateway with Load Balancing
- [ ] Project 8: Event-Driven Microservices
- [ ] Project 9: Metrics Collection System
- [ ] Project 10: Container Orchestration Platform

---

## Getting Started

1. **Setup Development Environment**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup component add clippy rustfmt
   cargo install cargo-watch cargo-nextest
   ```

2. **Project Structure Recommendation**

   ```
   rust-learning-journey/
   ├── project-01-task-manager/
   ├── project-02-expense-api/
   ├── project-03-web-scraper/
   └── ...
   ```

3. **Commit to completing each project fully before moving to the next**

---
