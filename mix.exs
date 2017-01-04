defmodule Json.Mixfile do
  use Mix.Project

  def project do
    [app: :fast_json,
     version: "0.1.0",
     elixir: "~> 1.3",
     compilers: [:rustler] ++ Mix.compilers,
     build_embedded: Mix.env == :prod,
     start_permanent: Mix.env == :prod,
     rustler_crates: rustler_crates(),
     deps: deps(),
     preferred_cli_env: [
       "bench": :bench,
       "bench.graph": :bench,
       "bench.cmp": :bench
     ]]
  end

  def application do
    [applications: []]
  end

  defp deps do
    [{:rustler, github: "hansihe/rustler", sparse: "rustler_mix"},

     # Benchmarking
     {:benchfella, "~> 0.3", only: :bench},
     {:jiffy, "~> 0.14", only: [:bench, :test]},
     {:poison, "~> 3.0", only: :bench}]
  end

  defp rustler_crates do
    [fast_json: [
      path: "/native/fast_json",
      mode: rustc_mode(Mix.env)]]
  end

  defp rustc_mode(:prod), do: :release
  defp rustc_mode(:bench), do: :release
  defp rustc_mode(_), do: :debug
end
