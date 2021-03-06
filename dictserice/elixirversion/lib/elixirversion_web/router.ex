defmodule ElixirversionWeb.Router do
  use ElixirversionWeb, :router

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_flash
    plug :protect_from_forgery
    plug :put_secure_browser_headers
  end

  scope "/", ElixirversionWeb do
    pipe_through :browser # Use the default browser stack

    get "/", IndexController, :index
  end

  pipeline :api do
    plug :accepts, ["json"]
    plug ElixirversionWeb.Plugs.RuntimePlug, []
  end

  scope "/api", ElixirversionWeb do
    pipe_through :api

    get "/ping", PingController, :ping

    post "/dict/logs", LogController, :create
    get "/dict/logs", LogController, :list
    delete "/dict/logs/:id", LogController, :delete
  end

end
