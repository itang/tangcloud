defmodule ElixirversionWeb.LogController do
  use ElixirversionWeb, :controller
  alias Elixirversion.Datatype.Result
  alias Elixirversion.Dict.Log, as: DictLog

  def list(conn, _params) do
    with {:ok, ret} <- DictLog.get_all_as_json() do
      json_raw_result conn, true, ret
    else
      _ -> conn |> put_status(500) |> json(Result.error())
    end
  end

  def create(conn, %{"from" => from, "to" => to} = params) do
    with {:ok, _} <- DictLog.create(%{:from => from, :to => to}) do
      json conn, Result.ok()
    else
      _ -> conn |> put_status(500) |> json(Result.error())
    end
  end

  def delete(conn, %{"id" => id} = _params) do
    with {:ok, _} <- DictLog.delete(id) do
      json conn, Result.ok()
    else
      _ -> conn |> put_status(500) |> json([])
    end
  end

  def json_raw(conn, content) when is_binary(content) do
    conn
    |> put_resp_content_type("application/json")
    |> send_resp(200, content)
  end

  def json_raw_result(conn, ok, data) when is_boolean(ok) and is_binary(data) do
    result_raw = ~s({"ok": #{ok}, "data": #{wrap_raw(data)}})
    json_raw(conn, result_raw)
  end

  defp wrap_raw(s) when is_binary(s) do
    if String.starts_with?(s, "[") or String.starts_with?(s, "{") do
        s
    else
        ~s("#{s}")
    end
  end

  defp uuid do
    UUID.uuid4() |> String.replace("-", "")
  end
end
