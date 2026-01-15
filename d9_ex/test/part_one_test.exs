defmodule PartOneTest do
  use ExUnit.Case

  test "sample case" do
    out =
      File.read!("./priv/sample.txt")
      |> PartOne.get_largest_area()

    assert out == 50
  end

  test "personal case" do
    out =
      File.read!("./priv/input.txt")
      |> PartOne.get_largest_area()

    assert out == 4_759_531_084
  end
end
