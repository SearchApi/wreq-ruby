#!/usr/bin/env ruby
# frozen_string_literal: true

# Build a platform-specific gem with pre-compiled native extensions.
#
# Usage: ruby script/build_platform_gem.rb PLATFORM
# Example: ruby script/build_platform_gem.rb arm64-darwin
#
# Expects compiled .bundle/.so files in version-specific directories:
#   lib/wreq_ruby/3.3/wreq_ruby.bundle
#   lib/wreq_ruby/3.4/wreq_ruby.bundle
#   lib/wreq_ruby/4.0/wreq_ruby.bundle

require "rubygems/package"
require "fileutils"

platform = ARGV.fetch(0) { abort "Usage: #{$0} PLATFORM" }

spec = Gem::Specification.load("wreq.gemspec")
spec.platform = Gem::Platform.new(platform)
spec.extensions = []
spec.required_ruby_version = Gem::Requirement.new(">= 3.3", "< 4.1.dev")

# Add version-specific compiled extensions
Dir.glob("lib/wreq_ruby/[0-9]*/*.{bundle,so}").each do |path|
  spec.files << path unless spec.files.include?(path)
end

FileUtils.mkdir_p("pkg")
gem_file = Gem::Package.build(spec)
FileUtils.mv(gem_file, "pkg/")

puts "Built: pkg/#{File.basename(gem_file)}"
