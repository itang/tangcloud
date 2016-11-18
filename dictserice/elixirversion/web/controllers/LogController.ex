defmodule Elixirversion.LogController do
  use Elixirversion.Web, :controller

  @dict_log_key "tc:dict:log"
  @dict_log_data_key "tc:dict:log:data"

  def list(conn, _params) do
    with {:ok, redis} <- Redix.start_link(),
         IO.puts("DEBUG: start link for redis"),
         {:ok, ret} <- Redix.command(redis, ~w(HVALS tc:dict:log:data)) do

        IO.inspect ret

        # json conn,  Enum.map(ret, fn it ->
        #   Poison.Parser.parse!(it)
        # end)

        ret = "[#{Enum.join(ret, ", ")}]"
        jsonr conn, ret
    else
        _ -> conn |> put_status(500) |> json([])
    end
  end

  def create(conn, %{"from" => _, "to" => _} = params) do
    IO.inspect params

    id = :os.system_time
    score = id
    member = to_string id
    entity = Map.take params, ["from", "to"]

    with {:ok, redis} <- Redix.start_link(),
         {:ok, ret} <- Redix.command(redis, ["zadd", @dict_log_key, score, member]),
         {:ok, entity_json} <- Poison.encode(entity),
         {:ok, ret2} <- Redix.command(redis, ["hset", @dict_log_data_key, member, entity_json]) do
        json conn, %{ok: true}
    else
        _ -> conn |> put_status(500) |> json([])
    end
  end

  defp jsonr(conn, content) when is_binary(content) do
    conn
    |> put_resp_content_type("application/json")
    |> send_resp(200, content)
  end
end
