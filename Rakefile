# frozen_string_literal: true

require "bundler/gem_tasks"
require "rake/testtask"
require "rb_sys/extensiontask"

begin
  require "standard/rake"
rescue LoadError
  # standard gem not available in cross-compilation environment
end

GEMSPEC = Gem::Specification.load("wreq.gemspec") || abort("Could not load wreq.gemspec")
CRATE_PACKAGE_NAME = "wreq-ruby"

RbSys::ExtensionTask.new(CRATE_PACKAGE_NAME, GEMSPEC) do |ext|
  ext.name = "wreq_ruby"
  ext.ext_dir = "."
  ext.lib_dir = "lib/wreq_ruby"
  ext.cross_compile = true
  # arm64-darwin is built natively on macOS (see .github/workflows/release.yml)
  ext.cross_platform = %w[
    x86_64-linux
    aarch64-linux
  ]

  # Override Ruby version for native gems (keep in sync with wreq.gemspec)
  ext.cross_compiling do |spec|
    spec.required_ruby_version = ">= 3.3", "< 4.1.dev"
  end
end

Rake::TestTask.new(:ruby_test) do |t|
  t.libs << "lib"
  t.libs << "test"
  t.pattern = "test/**/*_test.rb"
end

desc "Run Rust tests"
task :cargo_test do
  sh "cargo test"
end

desc "Format Rust code"
task :fmt do
  sh "cargo fmt"
end

desc "Run Clippy linter"
task :lint do
  sh "cargo clippy -- -D warnings"
end

desc "Build native gem for platform (e.g., rake 'native[x86_64-linux]')"
task :native, [:platform] do |_t, args|
  abort "Usage: rake 'native[PLATFORM]' (e.g., x86_64-linux, aarch64-linux)" unless args[:platform]
  sh "bundle", "exec", "rb-sys-dock", "--platform", args[:platform], "--build"
end

task purge: %i[clean clobber]

desc "Print gem version"
task :version do
  print GEMSPEC.version
end

task test: %i[compile ruby_test cargo_test]
task default: %i[compile test]

# Override bundler's release to push all platform gems
Rake::Task["release"].clear
Rake::Task["release:rubygem_push"].clear if Rake::Task.task_defined?("release:rubygem_push")

desc "Release all platform gems to RubyGems"
task release: ["release:guard_clean", "release:rubygem_push"]

namespace :release do
  task :rubygem_push do
    gem_files = Dir.glob("pkg/#{GEMSPEC.name}-*.gem").sort
    abort "No gems found in pkg/" if gem_files.empty?

    puts "Pushing #{gem_files.length} gem(s):"
    gem_files.each { |f| puts "  - #{File.basename(f)}" }

    gem_files.each do |gem_file|
      puts "\nPushing #{File.basename(gem_file)}..."
      sh "gem push #{gem_file}"
    end
  end
end
