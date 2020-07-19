# Hello World

## Concept

To demonstrate a simple `Hello, World` using Diesel.

## Instructions

* Setup your database with:

```shell
echo DATABASE_URL=postgres://localhost/hello_world > .env
diesel setup
diesel migration run
```

* Create a post with:

```shell
cargo run --bin write_post
```

* Publish a post with:

```shell
cargo run --bin publish_post <ID>
```

* View published posts with:

```shell
cargo run --bin show_posts  
```

* Delete a post with:

```shell
cargo run --bin delete_post <LIKE TARGET>
```
