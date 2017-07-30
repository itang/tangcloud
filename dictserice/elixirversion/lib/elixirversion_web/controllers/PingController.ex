defmodule ElixirversionWeb.PingController do
  use ElixirversionWeb, :controller

  def ping(conn, _params) do
    json conn, %{message: "pong", from: "elixirversion"}
  end
end
