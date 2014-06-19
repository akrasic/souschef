module Souschef
  class Template
    # Serverspec configurator
    class Serverspec < Souschef::Template::Base
      # Public - Create serverspec helper
      #
      # cookbook - String Cookbook name
      #
      # Returns nil
      def create(cookbook)
        spec_dir = File.join(Dir.pwd, cookbook, 'test', 'integration',
                             'default', 'serverspec')
        spec_helper = File.join(spec_dir, 'spec_helper.rb')

        tmpl = ERB.new(load_erb_file('serverspec_helper.rb'))
        data = tmpl.result(binding)

        create_spec_dir(spec_dir) unless File.directory?(spec_dir)
        Souschef::Print.info 'Creating Serverspec helper'
        write_file(spec_helper, data)
      end
    end
  end
end
