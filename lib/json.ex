defmodule Json do
  @moduledoc """
  Native JSON encoding/decoding library for Elixir using Rust.
  """
  use Rustler, otp_app: :fast_json

  defmodule Error do
    defexception [:message]
  end

  @doc ~S"""
  Decodes a binary string into Elixir terms.

  ## Examples
    iex> Json.decode(~s|{"hello":"world"}|)
    {:ok, %{"hello" => "world"}}

    iex> Json.decode(~s|{"lists":[1,2,3]}|)
    {:ok, %{"lists" => [1,2,3]}}

  """
  def decode(data, opts \\ []), do: decode_naive(data, opts)

  def decode!(data, opts \\ []) do
    case parse(data, opts) do
      {:ok, result} -> result
      {:error, error} -> raise Error, message: error
    end
  end

  def threaded_decode(data, opts \\ []) do
    :ok = decode_threaded(data, opts)
    receive do
      {:ok, result} ->
        {:ok, result}
      {:error, error} ->
        {:error, error}
    after 5000 ->
      {:error, :timeout}
    end
  end

  def parse(data, opts \\ []) do
    data
    |> decode_init(opts)
    |> handle_parse_result()
  end

  def handle_parse_result(result) do
    case result do
      {:ok, result} ->
        {:ok, result}
      {:more, resource, acc} ->
        resource
        |> decode_iter(acc)
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
  def encode!(data, opts \\ []) do
    case encode(data, opts) do
      {:ok, result} -> result
      {:error, error} -> raise Error, message: error
    end
  end
  def encode(data, opts \\ []), do: encode_dirty(data, opts)

  # NIFs
  def decode_naive(_, _ \\ []), do: nif_error()
  def decode_init(_, _), do: nif_error()
  def decode_iter(_, _), do: nif_error()
  def decode_dirty(_, _), do: nif_error()
  def decode_threaded(_, _), do: nif_error()
  def encode_dirty(_, _), do: nif_error()

  defp nif_error, do: :erlang.nif_error(:nif_not_loaded)
end
