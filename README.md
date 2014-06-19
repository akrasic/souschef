# Souschef

Souschef is a helper script to aid with starting a developing Chef cookbook by automating the creation of cookbook per standards.

Cookbook creation uses following steps:
- Using Berkshelf to create the cookbook structure
- Populates the `Gemfile`, `.kitchen.yml`, `.rubocop.yml`
- Adjusts the `metadata.rb`, `README` and `LICENSE˙
- Configured directorie and places `spec_helper.rb` for Chefspec and Serverspec
- Adds `Rakefile` which runs Foodcritic, Rubocop, RSpec and TestKitchecn tests 

## Installation
Clone this repository and do:
`gem build souschef.gemspec`
`gem install souschef-*.gem`

## Usage

Currently available options:

```
Options:
  --cookbook, -c <s>:   Name of your cookbook
        --docker, -d:   Enable Docker for TestKitchen
       --solusvm, -s:   Enable SoulsVM driver for TestKitchen
       --verbose, -v:   Print out detailed information
      --scaffold, -a:   Create recipe, chefspec and serverspec files for recipe
    --recipe, -r <s>:   Recipe name, used along with --scaffold
       --version, -e:   Print version and exit
          --help, -h:   Show this message

```

## Contributing

1. Fork it ( https://github.com/[my-github-username]/souschef/fork )
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create a new Pull Request

