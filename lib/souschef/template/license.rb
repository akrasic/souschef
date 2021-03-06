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
        @year = Time.now.year
        @maintainer = @opts[:souschef][:maintainer]
        data = tmpl.result(binding)

        info 'Updating LICENSE file'
        write_file(cookbook_file_path('LICENSE'), data)
      end
    end
  end
end
