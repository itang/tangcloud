defmodule Elixirversion.Web do
  @moduledoc """
  A module that keeps using definitions for controllers,
  views and so on.

  This can be used in your application as:

      use Elixirversion.Web, :controller
      use Elixirversion.Web, :view

  The definitions below will be executed for every view,
  controller, etc, so keep them short and clean, focused
  on imports, uses and aliases.

  Do NOT define functions inside the quoted expressions
  below.
  """

  def model do
    quote do
      # Define common model functionality
    end
  end

  def controller do
    quote do
      use Phoenix.Controller, namespace: Elixirversion.Web

      import Elixirversion.Web.Router.Helpers
      import Elixirversion.Web.Gettext
    end
  end

  def view do
    quote do
      use Phoenix.View, root: "lib/elixirversion/web/templates", namespace: Elixirversion.Web

      # Import convenience functions from controllers
      import Phoenix.Controller, only: [get_csrf_token: 0, get_flash: 2, view_module: 1]

      import Elixirversion.Web.Router.Helpers
      import Elixirversion.Web.ErrorHelpers
      import Elixirversion.Web.Gettext
    end
  end

  def router do
    quote do
      use Phoenix.Router
    end
  end

  def channel do
    quote do
      use Phoenix.Channel
      import Elixirversion.Web.Gettext
    end
  end

  @doc """
  When used, dispatch to the appropriate controller/view/etc.
  """
  defmacro __using__(which) when is_atom(which) do
    apply(__MODULE__, which, [])
  end
end
