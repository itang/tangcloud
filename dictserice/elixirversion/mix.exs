defmodule Elixirversion.Mixfile do
  use Mix.Project

  def project do
    [app: :elixirversion,
     version: "0.0.1",
     elixir: "~> 1.2",
     elixirc_paths: elixirc_paths(Mix.env),
     compilers: [:phoenix, :gettext] ++ Mix.compilers,
     build_embedded: Mix.env == :prod,
     start_permanent: Mix.env == :prod,
     deps: deps()]
  end

  # Configuration for the OTP application.
  #
  # Type `mix help compile.app` for more information.
  def application do
    [mod: {Elixirversion, []},
     applications: [:phoenix, :phoenix_pubsub, :cowboy, :logger, :gettext, :redix]]
  end

  # Specifies which paths to compile per environment.
  defp elixirc_paths(:test), do: ["lib", "web", "test/support"]
  defp elixirc_paths(_),     do: ["lib", "web"]

  # Specifies your project dependencies.
  #
  # Type `mix help deps` for examples and options.
  defp deps do
    [{:phoenix, "~> 1.2.1"},
     {:phoenix_pubsub, "~> 1.0"},
     {:gettext, "~> 0.11"},
     {:cowboy, "~> 1.0"},
     {:redix, ">= 0.0.0"},

     {:distillery, "~> 0.10", only: [:dev]},
     {:dialyxir, "~> 0.4", only: [:dev]},
     {:credo, "~> 0.5", only: [:dev, :test]},
     #{:json, "~> 1.0"},
   ]
  end
end
