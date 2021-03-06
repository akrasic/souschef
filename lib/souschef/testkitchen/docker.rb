module Souschef
  class Testkitchen
    # Testkitchen Docker configuration
    class Docker
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
        { 'name' => 'docker',
          'privileged' => true
        }
      end

      # Private - Define provisioner
      #
      # Returns Hash
      def define_provisioner
        { 'name' => 'chef_zero',
          'require_chef_omnibus' => 'latest' }
      end

      # Private - Define platforms configuration
      #
      # Returns Array
      def define_platforms
        if define_run_list['run_list'].empty?
          [define_centos_5, define_centos_6]
        else
          [define_centos_5, define_centos_6, define_run_list]
        end
      end

      # Private - Define CentOS 5.10 platforms
      #
      # Returns Hash
      def define_centos_5
        { 'name' => 'centos-5.10',
          'driver_config' => { 'image' => 'akrasic/centos5',
                               'platform' => 'centos',
                               'use_sudo' => false,
                               'privileged' => true } }
      end

      # Private - Define CentOS 6.4 platform
      #
      # Returns Hash
      def define_centos_6
        { 'name' => 'centos-6.4',
          'driver_config' => { 'image' => 'centos:6.4',
                               'platform' => 'centos',
                               'use_sudo' => false,
                               'privileged' => true } }
      end

      # Private - Define Runlist for platforms
      #
      # Returs Hash
      def define_run_list
        { 'run_list' => '' }
      end

      # Private - Define suits
      #
      # Returns Hash
      def define_suits
        [{ 'name' => 'default',
           'run_list' => ["recipe[#{@cookbook}::default]"] }]
      end
    end
  end
end
