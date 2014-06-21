module Souschef
  # Loads Souschef configuration YAML
  class Config
    # Public - Reads the configuration file
    #
    # Returns Hash
    def self.read
      YAML.load_file(File.expand_path('~/.souschef.yml'))
    end
  end
end
