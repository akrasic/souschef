module Souschef
  class Template
    # Create spechelper for Chefspec
    class Chefspec < Souschef::Template::Base
      def initialize(opts)
        super(opts)
      end
      # Public - Create spec/spec_helper.rb file
      #
      # cookbook - String Cookbook name
      #
      # Returns nil
      def create
        spec_dir = File.join(@path, 'spec')

        tmpl = ERB.new(load_erb_file('chefspec/spec_helper.rb'))
        data = tmpl.result(binding)

        create_spec_dir(spec_dir) unless File.directory?(spec_dir)

        Souschef::Print.info 'Creating Chefspec helper'
        write_file(cookbook_file_path('spec/spec_helper.rb'), data)
      end
    end
  end
end
