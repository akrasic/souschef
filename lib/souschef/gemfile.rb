module Souschef
  # Gemfile generator
  class Gemfile < Ruth::Gemfile::Yaml
    attr_accessor :cwd, :dir

    def initialize(opts)
      @cwd = Dir.pwd
      @dir = opts[:cookbook]
      @gemfile = "#{@cwd}/#{@dir}/Gemfile"

      Souschef::Print.info 'Populating Gemfile'
      yaml_file = File.expand_path('../../../data/gemfile.yml', __FILE__)

      super(yaml_file)
      output
    end
  end
end
