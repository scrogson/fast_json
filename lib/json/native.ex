defmodule Json.Native do
  @moduledoc """
  Native JSON encoding/decoding library for Elixir using Rust.
  """
  use Rustler, otp_app: :fast_json

  def decode_naive(_), do: nif_error()
  def decode_init(_), do: nif_error()
  def decode_iter(_, _), do: nif_error()
  def decode_dirty(_), do: nif_error()
  def decode_threaded(_), do: nif_error()
  def encode_dirty(_), do: nif_error()

  defp nif_error, do: :erlang.nif_error(:nif_not_loaded)
end
