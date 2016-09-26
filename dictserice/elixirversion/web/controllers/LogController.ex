defmodule Elixirversion.LogController do
  use Elixirversion.Web, :controller

  def list(conn, _params) do
    {:ok, redis} = Redix.start_link()

    {:ok, ret} = Redix.command(redis, ~w(HVALS tc:dict:log:data))

    IO.inspect ret

    json conn,  Enum.map(ret, fn it ->
      Poison.Parser.parse!(it)
    end)
  end
end
