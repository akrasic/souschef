module Souschef
  # Prit out colorized information
  class Print
    # Public - Print info messate
    #
    # msg - String message
    #
    # Returns nil
    def self.info(msg)
      puts "~> #{msg}".colorize(:yellow)
    end

    # Public - Print error messate
    #
    # msg - String message
    #
    # Returns nil
    def self.error(msg)
      puts "#{msg}".colorize(:red)
    end

    # Public - Print warning  messate
    #
    # msg - String message
    #
    # Returns nil
    def self.warning(msg)
      puts "#{msg}".colorize(:orange)
    end
  end
end
