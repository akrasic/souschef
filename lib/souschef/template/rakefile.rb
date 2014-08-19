module Souschef
  class Template
    # Add Rakefile for testing support
    class Rakefile < Souschef::Template::Base
      def initialize(opts)
        super(opts)
      end

      # Public - Create a Rakefile from our Template
      #
      # cookbook - String Cookbook name
      #
      # Returns nil
      def create
        tmpl = ERB.new(load_erb_file('rakefile.erb'))
        data = tmpl.result(binding)

        info 'Setting up Rakefile'
        write_file(cookbook_file_path('Rakefile'), data)
      end
    end
  end
end
