module Souschef
  # Configure Rspec
  class Spec
    # Public - Create spec/ directory and copy spec_helper.rb file
    #
    # Returns nil
    def self.create(opts)
      destination = "#{Dir.pwd}/#{opts[:cookbook]}/spec/"
      helper  = File.expand_path('../../../data/spec_helper.rb', __FILE__)

      Souschef::Print.info 'Create spec/ directory'
      Dir.mkdir(destination)

      Souschef::Print.info 'Copy spec_helper.rb to spec/'
      FileUtils.cp(helper, destination)
    end
  end
end
