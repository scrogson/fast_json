defmodule Json do

  defmodule Error do
    defexception [:message]
  end

  @doc ~S"""
  Dencodes a binary string into Elixir terms.

  ## Examples
    iex> Json.parse(~s|{"hello":"world"}|)
    {:ok, %{"hello" => "world"}}

    iex> Json.parse(~s|{"lists":[1,2,3]}|)
    {:ok, %{"lists" => [1,2,3]}}

  """
  def parse_naive(data, opts \\ []), do: naive_parse(data, opts)
  def parse!(data, opts \\ []) do
    case parse(data, opts) do
      {:ok, result} -> result
      {:error, error} -> raise Error, message: error
    end
  end

  def parse(data, opts \\ []) do
    case decode_init(data, opts) do
      {:ok, result} ->
        {:ok, result}
      {:more, resource, acc} ->
        IO.inspect resource
        #IO.inspect acc
        #more_chuck(resource, acc)
        {:ok, acc}
      {:error, error} ->
        {:error, error}
    end
  end

  defp naive_parse(_, _ \\ []), do: exit(:nif_not_loaded)
  defp decode_init(_, _ \\ []), do: exit(:nif_not_loaded)
  def decode_iter(_, _ \\ []), do: exit(:nif_not_loaded)

  @doc ~S"""
  Decodes a map or struct into a JSON string.

  ## Examples
    iex> Json.stringify(%{hello: "world",list: [%{a: "b"}]})
    ~s({"hello":"world","list":[{"a":"b"}]})

  """
  def stringify!(data, opts \\ []), do: stringify(data, opts)
  def stringify(_, _ \\ []), do: exit(:nif_not_loaded)

  @on_load :__load_nif__

  @doc false
  def __load_nif__ do
    require Rustler
    :ok = Rustler.load_nif(:fast_json, "fast_json")
  end
end
