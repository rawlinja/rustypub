# RustyPub

A minimal [ActivityPub](https://www.w3.org/TR/activitypub/) server written in Rust with [Axum](https://github.com/tokio-rs/axum) and [Tokio](https://tokio.rs/).

I'm building RustyPub to learn ActivityPub, the protocol behind Mastodon and the wider fediverse, and to get better at Rust. It's still small and in progress. I'm adding to it one piece of the spec at a time.

## What works today

- **WebFinger discovery.** `acct:user@host` resolves to an actor URL ([RFC 7033](https://www.rfc-editor.org/rfc/rfc7033)).
- **Actor documents.** `Person` actors are created on demand, with `@context`, `inbox` and `outbox`.
- **Inbox and outbox.** You can POST activities of any shape and read them back per user.
- Responses use `Content-Type: application/activity+json`.

## Running it

```bash
cargo run    # serves on http://127.0.0.1:3000
```

## Try it

```bash
# Discover an actor
curl "http://127.0.0.1:3000/.well-known/webfinger?resource=acct:alice@localhost"

# Fetch the actor document
curl http://127.0.0.1:3000/actor/alice

# Post an activity to alice's outbox
curl -X POST http://127.0.0.1:3000/actor/alice/outbox \
  -H "Content-Type: application/json" \
  -d '{"type":"Create","actor":"http://localhost:3000/actor/alice","object":{"type":"Note","content":"Hello, fediverse!"}}'

# Read it back
curl http://127.0.0.1:3000/actor/alice/outbox
```

## Project layout

| File | Contents |
|------|----------|
| `src/main.rs` | Router setup and server startup |
| `src/models.rs` | ActivityStreams and WebFinger types (serde) |
| `src/handlers.rs` | Axum request handlers |

State is shared across handlers through `Arc<RwLock<AppState>>`.

## Roadmap

1. **Client-to-server core.** Serve inbox and outbox as `OrderedCollection`s, give every activity a unique `id`, and wrap a bare `Note` in a `Create` activity automatically.
2. **Local delivery.** Deliver activities between actors on the same server.
3. **Client.** A small CLI for posting and reading.

## Not supported yet

These are left out on purpose, to keep the focus on learning the core protocol:

- **No persistence.** All state is in memory and is lost on restart.
- **No authentication or HTTP Signatures.** Anyone can post to any inbox. Federating with real servers like Mastodon would need signatures and HTTPS.
- The base URL is hardcoded to `http://localhost:3000`.
