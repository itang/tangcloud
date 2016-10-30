defmodule Elixirversion.Plugs.RuntimePlug do
 import Plug.Conn

  def init(options), do: options

  def call(conn, _opts) do
    Plug.Conn.put_resp_header(conn, "x-runtime", "todo-xxx")
  end
end
