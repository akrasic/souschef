# souschef

CLI utility to interact with Chef server

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

- search
- node list | show | ssh 
- roles list | show
- environment list | show
- data bag list | show (encrypted data bag items not supported yet)
- ssh

# SSH usage
For using SSH in `node ssh NODE` or `ssh`  subcommand a valid `~/.ssh/config` would be needed, as the process
would spawn a SSH client, equivalent to the `ssh NODE` command.

If you want to specify a username, use `-x` as:

```
  souschef knife ssh NODE -x user
  souschef ssh 'search:query' 'command' -x user 
```
