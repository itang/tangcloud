defmodule ElixirversionWeb.IndexController do
  use ElixirversionWeb, :controller

  def index(conn, _params) do
    redirect conn, to: "/index.html"
  end
end
