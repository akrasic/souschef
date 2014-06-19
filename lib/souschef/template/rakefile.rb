module Souschef
  class Template
    # Add Rakefile for testing support
    class Rakefile < Souschef::Template::Base
      # Public - Create a Rakefile from our Template
      #
      # cookbook - String Cookbook name
      #
      # Returns nil
      def create(cookbook)
        tmpl = ERB.new(load_erb_file('rakefile.erb'))
        data = tmpl.result(binding)

        Souschef::Print.info 'Setting up Rakefile'
        write_file(cookbook_file_path(cookbook, 'Rakefile'), data)
      end
    end
  end
end
