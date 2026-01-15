defmodule PartTwoTest do
  use ExUnit.Case

  test "sample case" do
    out =
      File.read!("./priv/sample.txt")
      |> PartTwo.get_largest_area()

    assert out == 24
  end

  test "personal case" do
    time_start_nano = :erlang.monotonic_time()

    out =
      File.read!("./priv/input.txt")
      |> PartTwo.get_largest_area()

    time_end_nano = :erlang.monotonic_time()
    time_diff_milli = div(time_end_nano - time_start_nano, 1_000_000)
    assert out == 1_539_238_860
    IO.puts("Time taken (ms): #{time_diff_milli}")
  end
end
