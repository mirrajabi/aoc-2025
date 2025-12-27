defmodule Lib do
  @spec count_invalid(Enumerable.t()) :: %{acc: integer(), count: number()}
  def count_invalid(lines) do
    lines
    |> Stream.map(&Lib.get_rotation/1)
    |> Stream.scan(%{count: 0, acc: 50}, &Lib.accumulate/2)
    |> Enum.at(-1)
  end

  @spec get_rotation(binary()) :: %{num: integer(), rotation: -1 | 1}
  def get_rotation(line) do
    line =
      String.trim_trailing(line)
      |> String.to_charlist()

    [rotation_char | num] = line
    num = String.to_integer("#{num}")

    rotation =
      case rotation_char do
        ?R -> 1
        _ -> -1
      end

    %{rotation: rotation, num: num}
  end

  @spec accumulate(%{num: integer(), rotation: -1 | 1}, %{acc: integer(), count: integer()}) ::
          %{acc: integer(), count: integer()}
  def accumulate(elem, acc) do
    rotation = elem.rotation * elem.num

    %{val: val, revs: revs} = turn(acc.acc, rotation)

    IO.puts(
      "#{String.pad_trailing("#{acc.acc}#{if rotation > 0, do: "+", else: ""}#{rotation}=#{val}", 12)} c: #{acc.count}, revs: #{revs}"
    )

    %{count: acc.count + revs, acc: val}
  end

  defp turn(current, rotation) do
    diff = current + rotation
    revs = floor(abs(diff / 100))
    revs = if current != 0 && diff <= 0, do: revs + 1, else: revs

    val = Integer.mod(diff, 100)

    %{val: val, revs: revs}
  end
end
