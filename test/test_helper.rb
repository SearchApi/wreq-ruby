# frozen_string_literal: true
#
# Run httpbin server: `docker run -d -p 8080:80 --name httpbin kennethreitz/httpbin`

$LOAD_PATH.unshift File.expand_path("../lib", __dir__)
require "wreq"

require "minitest/autorun"
