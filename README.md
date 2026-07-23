# souschef

CLI utility to interact with Chef server

# Install

Install the latest release from [crates.io](https://crates.io/crates/souschef) with Cargo:

```
cargo install souschef
```

This builds and installs the `souschef` binary into `~/.cargo/bin`. Make sure that
directory is on your `PATH`.

# Chef server API docs

https://chef-server-api-docs.chef.io/

# Profiles - Chef and Cinc

Supports using profiles from standard locations for:

- Chef `~/.chef/knife.rb`
- Cinc `~/.cinc/knife.rb`

If you have a configuration file in a non-standard location you can use `-c` or `--config` flag to use the path to the file.

### Example:

```
souschef -p cinc ...
souschef -p chef ...

souschef -c /some/path/knife.rb ... 
  
```

# Supported operations:

- `search 'query' [-a attribute ...]`
- `cookbook list | show COOKBOOK [-v VERSION]`
- `node list | show NODE | ssh NODE [-x user]`
- `role list | show ROLE`
- `environment list | show ENVIRONMENT`
- `data bag list | show`
- `data bag create | delete`
- `data bag encrypt-file | decrypt-file`
- `data bag upload-item | upload-encrypted-item`
- `login NODE [-x user]`
- `ssh 'query' 'command' [-x user]`

# Data bags

Full data bag management is supported, including encrypted items:

```
# Create / delete a data bag or item
souschef data bag create mybag
souschef data bag delete mybag [item]

# Encrypt / decrypt a JSON file locally using the shared secret
souschef data bag encrypt-file item.json --secret-file /path/to/secret
souschef data bag decrypt-file item.enc --secret-file /path/to/secret

# Upload a plain JSON file as an item (the file must contain an "id" field)
souschef data bag upload-item mybag item.json

# Decrypt an encrypted file and upload it as an item
souschef data bag upload-encrypted-item mybag item.enc --secret-file /path/to/secret

# Show contents
souschef data bag show mybag [item]
```

The secret can be provided inline with `--secret` or from a file with `--secret-file`.
Uploads update the item if it already exists (PUT), otherwise create it (POST).

# SSH usage

Two ways to reach a node over SSH:

- `login NODE` — resolves the node's IP address from the Chef server and opens an
  interactive shell, equivalent to running `ssh IP` yourself.
- `node ssh NODE` — same as `login`, kept for consistency with the `node` subcommands.
- `ssh 'query' 'command'` — runs a single command across every node matching a search
  query (non-interactive).

For the interactive commands a valid `~/.ssh/config` is helpful, as the process spawns a
SSH client equivalent to the `ssh NODE` command.

If you want to specify a username, use `-x`:

```
  souschef login NODE -x user
  souschef node ssh NODE -x user
  souschef ssh 'search:query' 'command' -x user
```
