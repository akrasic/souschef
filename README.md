# souschef

CLI utility to interact with Chef server

# Chef server API docs

https://chef-server-api-docs.chef.io/

# Profiles - Chef and Cinc

Supports using profiles from standard locations for:

- Chef `~/.chef/knife.rb`
- Cinc `~/.cinc/knife.rb`

If you have a configuration file in a non-standard location you can use `-c` or `--config` flag to use the path to the file.

# Supported operations:

- search
- node list | show
- roles list | show
- environment list | show
- data bag list | show (encrypted data bag items not supported yet)
