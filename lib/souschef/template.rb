require 'souschef/template/base'
require 'souschef/template/metadata'
require 'souschef/template/license'
require 'souschef/template/readme'

module Souschef
  # Creates various files from predefined templates
  class Template
    # Public - Create needed standardised files
    #
    # Returns nil
    def self.run(cookbook)
      Souschef::Template::Metadata.new.create(cookbook)
      Souschef::Template::License.new.create(cookbook)
      Souschef::Template::Readme.new.create(cookbook)
    end
  end
end
