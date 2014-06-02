module Souschef
  class Print

    def self.info(msg)
      puts "~> #{msg}".colorize(:yellow)
    end

    def self.error(msg)
      puts "#{msg}".colorize(:red)
    end

    def self.warning(msg)
      puts "#{msg}".colorize(:orange)
    end

  end
end
