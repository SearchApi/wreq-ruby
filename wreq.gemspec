# frozen_string_literal: true

Gem::Specification.new do |spec|
  spec.name = "wreq"
  # RubyGems integrates and expects `cargo`.
  # Read more about
  # [Gem::Ext::CargoBuilder](https://github.com/rubygems/rubygems/blob/v3.5.23/lib/rubygems/ext/cargo_builder.rb)
  #
  # RubyGems accepts a fourth version segment, which Cargo does not.
  spec.version = File.read(File.join(__dir__, "VERSION")).strip
  spec.authors = ["SearchApi"]
  spec.email = ["support@searchapi.io"]
  spec.summary = "Ruby bindings for wreq, an HTTP client with TLS/HTTP2 browser fingerprinting"
  spec.description = "An easy and powerful Ruby HTTP client with advanced browser fingerprinting " \
                     "that accurately emulates Chrome, Edge, Firefox, Safari, Opera, and OkHttp " \
                     "with precise TLS/HTTP2 signatures. Powered by wreq (Rust) and BoringSSL."
  spec.homepage = "https://github.com/SearchApi/wreq-ruby"
  spec.license = "Apache-2.0"
  spec.metadata = {
    "bug_tracker_uri" => "https://github.com/SearchApi/wreq-ruby/issues",
    "changelog_uri" => "https://github.com/SearchApi/wreq-ruby/releases",
    "documentation_uri" => "https://github.com/SearchApi/wreq-ruby#readme",
    "homepage_uri" => spec.homepage,
    "source_code_uri" => "https://github.com/SearchApi/wreq-ruby",
    "rubygems_mfa_required" => "true"
  }

  # Specify which files should be added to a source release gem when we release wreq Ruby gem.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir.chdir(__dir__) do
    git_output = `git ls-files -z`
    git_output.force_encoding("UTF-8")
    git_output.split("\x0").reject do |f|
      f.start_with?(*%w[gems/ pkg/ target/ tmp/ .git]) ||
        f.match?(/\.gem$/) || # Exclude gem files
        f.match?(/^wreq-.*\.gem$/)  # Exclude any wreq gem files
    end
  end

  spec.require_paths = ["lib"]

  spec.extensions = ["./extconf.rb"]

  # Exclude non-Ruby files from RDoc to prevent parsing errors
  spec.rdoc_options = ["--exclude", "Cargo\\..*", "--exclude", "\\.rs$"]

  spec.requirements = ["Rust >= 1.98"]
  # use a Ruby version which:
  # - supports Rubygems with the ability of compilation of Rust gem
  # - not end of life
  #
  # keep in sync with `Rakefile`.
  spec.required_ruby_version = ">= 3.3"

  # intentionally skipping rb_sys gem because newer Rubygems will be present
end
