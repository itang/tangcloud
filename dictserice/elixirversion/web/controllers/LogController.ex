defmodule Elixirversion.LogController do
  use Elixirversion.Web, :controller

  @dict_log_key "tc:dict:log"
  @dict_log_data_key "tc:dict:log:data"

  def list(conn, _params) do
    with {:ok, ret} <- Redix.command(:redix, ~w(HVALS tc:dict:log:data)) do
      ret = "[#{Enum.join(ret, ", ")}]"
      json_raw conn, ret
    else
      _ -> conn |> put_status(500) |> json([])
    end
  end

  def create(conn, %{"from" => _, "to" => _} = params) do
    id = :os.system_time
    score = id
    member = to_string id
    entity = Map.take params, ["from", "to"]

    with {:ok, _} <- Redix.command(:redix, ["zadd", @dict_log_key, score, member]),
         {:ok, entity_json} <- Poison.encode(entity),
         {:ok, _} <- Redix.command(:redis, ["hset", @dict_log_data_key, member, entity_json]) do
      json conn, %{ok: true}
    else
      _ -> conn |> put_status(500) |> json([])
    end
  end

  defp json_raw(conn, content) when is_binary(content) do
    conn
    |> put_resp_content_type("application/json")
    |> send_resp(200, content)
  end
end
