# Souschef
module Souschef
  class Template
    # Creates a well defined maetadata.rb
    class Metadata < Souschef::Template::Base
      attr_accessor :cookbook

      def initialize(opts)
        super(opts)
      end

      # Public - Create file start
      #
      # cookbook - String cookbook name
      #
      # Returns nil
      def create
        tmpl = ERB.new(load_erb_file('metadata.erb'))
        @cookbook = @opts[:cookbook]
        @maintainer = @opts[:souschef][:maintainer]
        @maintainer_email = @opts[:souschef][:maintainer_email]
        @license = @opts[:souschef][:license]
        data = tmpl.result(binding)

        info 'Updating metadata.rb'
        write_file(cookbook_file_path('metadata.rb'), data)
      end
    end
  end
end
