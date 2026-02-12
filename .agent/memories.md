# Memories

## Patterns

### mem-1770671543-f1c5
> TurboTUI code review 2026-02-09: Critical bug in main.rs:87 - KeyCode::Char(c) quits without Ctrl check so letter c cannot be typed. app.rs cursor_position is byte-index causing UTF-8 panics. format_query does not reset cursor. Config missing trusted_connection/trust_server_certificate fields. trust_cert always called. No connection pooling. No panic hook for terminal restore.
<!-- tags: review, turbotui | created: 2026-02-09 -->

## Decisions

## Fixes

## Context
