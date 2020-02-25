defmodule Json do
  defmodule Error do
    defexception [:message]
  end

  alias Json.Native

  @doc ~S"""
  Decodes a binary string into Elixir terms.

  ## Examples
    iex> Json.decode(~s|{"hello":"world"}|)
    {:ok, %{"hello" => "world"}}

    iex> Json.decode(~s|{"lists":[1,2,3]}|)
    {:ok, %{"lists" => [1,2,3]}}

  """
  def decode(data), do: threaded_decode(data)

  def decode!(data) do
    case decode(data) do
      {:ok, result} -> result
      {:error, error} -> raise Error, message: error
    end
  end

  def threaded_decode(data) do
    :ok = Native.decode_threaded(data)

    receive do
      {:ok, result} ->
        {:ok, result}

      {:error, error} ->
        {:error, error}
    after
      5000 ->
        {:error, :timeout}
    end
  end

  def parse(data) do
    data
    |> Native.decode_init()
    |> handle_parse_result()
  end

  def handle_parse_result(result) do
    case result do
      {:ok, result} ->
        {:ok, result}

      {:more, resource, acc} ->
        resource
        |> Native.decode_iter(acc)
        |> handle_parse_result()

      {:error, error} ->
        {:error, error}
    end
  end

  @doc ~S"""
  Decodes a map or struct into a JSON string.

  ## Examples
    iex> Json.encode(%{hello: "world",list: [%{a: "b"}]})
    {:ok, ~s({"hello":"world","list":[{"a":"b"}]})}

  """
  def encode!(data) do
    case encode(data) do
      {:ok, result} -> result
      {:error, error} -> raise Error, message: error
    end
  end

  def encode(data), do: Native.encode_dirty(data)
end
