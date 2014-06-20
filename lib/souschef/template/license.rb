module Souschef
  class Template
    # Create standard LICENSE
    class License < Souschef::Template::Base
      def initialize(opts)
        super(opts)
      end

      # Public - Create a valid License file
      #
      # cookbook - String Cookbook name
      #
      # Returns nil
      def create
        tmpl = ERB.new(load_erb_file('license.erb'))
        data = tmpl.result(binding)

        Souschef::Print.info 'Updating LICENSE file'
        write_file(cookbook_file_path('LICENSE'), data)
      end
    end
  end
end
