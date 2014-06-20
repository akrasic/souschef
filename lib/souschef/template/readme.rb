module Souschef
  class Template
    # Updates README.md inside cookbook folder
    class Readme < Souschef::Template::Base
      def initialize(opts)
        super(opts)
      end

      # Public - Create standardised README
      #
      # Return nil
      def create
        tmpl = ERB.new(load_erb_file('readme.erb'))
        @cookbook = @opts[:cookbook]
        data = tmpl.result(binding)

        Souschef::Print.info 'Updating README file'
        write_file(cookbook_file_path('README.md'), data)
      end
    end
  end
end
