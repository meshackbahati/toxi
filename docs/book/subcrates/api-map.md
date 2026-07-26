# Subcrate API Map

This page maps core public APIs so you can quickly find the right type, trait, or function.

## `toxi-core`

- modules: `error`, `extract`, `request`, `response`, `router`, `server`, `tls`, `types`, `versioning`, `cookie`
- common exports:
- `Error`, `Result`
- `Router`, `Handler`, `Server`
- `Request`, `Response`
- extractors: `FromRequest`, `Json`, `Path`, `Query`, `State`, `Form`, `Cookies`, `Body`
- versioning: `ApiVersion`, `VersionedRouter`

## `toxi-db`

- db types: `DatabaseType`, `PoolOptions`, `DbPool`, `DbTransaction`
- traits: `Database`, `Model`
- query types: `ModelQuery`, `QueryBuilder`, `Pagination`, `SortDirection`, `QueryValue`
- errors/results: `OrmError`, `OrmResult`, DB `Result`
- relations: `HasMany`, `HasOne`, `BelongsTo`
- migrations: `Migration`, `MigrationManager`

## `toxi-auth`

- password hashing: `PasswordHasher`, `hash_password`, `verify_password`
- JWT: `JwtManager`, `create_token`, `verify_token`, `Claims`
- middleware: `AuthMiddleware`, `SessionMiddleware`, `SessionLayer`, `ApiKeyMiddleware`
- sessions: `Session`, `SessionStore`, `InMemorySessionStore`, `RedisSessionStore`, `SessionManager`
- authorization: `Role`, `Permission`, `RequireRole`, `RequirePermission`, `AuthorizationService`
- OAuth: `OAuth2Client`, `OAuth2Config`, `ProviderConfig`, `OAuth2Provider`
- API keys: `ApiKey`
- errors: `AuthError`, auth `Result`

## `toxi-cache`

- trait: `Cache`
- implementations: `MemoryCache`, `RedisCache`, `NamespacedCache`
- support types: `CacheStats`
- errors: `CacheError`, cache `Result`

## `toxi-queue`

- queue/job: `Queue`, `QueueBackend`, `MemoryBackend`, `RedisBackend`, `PostgresBackend`
- job model: `Job`, `JobStatus`, `JobResult`
- workers/stats: `Worker`, `QueueStats`, `StatsTracker`
- errors: `QueueError`, queue `Result`

## `toxi-realtime`

- SSE: `SseEvent`, `SseStream`, `SseConfig`
- pubsub: `PubSub`, `Subscriber`, `Channel`
- event: `Event`, `EventType`
- websocket: `WebSocketConnection`, `WebSocketManager`, `WsMessage`, `WebSocketError`
- errors: `RealtimeError`, realtime `Result`

## `toxi-template`

- rendering: `TemplateEngine`, `Template`, `Context`
- internals: `Parser`, `TemplateNode`, `Renderer`, `Filters`
- static serving: `StaticFiles`, `serve_static`, `static_handler`
- errors: `TemplateError`, template `Result`

## `toxi-storage`

- trait: `Storage`
- backends: `LocalStorage`, `S3Storage`
- validation: `FileValidator`, `ValidationRules`
- metadata: `StoredFile`, `FileMetadata`
- errors: `StorageError`, storage `Result`

## `toxi-security`

- crypto: `encrypt`, `decrypt`, `AesKey`
- hash/HMAC: `sha256`, `sha512`, `hmac_sha256`, `verify_hmac_sha256`
- random: `random_bytes`, `random_hex`, `secure_token`, `random_alphanumeric`, `random_range`, `try_random_range`
- sanitize: `sanitize_html`, `escape_html`, `strip_tags`
- errors: `SecurityError`, security `Result`

## `toxi-openapi`

- spec types: `OpenApiSpec`, `Info`, `Server`, `PathItem`, `Operation`, `Parameter`, `RequestBody`, `Response`, `MediaType`, `Schema`, `Components`
- builders/helpers: `OpenApiBuilder`, `get_operation`, `post_operation`, `generate_docs_html`
- traits: `ToSchema`, `AutoDocs`

## `toxi-graphql`

- runtime: `GraphQLSchema`, `GraphQLHandler`, `create_handler`
- context/resolvers: `Context`, `ResolverExtension`, `ResolverRegistry`

## `toxi-plugin`

- plugin model: `Plugin`, `PluginInfo`, `PluginHook`, `HookResult`
- runtime: `PluginLoader`, `PluginManager`
- setup: `PluginConfig`, `create_manager`

## `toxi-mail`

- mail APIs: `Mailer`, `Message`, `Attachment`
- transport: `SmtpTransport`, `SmtpConfig`
- errors: `MailError`, mail `Result`

## `toxi-config`

- `Config`, `Environment`
- `AppConfig`, `ServerConfig`, `DatabaseConfig`, `CacheConfig`, `QueueConfig`, `SecurityConfig`
- errors: `ConfigError`

## `toxi-testing`

- `TestRequest`, `TestRequestError`
- `TestResponse`
- `TestServer`, `test_router`
- async test support: `tokio::test` re-export

## `toxi-utils`

- date utilities
- id utilities
- string utilities
- validation utilities

## `toxi-cli`

- command surface for project scaffolding, code generation, migrations, seeds, dev/runtime workflows.
