defmodule LibTest do
  use ExUnit.Case
  doctest Lib

  test "sample case" do
    out =
      """
      123 328  51 64
       45 64  387 23
        6 98  215 314
      *   +   *   +
      """
      |> String.trim_trailing()
      |> Lib.calculate_sum()

    assert out == 4_277_556
  end

  test "personal case" do
    # Total # of windows = len - count + 1
    out =
      File.read!("./priv/input.txt")
      |> Lib.calculate_sum()

    assert out == 4_309_240_495_780
  end

  test "sample case pt2" do
    out =
      File.read!("./priv/sample.txt")
      |> Lib.calculate_sum_vertically()

    assert out == 3_263_827
  end

  test "personal case pt2" do
    # Total # of windows = len - count + 1
    out =
      File.read!("./priv/input.txt")
      |> Lib.calculate_sum_vertically()

    assert out == 9_170_286_552_289
  end
end
