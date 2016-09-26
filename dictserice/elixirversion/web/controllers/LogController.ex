defmodule Elixirversion.LogController do
  use Elixirversion.Web, :controller

  def list(conn, _params) do
    {:ok, redis} = Redix.start_link()

    {:ok, ret} = Redix.command(redis, ~w(HVALS tc:dict:log:data))

    IO.inspect ret

    # json conn,  Enum.map(ret, fn it ->
    #   Poison.Parser.parse!(it)
    # end)

    ret = "[#{Enum.join(ret, ", ")}]"
    jsonr conn, ret
  end

  defp jsonr(conn, content) when is_binary(content) do
    conn
    |> put_resp_content_type("application/json")
    |> send_resp(200, content)
  end
end
