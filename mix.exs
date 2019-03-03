defmodule ExGpgme.MixProject do
  use Mix.Project

  def project do
    [
      app: :ex_gpgme,
      version: "0.1.0",
      elixir: "~> 1.7",
      start_permanent: Mix.env() == :prod,
      compilers: [:rustler] ++ Mix.compilers(),
      elixirc_paths: elixirc_paths(Mix.env()),
      rustler_crates: rustler_crates(),
      aliases: aliases(),
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp elixirc_paths(:test), do: ["lib", "test/support"]
  defp elixirc_paths(_), do: ["lib"]

  defp rustler_crates do
    [
      gpgme_bindings: [
        path: "native/gpgme_bindings",
        mode: if(Mix.env() == :prod, do: :release, else: :debug)
      ]
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.19.1"}
    ]
  end

  def aliases do
    [
      test: "test --trace"
    ]
  end
end
