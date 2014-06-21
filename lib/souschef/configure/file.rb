module Souschef
  class Configure
    # Create Configuration file
    class Yaml
      attr_accessor :opts, :data

      def initialize(opts)
        @opts = opts
        @souschef = File.expand_path('~/.souschef.yml')
        read_configuration
        add_values
        write_configuration
      end

      private

      # Private - Add CLI passed data into the configuration
      #
      # Return nil
      def add_values
        @data[@opts[:profile]] = { maintainer: @opts[:maintainer],
                                   maintainer_email: @opts[:maintainer_email],
                                   license: @opts[:license] }
      end

      # Private - Read configuration file, if it exists, otherwise define @data
      # as a empty Hash
      #
      # Returns Hash
      def read_configuration
        if File.exist?(@souschef)
          @data ||= YAML.load_file(@souschef)
        else
          @data = {}
        end
      end

      # Private - Write down configuration file
      #
      # Returns nil
      def write_configuration
        File.open(@souschef, 'w+') { |fd| fd.write(@data.to_yaml) }
      end
    end
  end
end
