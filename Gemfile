source "https://rubygems.org"

# Includes runtime dependencies from wreq.gemspec
gemspec

group :development, :test do
  gem "rake", ">= 13.2"
  gem "rb_sys", "~> 0.9.110" # for Makefile generation in extconf.rb
  gem "rake-compiler", "~> 1.2.9" # to build a debug build
  gem "minitest", "~> 5.25.0" # test library
  gem "minitest-reporters", "~> 1.7.1" # better test output
  gem "activesupport", "~> 8.0.1" # testing support
  gem "standard", "~> 1.52" # linter with pre-specified rules
  gem "redcarpet", "~> 3.6" # for documentation markdown parsing
  gem "ruby-lsp", "~> 0.22" # Ruby language server for IDE support
  gem "racc", "~> 1.8.1" # for parsing
  gem "observer", "~> 0.1.2" # for parsing
end
