require 'fileutils'

module Souschef
  # Rubocop configuration
  class Rubocop
    # Public - Copy rubocop.yml configuration to cookbook directory
    #
    # Returns nil
    def self.create(opts)
      Souschef::Print.info 'Copying Rubocop configuration'

      destination = "#{Dir.pwd}/#{opts[:cookbook]}/.rubocop.yml"
      rubocop_yml = File.expand_path('../../../data/rubocop.yml', __FILE__)

      FileUtils.cp(rubocop_yml, destination)
    end
  end
end
