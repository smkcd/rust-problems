First, install sqlx-cli. This is SQLx's associated command-line utility for managing databases, migrations, and more:

```bash
$ cargo install sqlx-cli
```

Now you can execute migrations by running:

```bash
$ sqlx migrate run
```

**NOTE**: If you ever want to revert the migrations, simply run:

```bash
$ sqlx migrate revert
```

cargo watch -q -c -w src/ -x run