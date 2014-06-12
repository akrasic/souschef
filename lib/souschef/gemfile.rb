module Souschef
  # Gemfile generator
  class Gemfile < Ruth::Gemfile::Yaml
    attr_accessor :opts

    def initialize(opts)
      @opts = opts
      super(yaml_file_location)
    end

    # Public - Writes down the Gemfile location
    #
    # Returns nil
    def write
      Souschef::Print.info 'Populating Gemfile'
      write_gemfile
    end

    private

    # Private - Return location of YAML file
    #
    # Returns String
    def yaml_file_location
      File.expand_path('../../../data/gemfile.yml', __FILE__)
    end

    # Private - Return location of Gemfile
    #
    # Returns string
    def gemfile_location
      File.join(Dir.pwd, @opts[:cookbook], 'Gemfile')
    end

    # Private - Write Gemfile data
    #
    # Returns nil
    def write_gemfile
      File.open(gemfile_location, 'w') { |file| file.write(output) }
    end
  end
end
