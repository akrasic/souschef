require 'souschef/template/base'
require 'souschef/template/metadata'
require 'souschef/template/license'
require 'souschef/template/readme'
require 'souschef/template/rubocop'
require 'souschef/template/chefspec'
require 'souschef/template/serverspec'
require 'souschef/template/rakefile'

module Souschef
  # Creates various files from predefined templates
  class Template
    # Public - Create needed standardised files
    #
    # Returns nil
    def self.run(opts)
      Souschef::Template::Rubocop.new(opts).create
      Souschef::Template::Chefspec.new(opts).create
      Souschef::Template::Serverspec.new(opts).create
      Souschef::Template::Metadata.new(opts).create
      Souschef::Template::License.new(opts).create
      Souschef::Template::Readme.new(opts).create
      Souschef::Template::Rakefile.new(opts).create
    end
  end
end
