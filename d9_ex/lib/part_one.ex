defmodule PartOne do
  def get_largest_area(text) do
    red_tiles =
      String.split(text, "\n")
      |> Enum.map(&parse_line/1)
      |> List.to_tuple()

    for i <- 0..(tuple_size(red_tiles) - 2) do
      {xi, yi} = elem(red_tiles, i)

      for j <- (i + 1)..(tuple_size(red_tiles) - 1) do
        {xj, yj} = elem(red_tiles, j)

        {dx, dy} = {abs(xi - xj) + 1, abs(yi - yj) + 1}
        dx * dy
      end
    end
    |> List.flatten()
    |> Enum.max()
  end

  def parse_line(line) do
    [x, y] =
      String.split(line, ",")
      |> Enum.map(&String.to_integer/1)

    {x, y}
  end
end
