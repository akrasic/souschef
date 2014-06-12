module Souschef
  class Template
    # Base class containing reusable functions
    class Base
      private

      # Private - Return location of file inside data/ directory
      #
      # Return String
      def datafile_path(file)
        p = "../../../data/#{file}"
        File.expand_path(p, __FILE__)
      end

      # Private - Return path to the file inside cookbook directory
      #
      # Return String
      def cookbook_file_path(cookbook, file)
        File.join(Dir.pwd, cookbook, file)
      end

      # Private - Write data to the desired file
      #
      # Returns nil
      def write_file(file, data)
        File.open(file, 'w') { |fd| fd.write(data) }
      end
    end
  end
end
