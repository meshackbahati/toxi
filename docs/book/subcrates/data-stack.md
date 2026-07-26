# Data and State Crates

## `toxi-db`

Key types:

- `DbPool`, `DbTransaction`
- `DatabaseType`, `PoolOptions`
- `Model` trait + `#[derive(Model)]`
- `ModelQuery`, `Pagination`, `SortDirection`, `QueryBuilder`
- `OrmError`, `OrmResult`
- relations: `HasMany`, `HasOne`, `BelongsTo`
- migrations: `Migration`, `MigrationManager`

Golden path:

1. connect with `DbPool::connect` or `connect_with_options`
2. derive `Model`
3. query with `Model::query()` + typed filters/order/pagination
4. use `with_transaction` for multi-step writes

## `toxi-macros`

Main macro:

- `#[derive(Model)]`

Attribute forms:

- `#[model(table = "...")]`
- supports validation attributes handled by the derive

Use this crate with `toxi-db` to reduce model boilerplate while keeping compile-time diagnostics.

## `toxi-cache`

Main APIs:

- trait `Cache`
- `MemoryCache`
- `RedisCache`
- `NamespacedCache`

Use for caching read-heavy paths and invalidating by namespace/tag strategy.

## `toxi-queue`

Main APIs:

- `Job`, `JobStatus`, `JobResult`
- `Queue`, `QueueBackend`, `MemoryBackend`
- `RedisBackend`, `PostgresBackend`
- `Worker`
- `QueueStats`, `StatsTracker`

Use for background jobs with selectable backends.

## `toxi-storage`

Main APIs:

- trait `Storage`
- `LocalStorage`
- `S3Storage`
- `FileValidator`, `ValidationRules`
- `StoredFile`, `FileMetadata`

Use for user uploads and object storage abstraction.
