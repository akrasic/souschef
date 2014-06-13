module Souschef
  class Template
    # Updates README.md inside cookbook folder
    class Readme < Souschef::Template::Base
      # Public - Create standardised README
      #
      # Return nil
      def create(cookbook)
        tmpl = ERB.new(load_erb_file('readme.erb'))
        @cookbook = cookbook
        data = tmpl.result(binding)

        Souschef::Print.info 'Updating README file'
        write_file(cookbook_file_path(cookbook, 'README.md'), data)
      end
    end
  end
end
