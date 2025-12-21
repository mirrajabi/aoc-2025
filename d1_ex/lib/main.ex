defmodule Main do
  use Application

  def start(_type, _args) do
    File.stream!("./priv/input.txt", :line)
    |> Stream.map(fn line ->
        line = String.trim_trailing(line)
          |> String.to_charlist()
        [rotation_char | num] = line
        num = String.to_integer("#{num}")
        rotation = case rotation_char do
          ?R -> 1
          _ -> -1
        end
        %{rotation: rotation, num: num}
    end)
    |> Stream.scan(%{count: 0, acc: 50}, fn elem, acc ->
      val = acc.acc
       |> Kernel.+(elem.rotation * elem.num)
       |> rem(100)

      val = if val < 0, do: val + 100, else: val

      count = acc.count
      count = if val == 0, do: count + 1, else: count

      %{count: count, acc: val}
    end)
    |> Enum.at(-1)
    |> IO.inspect()



    Supervisor.start_link([], strategy: :one_for_one)
  end

  {:ok, }
end
