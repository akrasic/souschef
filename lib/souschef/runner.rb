module Souschef
  # Runner library that does all the hard work
  class Runner
    attr_accessor :opts

    def initialize(opts)
      @opts = opts
      adjust_cb_path
      load_configuration unless @opts[:configure]
    end

    # Public - Run Souschef
    #
    # Returns nil
    def run
      Souschef::Print.header "Using Souschef profile: #{opts[:profile]}"

      if @opts[:scaffold]
        Souschef::Scaffold.new(@opts).start
      elsif @opts[:configure]
        verify_configure_input
        Souschef::Configure::Yaml.new(@opts)
      else
        Souschef::Print.header "Starting cookbook creation...\n"
        cookbook_runlist
      end
    end

    private

    # Private - Create cookbook run list
    #
    # Returns nil
    def cookbook_runlist
      verify_cookbook_creation

      Souschef::Print.header 'Berkshelf configuration'
      Souschef::Berkshelf.new(@opts).berks_create
      Souschef::Print.header 'Configure gemfile'
      Souschef::Gemfile.new(@opts).write
      Souschef::Print.header 'Create essential template files'
      Souschef::Template.run(@opts)
      # Mock Scaffold to generate default recipe and tests

      Souschef::Print.header 'Default recipe'
      Souschef::Scaffold.new(path: @opts[:path],
                             recipe: 'default',
                             profile: @opts[:profile],
                             force: true).start

      Souschef::Print.header 'Testkitchen'
      Souschef::Testkitchen.new(@opts).setup

      Souschef::Print.header "Don't forget to run bundle install!"
    end

    # Private - Verify @opts values
    #
    # Return nil
    def verify_configure_input
      if @opts[:maintainer_given] || @opts[:maintainer_email_given] ||
        @opts[:license_given]
        unless @opts[:configure]
          fail 'Please check Souschef options, you are missing --configure'
        end
      else
        fail 'Please check if you have all options for configuration selected'
      end
    end

    # Private - Verify cookbook creation options
    #
    # Return nil
    def verify_cookbook_creation
      fail 'You need to specify the cookbook name' if @opts[:cookbook].nil?
    end

    # Private - Verify Scaffold input
    #
    # Return nil
    def verify_scaffold_input
      fail 'Recipe name is missing for scaffold creation' if @opts[:recipe].nil?
    end

    # Private - Adjust cookbook path
    #
    # Return nil
    def adjust_cb_path
      if @opts[:path]
        @opts[path] = File.join(Dir.pwd, @opts[:path])
      else
        @opts[:path] = Dir.pwd
      end
    end

    # Private - Load contents of .souschef.yml configuration
    #
    # Returns nil
    def load_configuration
      @opts[:souschef] = Souschef::Config.read[@opts[:profile]]
    end
  end
end
