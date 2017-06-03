defmodule Elixirversion.Web.LogController do
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
    timestamp = round(:os.system_time / 1000 / 1000)
    id = timestamp
    score = timestamp
    member = to_string id
    form = Map.take params, ~w[from to]
    entity = form |> Map.merge(%{"id": id, "created_at": timestamp, "updated_at": timestamp})

    with {:ok, _} <- Redix.command(:redix, ["zadd", @dict_log_key, score, member]),
         {:ok, entity_json} <- Poison.encode(entity),
         {:ok, _} <- Redix.command(:redix, ["hset", @dict_log_data_key, member, entity_json]) do
      json conn, %{ok: true}
    else
      _ -> conn |> put_status(500) |> json([])
    end
  end

  def delete(conn, %{"id" => id} = _params) do
    with {:ok, i} <- Redix.command(:redix, ["hdel", @dict_log_data_key, id]) do
      if i == 0 do
         conn |> put_status(400) |> json(%{message: "id为#{id}的日志不存在!"})
      else
         json conn, %{ok: true}
      end
    else
      _ -> conn |> put_status(500) |> json([])
    end
  end

  defp json_raw(conn, content) when is_binary(content) do
    conn
    |> put_resp_content_type("application/json")
    |> send_resp(200, content)
  end

  defp uuid do
    UUID.uuid4() |> String.replace("-", "")
  end
end
