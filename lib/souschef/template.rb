require 'souschef/template/base'
require 'souschef/template/metadata'

module Souschef
  # Creates various files from predefined templates
  class Template
    # Public - Create needed standardised files
    #
    # Returns nil
    def self.run(cookbook)
      Souschef::Template::Metadata.new.create(cookbook)
    end
  end
end
