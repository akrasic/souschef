module Souschef
  # Includes slave functions that do all the work
  class Berkshelf
    attr_accessor :opts, :cookbook_dir

    def initialize(opts)
      @opts = opts
    end

    # Public - Tell Berkshelf to create a cookbook
    #
    # Returns nil
    def berks_create
      remove_old_readme
      Souschef::Print.info 'Creating cookbook structure' if @opts[:verbose]

      i, o, e, w = Open3.popen3(berks_cmd)
      print_open3_stdout(o) if @opts[:verbose]
      remove_vagrantfile
      print_open3_stdout(e)
      i.close
      fail 'Berks failed' unless w.value == 0
    end

    private

    # Private - Returns berkshelf command depending on the path
    #
    # Returns String
    def berks_cmd
      if @opts[:path] == Dir.pwd
        "yes | #{which_berks} cookbook #{@opts[:cookbook]} ."
      else
        path = @opts[:path].gsub("#{Dir.pwd}/", '')
        "yes | #{which_berks} cookbook #{@opts[:cookbook]} #{path}"
      end
    end

    # Private - Remove README from cookbook dir
    #
    # Returns nil
    def remove_old_readme
      readme = File.join(@opts[:path], 'README.md')
      File.delete(readme) if File.exist?(readme)
    end

    # Private - Remove README from cookbook dir
    #
    # Returns nil
    def remove_vagrantfile
      vagrantfile = File.join(@opts[:path], 'Vagrantfile')
      File.delete(vagrantfile) if File.exist?(vagrantfile)
    end

    # Private - Obtain berks executable full path
    #
    # Returns String
    def which_berks
      i, o, e, w = Open3.popen3('which berks')
      fail 'berks executable not found on system' if w.value.to_i > 0
      i.close
      e.close
      o.read.chomp
    end

    # Private - Print out Open3 STDOUT stream
    #
    # stdout - Open3 stdout stream
    #
    # Return nil
    def print_open3_stdout(stdout)
      stdout.read.split("\n").each { |msg| puts msg.colorize(:green) }
    end

    # Private - Print out a colorized message
    #
    # msg - String message
    #
    # Returns nil
    def colour(msg)
      puts msg.colorize(:red)
    end

    # Private - Print out info message
    #
    # msg - String message
    #
    # Returns nil
    def info_msg(msg)
      puts msg.colorize(:yellow)
    end
  end
end
