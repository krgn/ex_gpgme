defmodule Test.GpgmeCase do
  use ExUnit.CaseTemplate

  using do
    quote do
      # compile time setup
    end
  end

  setup _tags do
    # runtime setup
    {:ok, data: :foo}
  end

  #             _            _
  #  _ __  _ __(_)_   ____ _| |_ ___
  # | '_ \| '__| \ \ / / _` | __/ _ \
  # | |_) | |  | |\ V / (_| | ||  __/
  # | .__/|_|  |_| \_/ \__,_|\__\___|
  # |_|

  def setup_agent do
    _dir = tmp_dir!()
    dir = "/home/k/gpgme/home"

    :ok =
      System.put_env([
        {"GNUPGHOME", dir},
        {"GPG_AGENT_INFO", ""}
      ])

    :ok =
      dir
      |> Path.join("gpg-agent.conf")
      |> File.write!(agent_conf("pinentry"))

    :ok = import_key("test/data/private.asc")
    :ok = import_key("test/data/public.asc")
  end

  def import_key(key_path) do
    gpg = System.get_env("GPG") || "gpg"

    data =
      key_path
      |> Path.expand()
      |> File.read!()

    passphrase = Application.get_env(:ex_gpgme, :test_passphrase)

    port =
      Port.open(
        {:spawn, "#{gpg} --no-permission-warning --passphrase #{passphrase} --import"},
        []
      )

    true = Port.command(port, "#{data}\n")
    true = Port.close(port)
    :ok
  end

  def agent_conf(pinentry) do
    ~s"""
    ignore-invalid-option allow-loopback-pinentry
    allow-loopback-pinentry
    ignore-invalid-option pinentry-mode
    pinentry-mode loopback
    pinentry-program #{pinentry}
    """
  end

  defp tmp_dir! do
    path =
      System.tmp_dir!()
      |> Path.join("tmp.ex_gpgme-#{rand_string()}")

    :ok = File.mkdir_p!(path)
    path
  end

  defp rand_string do
    :sha256
    |> :crypto.hash("#{:rand.uniform()} #{:rand.uniform()}")
    |> Base.encode16(case: :lower)
    |> String.slice(1..16)
  end
end
