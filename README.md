# Souschef

Souschef is a helper script to aid with starting a developing Chef cookbook by automating the creation of cookbook per standards.

Cookbook creation uses following steps:
- Using Berkshelf to create the cookbook structure
- Populates the `Gemfile`, `.kitchen.yml`, `.rubocop.yml`
- Adjusts the `metadata.rb`, `README` and `LICENSE`
- Configured directorie and places `spec_helper.rb` for Chefspec and Serverspec
- Adds `Rakefile` which runs Foodcritic, Rubocop, RSpec and TestKitchecn tests 

Souschef also can create new recipe and spec test files for you:
- file under `recipes/` directory
- file under `spec/unit/` directory
- file under `test/integration/default/serverspec/localhost/` directory


## Installation
Clone this repository and do:
`gem build souschef.gemspec`
`gem install souschef-*.gem`


# Getting started
First off you'd need to generate your configuration file that will hold data 
about your profile - `maintainer`, `maintainer_email` and `license` which are
written to the `~/.souschef.yml` file.
Additionally, you can have several profiles specified by the `--profile` argument.

**Creating default configuration**

`souschef --configure --maintainer 'John Doe' --maintainer-email 'john@doe.com' --license 'MIT'`

or 

`souschef  --profile 'default' --configure --maintainer 'John Doe' --maintainer-email 'john@doe.com' --license 'MIT'`
## Usage

Currently available options:

```
Options:
          --cookbook <s>:   Name of your cookbook
              --path <s>:   Define cookbook directory path (relative)
       --testkitchen <s>:   Pick your additional configuration to create
                            .kitchen.local.yml file
              --scaffold:   Create recipe, chefspec and serverspec files for
                            recipe
            --recipe <s>:   Recipe name, used along with --scaffold
           --profile <s>:   Pick your configuration profile (default: default)
                 --force:   Force create action
             --configure:   Create configuration file
               --verbose:   Print out detailed information
        --maintainer <s>:   Maintainer name
  --maintainer-email <s>:   Maintainer email
           --license <s>:   Licese you want to use, be explicit

```

# Example usage
**Create cookbook in current directory**

`souschef --cookbook mycb --verbose`

**Create cookbook using alternate profile from your configuration file**

`souschef --cookbook mycb --profile other --verbose`

**Create cookbook in a subfolder of current directory**

`souschef --cookbook mycb --path subdir --verbose`

**Use scaffold to get you started writing a new recipe and tests**

`souschef --scaffold --recipe install`

# Profile support
Souschef lets you one or several profiles/configurations under one roof, to get
started simply configure it via:

```
souschef --configure --maintainer "YOUR NAME" \
--maintainer-email "YOUR EMAIL" \
--license "Restricted license, do not touch"
```

Profile configuration is written inside `~/.souschef.yml` file and can be easily
edited by hand if needed.

# Customization

When creating a cookbook, Souschef will look first if a custom directory for the
chosen profile exists - `~/.souschef/$PROFILE` and search if the template it is
currently creating exists there - if so, it will be used instead of the bundled
one.


You can customize the default configuration that comes with few easy steps,
below is an example for a custom recipe file

**Create ~/.souschef/ directory for chosen profile**

`mkdir -p ~/souschef/default/recipe`

**Create your own rubocop.yml file**

`vim ~/.shouschef/default/recipe/recipe.erb`

**Run souschef command and you will see following in the output**

```
~> Create default[recipe] from /home/user/.souschef/default/recipe/recipe.erb
```

**Full directory structure of custom files**

```
├── chefspec                                                                                                                                                           
│   ├── chefspec.erb                                                                                                                                                   
│   └── spec_helper.rb                                                                                                                                                 
├── gemfile.yml                                                                                                                                                        
├── license.erb                                                                                                                                                        
├── metadata.erb                                                                                                                                                       
├── rakefile.erb                                                                                                                                                       
├── readme.erb                                                                                                                                                         
├── recipe                                                                                                                                                             
│   └── recipe.erb                                                                                                                                                     
├── rubocop                                                                                                                                                            
│   └── rubocop.yml                                                                                                                                                    
├── serverspec                                                                                                                                                         
│   ├── serverspec.erb                                                                                                                                                 
│   └── serverspec_helper.rb                                                                                                                                           
└── testkitchen                                                                                                                                                        
    └── kitchen.default.erb  
```

**

## Contributing

1. Fork it ( https://github.com/[my-github-username]/souschef/fork )
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create a new Pull Request
