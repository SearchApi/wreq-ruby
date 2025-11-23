#!/usr/bin/env ruby
# frozen_string_literal: true

# Demonstrates that Thread.kill cannot interrupt wreq requests.
# Fix: Register unblock function with rb_thread_call_without_gvl

require_relative "../lib/wreq"

HANGING_URL = "http://10.255.255.1:12345/" # Non-routable, hangs forever

def test_thread_kill(name, timeout: nil)
  print "#{name}: "

  start = Time.now
  thread = Thread.new do
    Wreq.get(HANGING_URL, timeout: timeout) rescue nil
  end

  sleep 2
  thread.kill
  killed = thread.join(3) # Wait max 3s for thread to die

  elapsed = Time.now - start
  alive = thread.alive?

  if killed && elapsed < 4
    puts "PASS (killed in #{elapsed.round(1)}s)"
    :pass
  elsif killed && !alive
    puts "TIMEOUT (exited after #{elapsed.round(1)}s, not via kill)"
    :timeout_exit
  else
    puts "FAIL (still alive after #{elapsed.round(1)}s)"
    Thread.kill(thread) rescue nil
    :fail
  end
end

puts "Thread Interrupt Test"
puts "=" * 40

r1 = test_thread_kill("No timeout")
r2 = test_thread_kill("With timeout", timeout: 5)

puts "=" * 40
exit(r1 == :pass && r2 == :pass ? 0 : 1)
