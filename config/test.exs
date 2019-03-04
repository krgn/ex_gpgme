use Mix.Config

config :ex_gpgme,
  test_passphrase: "6c616829565def2c"

config :porcelain, driver: Porcelain.Driver.Basic

try do
  import_config "#{Mix.env()}.exs"
rescue
  _ -> :nevermind
end
