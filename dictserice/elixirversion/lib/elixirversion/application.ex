defmodule Elixirversion.Application do
  @moduledoc """
  Application Entry
  """

  use Application

  alias ElixirversionWeb.Endpoint

  # See http://elixir-lang.org/docs/stable/elixir/Application.html
  # for more information on OTP Applications
  def start(_type, _args) do
    import Supervisor.Spec

    # Define workers and child supervisors to be supervised
    children = [
      # Start the endpoint when the application starts
      supervisor(Endpoint, []),
      # https://hexdocs.pm/redix/real-world-usage.html
      worker(Redix, [[host: redis_host()], [name: :redix]]),
      # Start your own worker by calling: Elixirversion.Worker.start_link(arg1, arg2, arg3)
      # worker(Elixirversion.Worker, [arg1, arg2, arg3]),
    ]

    # See http://elixir-lang.org/docs/stable/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: Elixirversion.Supervisor]
    Supervisor.start_link(children, opts)
  end

    # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  def config_change(changed, _new, removed) do
    ElixirversionWeb.Endpoint.config_change(changed, removed)
    :ok
  end

  defp redis_host do
    System.get_env("REDIS_HOST") || "localhost"
  end
end