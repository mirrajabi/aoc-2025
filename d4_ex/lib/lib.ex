defmodule Lib do
  @directions [{-1, -1}, {-1, 0}, {-1, 1}, {0, -1}, {0, 1}, {1, -1}, {1, 0}, {1, 1}]

  @spec clean_all_possible_rolls(binary()) :: any()
  def clean_all_possible_rolls(grid_str) do
    Stream.unfold(%{total_cleaned: 0, state: grid_str, last_cleaned: -1}, fn acc ->
      if acc.last_cleaned == 0 do
        nil
      else
        result = calculate_adjacent_count(acc.state)

        {acc,
         %{
           total_cleaned: acc.total_cleaned + result.accessible_count,
           state: result.state,
           diff: result.diff,
           last_cleaned: result.accessible_count
         }}
      end
    end)
    |> Enum.to_list()
  end

  @spec calculate_adjacent_count(binary()) :: %{
          accessible_count: any(),
          diff: binary(),
          state: binary()
        }
  def calculate_adjacent_count(grid_str) do
    lines =
      grid_str
      |> String.split("\n", trim: true)

    rows =
      lines
      |> hd()
      |> String.length()

    cols = if rows > 0, do: length(lines), else: 0

    lines =
      lines
      |> Enum.flat_map(&String.graphemes/1)
      |> Enum.map(fn ch -> if ch == "@", do: 1, else: 0 end)

    index_of = fn i, j ->
      if i < 0 || i >= rows || j < 0 || j >= cols, do: nil, else: cols * i + j
    end

    counts =
      for i <- 0..(rows - 1) do
        for j <- 0..(cols - 1) do
          count_filled_adjacents(lines, index_of, i, j)
        end
      end
      |> List.flatten()

    accessible_count =
      Enum.zip(lines, counts)
      |> Enum.reduce(0, fn {sign, count}, acc ->
        if sign != 0 && count < 4, do: acc + 1, else: acc
      end)

    diff =
      Enum.zip(lines, counts)
      |> Enum.map(fn {sign, count} ->
        cond do
          sign == 0 -> "."
          count < 4 -> "x"
          true -> "@"
        end
      end)
      |> Enum.chunk_every(cols)
      |> Enum.join("\n")

    state =
      Enum.zip(lines, counts)
      |> Enum.map(fn {sign, count} ->
        cond do
          sign == 0 -> "."
          count < 4 -> "."
          true -> "@"
        end
      end)
      |> Enum.chunk_every(cols)
      |> Enum.join("\n")

    %{accessible_count: accessible_count, state: state, diff: diff}
  end

  defp count_filled_adjacents(grid, index_of, i, j) do
    Enum.reduce(@directions, 0, fn {di, dj}, acc ->
      oi = i + di
      oj = j + dj

      idx = index_of.(oi, oj)
      if idx == nil, do: acc, else: acc + Enum.at(grid, idx)
    end)
  end
end
