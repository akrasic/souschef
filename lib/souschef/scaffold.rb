require 'chef/cookbook/metadata'
require 'fileutils'

module Souschef
  # Automagically create needed files
  class Scaffold
    attr_accessor :opts, :dir, :recipe, :recipe_file, :cookbook, :metadata

    def initialize(opts)
      @opts = opts
      @dir = Dir.pwd
      @templates = return_templates
      metadata_info
    end

    def start
      check_cookbook_name
      check_for_metadata
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
      rfile = ERB.new(File.read(return_templates[type.to_sym]))
      @recipe = @opts[:recipe]
      @cookbook = @metadata.name
      @maintainer = @metadata.maintainer
      @license = @metadata.license
      @year = Time.now.year

      data = rfile.result(binding)

      Souschef::Print.info "Creating #{opts[:recipe]} #{type} file"
      check_for_directories(type)
      write_file(return_file_location(type), data)
    end

    # Private - Return location of template files
    #
    # Returns Hash
    def return_templates
      { recipe:  File.expand_path('../../../data/recipe.erb', __FILE__),
        serverspec: File.expand_path('../../../data/serverspec.erb', __FILE__),
        chefspec: File.expand_path('../../../data/chefspec.erb', __FILE__) }
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
      fail "#{file} already exists. Frozen in fear!" if File.exist?(file)
      fd = File.open(file, 'w')
      fd.write(data)
    end
  end
end
