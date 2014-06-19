module Souschef
  class Template
    # Configure Rubocop configuratiopn
    class Rubocop < Souschef::Template::Base
      # Public - Create a .rubocop file
      #
      # cookbook - String Cookbook name
      #
      # Returns nil
      def create(cookbook)
        tmpl = ERB.new(load_erb_file('rubocop.yml'))
        data = tmpl.result(binding)

        Souschef::Print.info 'Setting up Rubocop configuration'
        write_file(cookbook_file_path(cookbook, '.rubocop.yml'), data)
      end
    end
  end
end
