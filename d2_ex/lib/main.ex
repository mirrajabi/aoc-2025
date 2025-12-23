defmodule Main do
  use Application

  def start(_type, _args) do
    File.read!("./priv/input.txt")
    |> String.split(",")
    |> Enum.map(fn item ->
      parts = String.split(item, "-")
      first = String.to_integer(Enum.at(parts, 0))
      last = String.to_integer(Enum.at(parts, 1)) + 1
      Range.new(first, last)
     end)
    |> Enum.flat_map(&Range.to_list/1)
    |> Enum.filter(fn f ->
      len_digits = trunc(floor(:math.log10(f)) + 1)
      upper_half = trunc(f / :math.pow(10, len_digits / 2))
      repeated = upper_half + upper_half * trunc(:math.pow(10, len_digits / 2))
      # IO.puts("#{f}, #{upper_half}, #{repeated}, eq: #{f == repeated}")
      rem(len_digits, 2) != 1 && f == repeated
    end)
    |> Enum.sum()
    |> IO.inspect()

    Supervisor.start_link([], strategy: :one_for_one)
  end

  {:ok, }
end
