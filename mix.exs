defmodule Json.Mixfile do
  use Mix.Project

  def project do
    [
      app: :fast_json,
      version: "0.1.0",
      elixir: "~> 1.3",
      compilers: [:rustler] ++ Mix.compilers(),
      build_embedded: Mix.env() == :prod,
      start_permanent: Mix.env() == :prod,
      rustler_crates: [fast_json: []],
      deps: deps(),
      preferred_cli_env: [
        bench: :bench,
        "bench.graph": :bench,
        "bench.cmp": :bench
      ]
    ]
  end

  def application do
    [applications: []]
  end

  defp deps do
    [
      {:rustler, "~> 0.21"},

      # Benchmarking
      {:benchfella, "~> 0.3", only: :bench},
      {:jiffy, "~> 0.14", only: [:bench, :test]},
      {:poison, "~> 3.0", only: :bench}
    ]
  end
end
