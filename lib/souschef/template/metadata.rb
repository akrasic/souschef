# Souschef
module Souschef
  class Template
    # Creates a well defined maetadata.rb
    class Metadata < Souschef::Template::Base
      attr_accessor :cookbook

      # Public - Create file start
      #
      # cookbook - String cookbook name
      #
      # Returns nil
      def create(cookbook)
        tmpl = ERB.new(File.read(datafile_path('metadata.erb')))
        @cookbook = cookbook
        data = tmpl.result(binding)

        Souschef::Print.info 'Updating metadata.rb'
        write_file(cookbook_file_path(cookbook, 'metadata.rb'), data)
      end
    end
  end
end
