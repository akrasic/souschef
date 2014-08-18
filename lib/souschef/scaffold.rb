require 'chef/cookbook/metadata'
require 'fileutils'

module Souschef
  # Automagically create needed files
  class Scaffold
    attr_accessor :opts, :dir, :recipe, :recipe_file, :cookbook, :metadata

    def initialize(opts)
      @opts = opts
      @dir = Dir.pwd
      metadata_info
    end

    def start
      check_cookbook_name
      check_for_metadata unless @opts[:force]
      process_templates
    end

    private

    # Private - Chef if cookbook name is set
    #
    # Returns nil
    def check_cookbook_name
      fail 'Please specify the recipe name' if @opts[:recipe].nil?
    end

    # Private - Read Chef metadata
    #
    # Returns String
    def metadata_info
      meta = File.join(@opts[:path], 'metadata.rb')
      @metadata = Chef::Cookbook::Metadata.new
      @metadata.from_file(meta)
    end

    # Private - Process tempaltes
    #
    # Return inl
    def process_templates
      %w( recipe serverspec chefspec ).each { |type| create_recipe_file(type) }
    end

    # Private - Creates recipe file based on the input
    #
    # Retunrns nil
    def create_recipe_file(type)
      source = template_location(type)
      rfile = ERB.new(File.read(source))
      @recipe = @opts[:recipe]
      @cookbook = @metadata.name
      @maintainer = @metadata.maintainer
      @license = @metadata.license
      @year = Time.now.year

      data = rfile.result(binding)

      Souschef::Print.info "Create #{@opts[:recipe]}[#{type}] from #{source}"
      check_for_directories(type)
      write_file(return_file_location(type), data)
    end

    # Private - Return location of the template file, depending if custom
    # configuration is set under ~/.souschef/%profile%/ or use the default
    # template provided by Souschef gem.
    #
    # Returns String
    def template_location(type)
      local = File.expand_path(
        "~/.souschef/#{@opts[:profile]}/#{type}/#{type}.erb", __FILE__)
      bundled = File.expand_path("../../../data/#{type}/#{type}.erb", __FILE__)
      File.exist?(local) ? local : bundled
    end

    # Private - Return location of directories
    #
    # Returns Hash
    def return_directories
      { recipe: File.join(@opts[:path], 'recipes'),
        serverspec: File.join(@opts[:path], 'test', 'integration', 'default',
                              'serverspec', 'localhost'),
        chefspec: File.join(@opts[:path], 'spec', 'unit') }
    end

    # Private - Get path to the recipes file
    #
    # Return String
    def return_file_location(type)
      case type
      when 'recipe'
        File.join(return_directories[:recipe], "#{@opts[:recipe]}.rb")
      when 'serverspec'
        File.join(return_directories[:serverspec], "#{opts[:recipe]}_spec.rb")
      when 'chefspec'
        File.join(return_directories[:chefspec], "#{@opts[:recipe]}_spec.rb")
      end
    end

    # Private - Check if directories exist
    #
    # Return nil
    def check_for_directories(type)
      dir = return_directories[type.to_sym]
      Souschef::Print.info "Creating missing directory #{dir}" unless
      File.directory?(dir)
      FileUtils.mkdir_p dir unless File.directory?(dir)
    end

    # Private - Locate metadata.rb
    #
    # Return nil
    def check_for_metadata
      meta = File.join(@opts[:path], 'metadata.rb')
      fail 'Please return to the root of your cookbook' unless File.exist?(meta)
    end

    # Private - Write file
    #
    # Returns nil
    def write_file(file, data)
      check_for_file(file) unless @opts[:force]
      fd = File.open(file, 'w')
      fd.write(data)
    end

    # Private - Exit if file exists
    #
    # Returns nil
    def check_for_file(file)
      fail "#{file} already exists. Frozen in fear!" if File.exist?(file)
    end
  end
end
