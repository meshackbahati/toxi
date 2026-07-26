# Web/API Feature Crates

## `toxi-realtime`

Main modules/exports:

- SSE: `SseEvent`, `SseStream`, `SseConfig`
- pub/sub: `PubSub`, `Subscriber`, `Channel`
- event model: `Event`, `EventType`
- websocket: `WebSocketConnection`, `WebSocketManager`, `WsMessage`, `WebSocketError`

## `toxi-template`

Main APIs:

- `TemplateEngine`, `Context`, `Template`
- parser/renderer modules
- filters module
- static files: `StaticFiles`, `serve_static`, `static_handler`

## `toxi-openapi`

Main APIs:

- spec types: `OpenApiSpec`, `Info`, `Server`, `PathItem`, `Operation`, `Parameter`, `RequestBody`, `Response`, `Schema`, `Components`
- builders/helpers: `OpenApiBuilder`, `get_operation`, `post_operation`
- traits: `ToSchema`, `AutoDocs`
- docs renderer: `generate_docs_html`

## `toxi-graphql`

Main APIs:

- `GraphQLSchema`
- `Context`
- `ResolverExtension`, `ResolverRegistry`
- `GraphQLHandler`
- `create_handler()`

## `toxi-mail`

Main APIs:

- `Mailer`
- `Message`
- `SmtpTransport`, `SmtpConfig`
- `Attachment`

## `toxi-plugin`

Main APIs:

- plugin model: `Plugin`, `PluginInfo`, `PluginHook`, `HookResult`
- runtime: `PluginLoader`, `PluginManager`
- setup: `PluginConfig`, `create_manager`
