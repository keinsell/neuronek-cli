# 2. Persistence Layer of Application

Date: 2024-09-09

## Status

accepted

## Context

What is the issue that we're seeing that is motivating this decision or change?

Core functionality of application is about gathering and analyzing data, there is no need for performance-oriented
solutions as there will not be much information to process - considering state of application and overall target user
base. There were different alternatives considered at very first we wanted to use `sled` database with binary
serialization of structs to have purely Rustic solution, yet `sled` seems to not be ready to be used in such way yet,
there are de facto crates like `sled_extensions` but all of this do not seem production ready in my opinion, and I would
like to avoid additional maintained costs of this application. I have considered `diesel` as ORM solution to be used
with `sqlite3`, however setup was a bit too harsh for what I need from ORM so eventually it all ended up on `seaorm` as
this solution is providing "good enough" migration toolkit and "good enough" integration with SQL through Query Builder
and Active Record pattern. This solution will be persisted for a long time as there are not viable alternatives in Rust
ecosystem to be considered. As additional libraries I have also taken attention to `typed_db`, `pallet` and some others
which names I do not remember.

## Decision

What is the change that we're proposing and/or doing?

## Consequences

What becomes easier or more difficult to do because of this change?
