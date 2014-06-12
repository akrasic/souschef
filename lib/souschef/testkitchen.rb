require 'souschef/testkitchen/docker'
require 'souschef/testkitchen/solusvm'
require 'souschef/testkitchen/virtualbox'

module Souschef
  # TestKitchen generator
  class Testkitchen
    attr_accessor :files, :cookbook, :opts

    def initialize(opts)
      @opts = opts
      @cookbook = opts[:cookbook]
      @file = kitchen_driver_file
    end

    # Public - Write down TestKitchen configuration
    #
    # Returns nil
    def configure
      if @opts[:docker]
        Souschef::Print.info 'Creating Docker configuration .kitchen.local.yml'
        write(:docker, Souschef::Testkitchen::Docker.new(@cookbook).yaml)
      end
      if @opts[:solusvm]
        Souschef::Print.info 'Creating SolusVM configuraton .kitchen.local.yml'
        write(:solusvm, Souschef::Testkitchen::Solusvm.new(@cookbook).yaml)
      end

      Souschef::Print.info 'Creating Virtualbox configuration in .kitchen.yml'
      write(:virtualbox, Souschef::Testkitchen::Virtualbox.new(@cookbook).yaml)
    end

    private

    # Private - Return full locations for files
    #
    # Returns Hash
    def kitchen_driver_file
      { virtualbox: "#{Dir.pwd}/#{@cookbook}/.kitchen.yml",
        docker: "#{Dir.pwd}/#{@cookbook}/.kitchen.local.yml",
        solusvm: "#{Dir.pwd}/#{@cookbook}/.kitchen.local.yml"
      }
    end

    # Private - Write down the configuration file
    #
    # Returns nile
    def write(driver, data)
      fd = File.open(kitchen_driver_file[driver], 'w')
      fd.write(data)
      fd.close unless fd.nil?
    end
  end
end
