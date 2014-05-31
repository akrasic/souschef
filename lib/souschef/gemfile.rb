module Souschef
  # Gemfile generator
  class Gemfile < Ruth::Gemfile

    def initialize
      super()
    end

    private

    # Private - Read the internal YAML Gemfile
    #
    # Returns Array
    def read_yaml
      yaml_file = File.expand_path('../../data/gemfile.yml', __FILE__)
      YAML.load_file(yaml_file)
    end


  end
end
