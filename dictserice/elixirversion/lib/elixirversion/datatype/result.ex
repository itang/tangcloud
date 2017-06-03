defmodule Elixirversion.Datatype.Result do
    @moduledoc """
    数据类型: {"ok":false,"code":404,"message":"hello都不知道啊, 老子不干了","data":null}
    """
    @enforce_keys [:ok]
    defstruct [:ok, :code, :message, :data]

    def ok(code \\ 0, message \\ nil, data \\ nil) do
       result(true, code, message, data)
    end

    def error(code \\ 0, message \\ nil, data \\ nil) do
        result(false, code, message, data)
    end

    defp result(ok, code \\ 0, message \\ nil, data \\ nil) do
        %Elixirversion.Datatype.Result{ok: ok, code: code,
           message: message, data: data}
    end
end
