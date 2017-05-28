defmodule Elixirversion.Web.PingController do
  use Elixirversion.Web, :controller

  def ping(conn, _params) do
    json conn, %{message: "pong"}
  end
end
