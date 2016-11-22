defmodule Elixirversion.Plugs.RuntimePlug do
  @moduledoc """
  X-Runtime Plug Middleware
  """
  alias Plug.Conn

  def init(options), do: options

  def call(conn, _opts) do
    start = System.monotonic_time()

    Conn.register_before_send(conn, fn conn ->
      stop = System.monotonic_time()
      diff = System.convert_time_unit(stop - start, :native, :micro_seconds)

      conn |> Conn.put_resp_header("x-runtime", formatted_diff(diff))
    end)
  end

  @spec formatted_diff(integer) :: String.t
  defp formatted_diff(diff) when diff > 1000, do: (diff |> div(1000) |> Integer.to_string) <> "ms"
  defp formatted_diff(diff), do: (diff |> Integer.to_string) <> "us"
end
