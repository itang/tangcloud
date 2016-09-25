defmodule Elixirversion.IndexController do
  use Elixirversion.Web, :controller

  def index(conn, _params) do
    redirect conn, to: "/index.html"
  end
end
