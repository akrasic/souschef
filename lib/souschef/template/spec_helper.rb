module Souschef
  class Template
    class Spechelper < Souschef::Template::Base
      # Public - Create spec/spec_helper.rb file
      #
      # cookbook - String Cookbook name
      #
      # Returns nil
      def create(cookbook)
        spec_dir = File.join(Dir.pwd, cookbook, 'spec')

        tmpl = ERB.new(load_erb_file('spec_helper.rb'))
        data = tmpl.result(binding)

        create_spec_dir(spec_dir) unless File.directory?(spec_dir)

        Souschef::Print.info 'Creating Chefspec helper'
        write_file(cookbook_file_path(cookbook, 'spec/spec_helper.rb'), data)
      end

      private

      # Private - Creates spec/ directory if missing
      #
      # Return nil
      def create_spec_dir(spec_dir)
        Souschef::Print.info 'Create spec/ directory'
        Dir.mkdir(spec_dir)
      end
    end
  end
end
