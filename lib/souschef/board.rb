module Souschef
  # Creates the needed cookbook
  class Board

    def initialize(opts)
      super(opts)
    end

    # Public - Creates cookbook and adjuss the needed files
    #
    # Returns nil
    def begin
      berks_create
    end
  end
end
