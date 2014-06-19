module Souschef
  class Testkitchen
    # TestKitchen Virtualbox configuration
    class Virtualbox
      attr_accessor :config, :cookbook

      def initialize(cookbook)
        @cookbook = cookbook
        populate_configuration
      end

      # Public - Return Testkitchecn Virtualbox configuration in YAML format
      #
      # Returns String
      def yaml
        @config.to_yaml
      end

      private

      # Private - Populate @config
      #
      # Returns nil
      def populate_configuration
        @config = { 'driver' => define_driver,
                    'provisioner' => define_provisioner,
                    'platforms' => define_platforms,
                    'suites' => define_suits }
      end

      # Private - Define driver section
      #
      # Returns Hash
      def define_driver
        { 'name' => 'vagrant',
          'customize' => { 'memory' => 1024 } }
      end

      # Private - Define provisioner
      #
      # Returns Hash
      def define_provisioner
        { 'name' => 'chef_zero',
          'require_chef_omnibus' => 'latest' }
      end

      # Private - Define Platform
      #
      # Returns Hash
      def define_platforms
        [{ 'name' => 'centos-5.10',
           'driver_config' => { 'box' => 'centos-5.10-min-x86_64' } },
         { 'name' => 'centos-6.4',
           'driver_config' => { 'box' => 'centos-6.5-x86_64' } }]
      end

      # Private - Define suits
      #
      # Returns Hash
      def define_suits
        [{ 'name' => 'default',
           'run_list' => ["recipe[#{@cookbook}::default]"],
           'attributes' => 'nil' }]
      end
    end
  end
end
