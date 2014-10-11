require 'serverspec'
require 'pathname'
set :backend, :exec

RSpec.configure do |c|
  c.path = '/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin'
end
