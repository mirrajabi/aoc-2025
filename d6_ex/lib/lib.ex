defmodule Lib do
  def calculate_sum(text) do
    rows = String.split(text, "\n")

    rows
    |> Enum.map(&split_line/1)
    |> Enum.zip()
    |> Enum.map(&calculate_column/1)
    |> Enum.sum()
  end

  def split_line(line) do
    line
    |> String.split(" ")
    |> Enum.filter(fn s -> String.trim(s) != "" end)
  end

  def calculate_column(col) do
    col = Tuple.to_list(col)
    len_col = length(col)
    sign = Enum.at(col, -1)
    nums = col |> Enum.take(len_col - 1) |> Enum.map(&String.to_integer/1)

    op = if sign == "+", do: fn acc, num -> acc + num end, else: fn acc, num -> acc * num end

    nums |> Enum.reduce(op)
  end
end
