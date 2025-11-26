#!/usr/bin/env ruby

# Tests Thread.kill interrupt support for wreq HTTP operations.
# All blocking operations should respond to thread interruption.

require_relative "../lib/wreq"

HANGING_URL = "http://10.255.255.1:12345/" # Non-routable, hangs forever
SLOW_BODY_URL = "https://httpbin.io/drip?duration=5&numbytes=5" # 5s slow body

def test_interrupt(name, kill_after: 2, max_wait: 5)
  print "#{name}: "

  start = Time.now
  thread = Thread.new { yield }

  sleep kill_after
  thread.kill
  killed = thread.join(max_wait)

  elapsed = Time.now - start

  if killed && elapsed < (kill_after + max_wait)
    puts "PASS (killed in #{elapsed.round(1)}s)"
    :pass
  else
    puts "FAIL (still alive after #{elapsed.round(1)}s)"
    begin
      Thread.kill(thread)
    rescue
      nil
    end
    :fail
  end
end

puts "Thread Interrupt Test"
puts "=" * 40

results = []

# Test 1: Connection phase (non-routable IP)
results << test_interrupt("Connect phase") {
  begin
    Wreq.get(HANGING_URL)
  rescue
    nil
  end
}

# Test 2: Connection phase with timeout
results << test_interrupt("Connect + timeout") {
  begin
    Wreq.get(HANGING_URL, timeout: 60)
  rescue
    nil
  end
}

# Test 3: Body reading phase (slow drip endpoint)
results << test_interrupt("Body reading") {
  resp = Wreq.get(SLOW_BODY_URL)
  begin
    resp.text
  rescue
    nil
  end
}

# Test 4: Body streaming phase
results << test_interrupt("Body streaming") {
  resp = Wreq.get(SLOW_BODY_URL)
  begin
    resp.chunks { |chunk| chunk }
  rescue
    nil
  end
}

puts "=" * 40
passed = results.count(:pass)
puts "#{passed}/#{results.size} tests passed"
exit(results.all?(:pass) ? 0 : 1)
