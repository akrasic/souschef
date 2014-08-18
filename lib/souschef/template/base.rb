module Souschef
  class Template
    # Base class containing reusable functions
    class Base
      attr_accessor :path, :ots

      def initialize(opts)
        @opts = opts
        @path = opts[:path]
      end

      private

      # Private - Return location of a custom template file if it exists, or
      # return the default version
      #
      # Return String
      def datafile_path(file)
        local_profile = "~/.souschef/#{@opts[:profile]}/#{file}"
        profile = File.expand_path(local_profile, __FILE__)

        if File.exist?(profile)
          profile
        else
          p = "../../../../data/#{file}"
          File.expand_path(p, __FILE__)
        end
      end

      # Private - Return path to the file inside cookbook directory
      #
      # Return String
      def cookbook_file_path(file)
        File.join(@path, file)
      end

      # Private - Write data to the desired file
      #
      # Returns nil
      def write_file(file, data)
        File.open(file, 'w') { |fd| fd.write(data) }
      end

      # Private - Load ERB template
      #
      # file - String file name
      #
      # Returns String
      def load_erb_file(file)
        File.read(datafile_path(file))
      end

      # Private - Creates spec/ directory if missing
      #
      # Return nil
      def create_spec_dir(spec_dir)
        Souschef::Print.info "Create #{spec_dir} directory"
        Dir.mkdir(spec_dir)
      end
    end
  end
end
