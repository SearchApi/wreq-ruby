require "mkmf"
# We use rb_sys for makefile generation only.
# We can use `RB_SYS_CARGO_PROFILE` to choose Cargo profile
# Read more https://github.com/oxidize-rb/rb-sys/blob/main/gem/README.md
require "rb_sys/mkmf"

create_rust_makefile("wreq_ruby/wreq_ruby")
