module Souschef
  # Includes slave functions that do all the work
  class Berkshelf
    attr_accessor :opts

    def initialize(opts)
      @opts = opts
    end

    # Public - Tell Berkshelf to create a cookbook
    #
    # Returns nil
    def berks_create
      Souschef::Print.info "Creating cookbook structure"
      check_cookbook_dir
      i, o, e, w = Open3.popen3(which_berks, 'cookbook', @opts[:cookbook])
      i.close
      e.close
      print_open3_stdout(o) if @opts[:verbose]
      raise 'Berks failed' unless w.value == 0
    end

    private

    # Private - Obtain berks executable full path
    #
    # Returns String
    def which_berks
      i, o, e, w = Open3.popen3('which berks')
      if w.value.to_i > 0
        raise "berks executable not found on system"
      end
      o.read.chomp
    end

    # Private - Print out Open3 STDOUT stream
    #
    # stdout - Open3 stdout stream
    #
    # Return nil
    def print_open3_stdout(stdout)
      stdout.read.split("\n").each {|msg| puts msg.colorize(:green) }
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


    # Private - Check if the cookbook directory doesn't exist
    #
    # Returns nil
    def check_cookbook_dir
      if File.directory?("#{Dir.pwd}/#{@opts[:cookbook]}")
        raise "Cookbook directory #{@opts[:cookbook]}  already exists"
      end
    end

  end
end
