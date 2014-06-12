module Souschef
  class Testkitchen
    # Testkitchen Docker configuration
    class Solusvm
      attr_accessor :config, :cookbook

      def initialize(cookbook)
        @cookbook = cookbook
        populate_configuration
      end

      # Public - Return YAML formatted data
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
                    'suits' => define_suits }
      end

      # Private - Define driver section
      #
      # Returns Hash
      def define_driver
        { 'name' => 'solusvm',
          'privileged' => 'true'
        }
      end

      # Private - Define provisioner
      #
      # Returns Hash
      def define_provisioner
        { 'name' => 'chef-zero', 'require_chef_omnibus' => 'latest' }
      end

      # Private - Define platforms configuration
      #
      # Returns Array
      def define_platforms
        [define_centos_5, define_centos_6, define_run_list]
      end

      # Private - Define CentOS 5.10 platforms
      #
      # Returns Hash
      def define_centos_5
        { 'name' => 'centos-5.10',
          'driver_config' => { 'template' => 'centos-5.10-x86_64-solus',
                               'plan' => 'VPS2',
                               'node' => 'chefsolushv',
                               'privileged' => 'true',
                               'type' => 'xen',
                               'platform': 'centos',
                               'use_sudo' => 'false',
                               'username' => 'internal'   } }
      end

      # Private - Define CentOS 6.4 platform
      #
      # Returns Hash
      def define_centos_6
        { 'name' => 'centos-6-5',
          'driver_config' => { 'template' => 'centos-6.5-x86_64-solus',
                               'plan' => 'VPS2',
                               'node' => 'chefsolushv',
                               'privileged' => 'true',
                               'type' => 'xen',
                               'platform': 'centos',
                               'use_sudo' => 'false',
                               'username' => 'internal'   } }
      end

      # Private - Define Runlist for platforms
      #
      # Returs Hash
      def define_run_list
        { 'run_list' => 'nil' }
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
