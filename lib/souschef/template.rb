require 'souschef/template/base'
require 'souschef/template/metadata'
require 'souschef/template/license'
require 'souschef/template/readme'
require 'souschef/template/rubocop'
require 'souschef/template/spec_helper'

module Souschef
  # Creates various files from predefined templates
  class Template
    # Public - Create needed standardised files
    #
    # Returns nil
    def self.run(cookbook)
      Souschef::Template::Rubocop.new.create(cookbook)
      Souschef::Template::Spechelper.new.create(cookbook)
      Souschef::Template::Metadata.new.create(cookbook)
      Souschef::Template::License.new.create(cookbook)
      Souschef::Template::Readme.new.create(cookbook)
    end
  end
end
