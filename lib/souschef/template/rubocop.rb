module Souschef
  class Template
    # Configure Rubocop configuratiopn
    class Rubocop < Souschef::Template::Base
      def initialize(opts)
        super(opts)
      end
      # Public - Create a .rubocop file
      #
      # cookbook - String Cookbook name
      #
      # Returns nil
      def create
        tmpl = ERB.new(load_erb_file('rubocop/rubocop.yml'))
        data = tmpl.result(binding)

        info 'Setting up Rubocop configuration'
        write_file(cookbook_file_path('.rubocop.yml'), data)
      end
    end
  end
end
