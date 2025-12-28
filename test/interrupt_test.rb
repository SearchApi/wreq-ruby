require "test_helper"

class ThreadInterruptTest < Minitest::Test
  HANGING_URL = "http://10.255.255.1:12345/" # Non-routable, hangs forever
  SLOW_BODY_URL = "https://httpbin.io/drip?duration=5&numbytes=5" # 5s slow body

  def test_connect_phase_interrupt
    thread = Thread.new do
      begin
        Wreq.get(HANGING_URL)
      rescue => e
        e
      end
    end
    sleep 2
    thread.kill
    result = thread.join(5)
    assert result, "Thread should be killed and joined"
  end

  def test_connect_with_timeout_interrupt
    thread = Thread.new do
      begin
        Wreq.get(HANGING_URL, timeout: 60)
      rescue => e
        e
      end
    end
    sleep 2
    thread.kill
    result = thread.join(5)
    assert result, "Thread should be killed and joined"
  end

  def test_body_reading_interrupt
    thread = Thread.new do
      resp = Wreq.get(SLOW_BODY_URL)
      begin
        resp.text
      rescue => e
        e
      end
    end
    sleep 2
    thread.kill
    result = thread.join(5)
    assert result, "Thread should be killed and joined"
  end

  def test_body_streaming_interrupt
    thread = Thread.new do
      resp = Wreq.get(SLOW_BODY_URL)
      begin
        resp.chunks { |chunk| chunk }
      rescue => e
        e
      end
    end
    sleep 2
    thread.kill
    result = thread.join(5)
    assert result, "Thread should be killed and joined"
  end
end
