# Embedb
## We implement a vector databse from scratch in Rust
### Features
1. In Memory Vector DB
2. Recovery
3. ACID
4. Telemetry/Health Monitoring
5. Replication
6. Sharding
### Extensions
7. Embedding
8. Fine Tuning Embedding
9. Unstructured/ Semi Structured Db
10. Suppprt for multiple source push and pull
### Objective
1. High Availibility
2. Low Latency
3. Consistency Support
4. Scalable
5. Recovery in case of parition/ node failure
6. Reliable

### Implementation
Entities
    DB -> List <Record>
    Record(id, value, Embeddings)
    Index
    Embeddings

Services
    DBService -> CreateDB, DestroyDB, Insert, Search