require 'souschef/testkitchen/docker'
require 'souschef/testkitchen/solusvm'
require 'souschef/testkitchen/virtualbox'

module Souschef
  # TestKitchen generator
  class Testkitchen
    attr_accessor :files, :cookbook, :opts, :metadata

    def initialize(opts)
      @opts = opts
      @cookbook = opts[:cookbook]
      @file = kitchen_driver_file
      metadata_info
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

      Souschef::Print.info 'Creating Vagrant configuration in .kitchen.yml'
      write(:virtualbox, Souschef::Testkitchen::Virtualbox.new(@cookbook).yaml)
    end

    # Public - Copy the already generated files from the data/ directory
    #
    # Returns nil
    def setup
      templates = ['default']
      templates << @opts[:testkitchen] if @opts[:testkitchen]

      templates.each { |type| create_file(type) }
    end

    private

    # Private - Read metadata file
    #
    # Return Hash
    def metadata_info
      @metadata = Chef::Cookbook::Metadata.new
      @metadata.from_file(File.join(@opts[:path], 'metadata.rb'))
    end

    # Private - Creates the TestKitchen YML file based on the provided template
    #
    # Returns nil
    def create_file(type)
      cb_file, source_file = get_locations(type)

      File.open(cb_file, 'w') { |f| f.write(process_template(source_file)) }

      Souschef::Print.info "Creating Testkitchen #{type} configuration"
    rescue TypeError
      Souschef::Print.error 'SKipping'
    end

    # Private - Return location of cookbook file and source template file
    #
    # Return Array
    def get_locations(type)
      case type
      when 'default'
        cb_file = File.join(@opts[:path], '.kitchen.yml')
        source_file = template_location(type)
      else
        cb_file = File.join(@opts[:path], '.kitchen.local.yml')
        source_file = template_location(type)
      end

      [cb_file, source_file]
    end

    # Private - Generate the .kitchen.yml file based on our cookbook
    #
    # Returns String
    def process_template(source_file)
      rfile = ERB.new(File.read(source_file))
      @cookbook = @metadata.name

      rfile.result(binding)
    end

    # Private - Return location of the preconfigured .kitchen*.yml files
    #
    # Returns String
    def template_location(type)
      local = File.expand_path(
        "~/.souschef/#{@opts[:profile]}/testkitchen/kitchen.#{type}.erb",
        __FILE__)

      bundled = File.expand_path(
        "../../../data/testkitchen/kitchen.#{type}.erb", __FILE__)

      if !File.exist?(local) && !File.exist?(bundled)
        Souschef::Print.error "Missing custom configuration for TestKitchen \
#{@opts[:testkitchen]} configuration"
      else
        File.exist?(local) ? local : bundled
      end
    end

    # Private - Return full locations for files
    #
    # Returns Hash
    def kitchen_driver_file
      { virtualbox: File.join(@opts[:path], '.kitchen.yml'),
        docker: File.join(@opts[:path], '.kitchen.local.yml'),
        solusvm: File.join(@opts[:path], '.kitchen.local.yml')
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
