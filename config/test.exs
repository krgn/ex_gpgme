use Mix.Config

config :ex_gpgme,
  test_passphrase: "6c616829565def2c"

try do
  import_config "#{Mix.env()}.exs"
rescue
  _ -> :nevermind
end
