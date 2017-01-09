defmodule EncoderBench do
  use Benchfella

  # Lists
  bench "lists (Poison)", [list: gen_list] do
    Poison.encode!(list)
  end

  bench "lists (jiffy)", [list: gen_list] do
    :jiffy.encode(list)
  end

  bench "lists (Json)", [list: gen_list] do
    Json.encode!(list)
  end

  # Maps
  bench "maps (Poison)", [map: gen_map] do
    Poison.encode!(map)
  end

  bench "maps (jiffy)", [map: gen_map] do
    :jiffy.encode(map)
  end

  bench "maps (JSON)", [map: gen_map] do
    Json.encode!(map)
  end

  # Strings
  bench "strings (Poison)", [string: gen_string] do
    Poison.encode!(string)
  end

  bench "strings (jiffy)", [string: gen_string] do
    :jiffy.encode(string)
  end

  bench "strings (Json)", [string: gen_string] do
    Json.encode!(string)
  end

  # String escaping
  bench "string escaping (Poison)", [string: gen_string] do
    Poison.encode!(string, escape: :unicode)
  end

  bench "string escaping (jiffy)", [string: gen_string] do
    :jiffy.encode(string, [:uescape])
  end

  # Structs
  bench "structs (Poison)", [structs: gen_structs] do
    Poison.encode!(structs)
  end

  bench "structs (JSON)", [structs: gen_structs] do
    Json.encode!(structs)
  end

  bench "Poison", [data: gen_data] do
    Poison.encode!(data)
  end

  bench "jiffy", [data: gen_data] do
    :jiffy.encode(data)
  end

  bench "JSON", [data: gen_data] do
    Json.encode!(data)
  end

  bench "Poison (pretty)", [data: gen_data] do
    Poison.encode!(data, pretty: true)
  end

  bench "jiffy (pretty)", [data: gen_data] do
    :jiffy.encode(data, [:pretty])
  end

  defp gen_list do
    1..1000 |> Enum.to_list
  end

  defp gen_map do
    Stream.map(?A..?Z, &<<&1>>) |> Stream.with_index |> Enum.into(%{})
  end

  defp gen_string do
    Path.expand("data/UTF-8-demo.txt", __DIR__) |> File.read!
  end

  defmodule Struct do
    @derive [Poison.Encoder]
    defstruct x: nil
  end

  defp gen_structs do
    1..10 |> Enum.map(&(%Struct{x: &1}))
  end

  defp gen_data do
    Path.expand("data/generated.json", __DIR__) |> File.read! |> Poison.decode!
  end
end
