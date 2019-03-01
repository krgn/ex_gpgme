defmodule ExGpgme.MixProject do
  use Mix.Project

  def project do
    [
      app: :ex_gpgme,
      version: "0.1.0",
      elixir: "~> 1.7",
      start_permanent: Mix.env() == :prod,
      compilers: [:rustler] ++ Mix.compilers(),
      rustler_crates: rustler_crates(),
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

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
end
