defmodule Main do
  use Application

  def start(_type, _args) do
    File.stream!("./priv/input.txt", :line)
    |> Lib.count_invalid()
    |> IO.inspect()

    Supervisor.start_link([], strategy: :one_for_one)
  end
end
