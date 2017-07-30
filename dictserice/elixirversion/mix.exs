defmodule Elixirversion.Mixfile do
  use Mix.Project

  def project do
    [app: :elixirversion,
     version: "0.0.1",
     elixir: "~> 1.4",
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
#    [mod: {Elixirversion, []},
 #    applications: [:phoenix, :phoenix_pubsub, :cowboy, :logger, :gettext, :redix]]
    [mod: {Elixirversion.Application, []},
     extra_applications: [:logger, :runtime_tools]]
  end

  # Specifies which paths to compile per environment.
  defp elixirc_paths(:test), do: ["lib", "test/support"]
  defp elixirc_paths(_),     do: ["lib"]

  # Specifies your project dependencies.
  #
  # Type `mix help deps` for examples and options.
  defp deps do
    [{:phoenix, "~> 1.3.0"},
     {:phoenix_pubsub, "~> 1.0"},
     {:gettext, "~> 0.13"},
     {:cowboy, "~> 1.1"},
     {:redix, ">= 0.0.0"},
     {:uuid, "~> 1.1"},

     {:phoenix_live_reload, "~> 1.0", only: :dev},
     {:ex_doc, "~> 0.14", only: :dev},
     {:distillery, "~> 0.10"},
     {:dialyxir, "~> 0.5", only: [:dev]},

     {:exfmt, "~> 0.3.0", only: [:dev]},
     {:credo, "~> 0.7", only: [:dev, :test]},
   ]
  end
end
