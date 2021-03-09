require 'aruba/cucumber'

Aruba.configure do |config|
  config.exit_timeout = 5 * 60
  config.home_directory = ENV['HOME']
end
