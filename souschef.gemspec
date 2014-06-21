# coding: utf-8
lib = File.expand_path('../lib', __FILE__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)
require 'souschef/version'

Gem::Specification.new do |spec|
  spec.name          = 'souschef'
  spec.version       = Souschef::VERSION
  spec.authors       = ['Antun Krasic']
  spec.email         = ['antun@martuna.co']
  spec.summary       = %q(Chef cookbook helper)
  spec.description   = %q(Chef helper to get you started fast)
  spec.homepage      = 'http://interwebz.com/404/'
  spec.license       = 'MIT'

  spec.files         = `git ls-files -z`.split("\x0")
  spec.executables   = spec.files.grep(/^bin\//) { |f| File.basename(f) }
  spec.test_files    = spec.files.grep(/^(test|spec|features)\//)
  spec.require_paths = ['lib']

  spec.add_development_dependency 'bundler', '~> 1.6'
  spec.add_development_dependency 'rake'
  spec.add_dependency 'trollop'
  spec.add_dependency 'ruth'
  spec.add_dependency 'colorize'
  spec.add_dependency 'chef'
  spec.add_dependency 'berkshelf'
  spec.add_dependency 'test-kitchen'
  spec.add_dependency 'rubocop'
  spec.add_dependency 'foodcritic'
  spec.add_dependency 'chefspec', '~> 4'
  spec.add_dependency 'serverspec'
end
