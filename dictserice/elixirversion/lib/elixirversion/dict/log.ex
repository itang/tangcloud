defmodule Elixirversion.Dict.Log do
  @moduledoc false

  @dict_log_key "tc:dict:log"
  @dict_log_data_key "tc:dict:log:data"

  def get_all_as_json do
    with {:ok, ret} <- Redix.command(:redix, ["HVALS",  @dict_log_data_key]) do
      ret = "[#{Enum.join(ret, ", ")}]"
      {:ok, ret}
    else
      err -> err
    end
  end

  def create(%{:from => from, :to => to} = log_form) do
    timestamp = round(:os.system_time / 1000 / 1000)
    id = timestamp
    score = timestamp
    member = to_string id
    form = %{"from" => from, "to" => to}
    entity = form |> Map.merge(%{"id": id, "created_at": timestamp, "updated_at": timestamp})

    with {:ok, _} <- Redix.command(:redix, ["zadd", @dict_log_key, score, member]),
         {:ok, entity_json} <- Poison.encode(entity),
         {:ok, _} <- Redix.command(:redix, ["hset", @dict_log_data_key, member, entity_json]) do
      {:ok, ""}
    else
      err -> err
    end
  end

  def delete(id) when is_binary(id) do
    with {:ok, i} <- Redix.command(:redix, ["hdel", @dict_log_data_key, id]) do
      if i == 0 do
        {:error, "id为#{id}的日志不存在!"}
      else
         {:ok, ""}
      end
    else
      err -> err
    end
  end
end
