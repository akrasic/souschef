module Souschef
  # Loads Souschef configuration YAML
  class Config
    # Public - Reads the configuration file
    #
    # Returns Hash
    def self.read
      verify_file
      YAML.load_file(File.expand_path('~/.souschef.yml'))
    end

    # Private - Checks if we have a configuraiton file
    #
    # Returns nil
    def self.verify_file
      conf = File.expand_path('~/.souschef.yml')
      fail "'~/.souschef.yml' missing!" unless File.exist?(conf)
    end
  end
end
