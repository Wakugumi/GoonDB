
# GoonDB

Hey twin, do you like gooning? Sure do I like gooning. That is why I wrote this cutting-EDGE database for you to goon.
**GoonDB** is a lightweight key–value hash table database written in **Rust**.  
It is conceptually similar to Redis, but with goonology in mind to make your productivity even more productive.

Why do I make this? Simple answer, I like gooning, I am unemployed, I like coding, I make this. With goonology, I can write database and gooning at the same time.

This project exists primarily for **learning, experimentation, and fun**.

---

## Features

- 🧠 In-memory key–value storage
- ⚡ Fast hash-table based access
- 🦀 Written in Rust
- 🧩 Simple server–client model

---

## Non-Goals

GoonDB is **not** intended to:
- Replace Redis
- Be production-ready
- Provide durability guarantees
- Be taken too seriously
- For gooning
- For NSFW stuff

---

## Architecture Overview

- Core storage: hash table (key → value)
- WAL mechanism for recovery
- Minimal abstractions
- No persistence (for now)
- No clustering (for now)

Right now we still use WAL for source of truth

---

## Example Usage

We design a simple query syntax to do stuff with the database. With our REPL (coming soon) you can goon directly to the database file.
To insert a value:
```text
GOON <key> <value>
```

Get a value:
```
EDGE <key>
```
```

```
Remove a value:
```
BUST <key>
```


# Contributing
Hop on now twin and start gooning (contributing) together twin
