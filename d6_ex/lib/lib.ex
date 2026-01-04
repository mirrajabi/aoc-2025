defmodule Lib do
  def calculate_sum(text) do
    rows = String.split(text, "\n")

    rows
    |> Enum.map(&split_line/1)
    |> Enum.zip()
    |> Enum.map(&calculate_column/1)
    |> Enum.sum()
  end

  def calculate_sum_vertically(text) do
    text
    |> String.split("\n")
    # The space at the end is a hack to include the final batch in the sum
    |> Enum.map(fn str -> String.graphemes(str) ++ [" "] end)
    |> Enum.zip()
    |> Enum.reduce(%{sign: nil, sum: 0, batch: -1}, &accumulate_vertically/2)
    |> Map.get(:sum)
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

    nums =
      col
      |> Enum.take(len_col - 1)
      |> Enum.map(&String.to_integer/1)

    op = if sign == "+", do: fn acc, num -> acc + num end, else: fn acc, num -> acc * num end

    nums |> Enum.reduce(op)
  end

  def accumulate_vertically(col, acc) do
    col = Tuple.to_list(col)

    if Enum.all?(col, fn x -> x == " " end) do
      %{acc | batch: -1, sum: acc.sum + acc.batch}
    else
      len_col = length(col)
      sign = Enum.at(col, -1)
      sign = if sign == " ", do: acc.sign, else: sign

      n =
        col
        |> Enum.take(len_col - 1)
        |> List.to_string()
        |> String.trim()
        |> String.to_integer()

      op = if sign == "+", do: fn acc, num -> acc + num end, else: fn acc, num -> acc * num end

      batch = if acc.batch == -1, do: n, else: op.(acc.batch, n)
      %{acc | batch: batch, sign: sign}
    end
  end
end
