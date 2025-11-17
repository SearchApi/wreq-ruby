# frozen_string_literal: true

require "bundler/gem_tasks"
require "rake/testtask"
require "rb_sys/extensiontask"
require "standard/rake"

GEMSPEC = Gem::Specification.load("wreq.gemspec")
CRATE_PACKAGE_NAME = "wreq-ruby"

RbSys::ExtensionTask.new(CRATE_PACKAGE_NAME, GEMSPEC) do |ext|
  ext.name = "wreq_ruby"
  ext.ext_dir = "."
  ext.lib_dir = "lib/wreq_ruby"

  ext.cross_compile = true
  ext.cross_platform = %w[
    x86_64-linux
    arm64-linux
    arm64-darwin
    arm64-darwin23
  ]

  # Override Ruby version requirement for native gems
  # This prevents automatic constraint to the build Ruby version (e.g., >= 3.3, < 3.4.dev)
  ext.cross_compiling do |gem_spec|
    # keep in sync with wreq.gemspec
    gem_spec.required_ruby_version = ">= 3.2", "< 3.5.dev"
  end
end

Rake::Task[:test].prerequisites << :compile

Rake::TestTask.new do |t|
  t.libs << "lib"
  t.libs << "test"
  t.pattern = "test/**/*_test.rb"
end

namespace :doc do
  task default: %i[rustdoc yard]

  desc "Generate YARD documentation"
  task :yard do
    run("bundle exec yard doc --plugin rustdoc -- lib tmp/doc/wreq_ruby.json")
  end

  desc "Generate Rust documentation as JSON"
  task :rustdoc do
    target_dir = "tmp"
    ext_dir = "wreq-ruby"
    run(<<~CMD)
      cargo +nightly-2025-03-18 rustdoc \
        --target-dir #{target_dir} \
        --package #{ext_dir} \
        -Zunstable-options \
        --output-format json \
        --lib \
        -- \
        --document-private-items
    CMD
  end

  def run(cmd)
    system(cmd)
    fail if $? != 0
  end
end

task doc: "doc:default"
task default: %i[compile test standard]
task purge: %i[clean clobber]

desc "report gem version"
task :version do
  print GEMSPEC.version
end

Rake::Task["release"].clear # clear the existing release task to allow override
Rake::Task["release:rubygem_push"].clear if Rake::Task.task_defined?("release:rubygem_push")

# overrides bundler's default rake release task
# we removed build and git tagging steps. Read more in ``./.github/workflows/release-ruby.yml`
# read more https://github.com/rubygems/rubygems/blob/master/bundler/lib/bundler/gem_helper.rb
desc "Multi-arch release"
task release: [
  "release:guard_clean",
  "release:rubygem_push"
]

namespace :release do
  desc "Push all gems to RubyGems"
  task :rubygem_push do
    # Push all gem files (source + platform-specific) like Nokogiri does
    # Bundler's built_gem_path only returns the last modified gem, so we glob all gems instead
    gem_files = Gem::Util.glob_files_in_dir("#{GEMSPEC.name}-*.gem", "pkg").sort

    if gem_files.empty?
      abort "No gem files found in pkg/"
    end

    puts "Found #{gem_files.length} gem(s) to push:"
    gem_files.each { |f| puts "  - #{File.basename(f)}" }

    gem_files.each do |gem_file|
      puts "\nPushing #{File.basename(gem_file)}..."
      sh "gem push #{gem_file}"
    end
  end
end