# While I did have similar ideas, I couldn't get it to click for the life of me.
# So at the end I took a massive "inspiration" from https://topaz.github.io/paste/#XQAAAQDyAQAAAAAAAAAzHIoib6p4r/McpYgEEgWhHoa5LSRMkVi92ASWXgRJn/5/sndZAYKRSan8HX+LguYN60bC2arqnxhqN26BOsa6g+9VgxnSjCnKbz5ATuB9GqjUma+uNm2MqwSTO0QGXq83D7PtVR3tAFqi1BOhP7r7o7R0k/0YGYtOnIXgVGveK9ydisgunhygX4O43LP47HX3TFw2PmXcISrZUOvjt6B2ByMJcxnntsXTSaZDiGXaIuE1wgNWT/I2v3udkW+PrmyFRvMgV2tVONP0B7s55pxaPXxq6fmzeSU6XRay5L4nVlRgfAl//B+13RAIBBp7VR45rgkn//8vhi8RwuQM1QDEwbaFZgEq9uSCrJ9WG0D2MFkFaXCqwPTTTqVIi3nmwNBp25rZK48kyEuKMmStRTqW5js/xLZM/53teB8TQqV4ykWa27YrdcF+av/2h5bS
defmodule PartTwo do
  def get_largest_area(text) do
    red_tiles =
      String.split(text, "\n")
      |> Enum.map(&parse_line/1)

    # It would be more performant to do this in one iteration maybe but I'll keep it like this for readability
    {min_x, _} = Enum.min_by(red_tiles, fn {x, _} -> x end)
    {max_x, _} = Enum.max_by(red_tiles, fn {x, _} -> x end)
    {_, min_y} = Enum.min_by(red_tiles, fn {_, y} -> y end)
    {_, max_y} = Enum.max_by(red_tiles, fn {_, y} -> y end)
    IO.puts("min_x: #{min_x}, min_y: #{min_y}, max_x: #{max_x}, max_y: #{max_y}")

    draw_image(red_tiles, 0.01, min_x, min_y, max_x, max_y)

    IO.puts("Calculating edges")

    edges =
      red_tiles
      |> Enum.chunk_every(2, 1, :discard)

    edges = edges ++ [[Enum.at(red_tiles, -1), Enum.at(red_tiles, 0)]]

    edges =
      edges
      |> Enum.sort_by(fn [{ax, ay}, {bx, by}] -> abs(ax - bx) + abs(ay - by) end, :desc)

    red_tiles = List.to_tuple(red_tiles)

    IO.puts("Calculating pair areas")

    for i <- 0..(tuple_size(red_tiles) - 2) do
      {xi, yi} = elem(red_tiles, i)

      for j <- (i + 1)..(tuple_size(red_tiles) - 1) do
        {xj, yj} = elem(red_tiles, j)
        {min(xi, xj), min(yi, yj), max(xi, xj), max(yi, yj), area({xi, yi}, {xj, yj})}
      end
    end
    |> List.flatten()
    |> Enum.sort_by(&elem(&1, 4), :desc)
    |> Enum.find_value(&first_non_colliding(edges, &1))
    |> IO.inspect(label: "Largest area")
  end

  def first_non_colliding(edges, {p_min_x, p_min_y, p_max_x, p_max_y, area}) do
    case Enum.find(edges, fn [{gx, gy}, {hx, hy}] ->
           g_min_x = min(gx, hx)
           g_min_y = min(gy, hy)
           g_max_x = max(gx, hx)
           g_max_y = max(gy, hy)

           g_min_x < p_max_x and g_min_y < p_max_y and g_max_x > p_min_x and g_max_y > p_min_y
         end) do
      nil -> area
      _ -> nil
    end
  end

  def parse_line(line) do
    [x, y] =
      String.split(line, ",")
      |> Enum.map(&String.to_integer/1)

    {x, y}
  end

  def area({xi, yi}, {xj, yj}), do: (abs(xi - xj) + 1) * (abs(yi - yj) + 1)

  def draw_image(red_tiles, scale, min_x, min_y, max_x, max_y) do
    padding = 100
    w = scale(max_x - min_x + 1 + padding, scale)
    h = scale(max_y - min_y + 1 + padding, scale)
    # Padding halved scaled
    phs = div(padding, 2)
    img = ExPng.Image.new(w, h)

    img =
      red_tiles
      |> Enum.chunk_every(2, 1, :discard)
      |> Enum.reduce(img, fn [{x0, y0}, {x1, y1}], img ->
        ExPng.Image.line(
          img,
          {scale(x0 - min_x + phs, scale), scale(y0 - min_y + phs, scale)},
          {scale(x1 - min_x + phs, scale), scale(y1 - min_y + phs, scale)},
          ExPng.Color.rgb(190, 80, 30)
        )
      end)

    ExPng.Image.to_file(img, "./output.png")
  end

  def scale(n, scale), do: trunc(ceil(n * scale))
end
