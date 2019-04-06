defmodule Test.GpgmeCase do
  use ExUnit.CaseTemplate

  require Logger

  alias Porcelain.Result
  alias Porcelain.Process

  using do
    quote do
      # compile time setup
    end
  end

  setup _tags do
    {agent, dir} = setup_agent()
    on_exit(fn -> kill_agent(agent, dir) end)

    {:ok, gnupg_home: dir}
  end

  def setup_agent do
    dir = tmp_dir!()
    pinentry = System.find_executable("pinentry")
    conf = agent_conf(dir, pinentry)

    :ok =
      System.put_env([
        {"GNUPGHOME", dir},
        {"GPG_AGENT_INFO", ""}
      ])

    :ok =
      dir
      |> Path.join("gpg-agent.conf")
      |> File.write!(conf)

    agent = start_agent(dir)

    :ok = import_key("test/data/foo_mcbar/public.asc")
    :ok = import_key("test/data/foo_mcbar/private.asc")

    {agent, dir}
  end

  def start_agent(dir) do
    Logger.debug("Starting test gpg-agent")

    proc =
      Porcelain.spawn_shell("gpg-agent --homedir #{dir} --server",
        in: :receive,
        out: {:send, self()}
      )

    receive do
      {_pid, :data, :out, "OK" <> _rest} -> Logger.debug("Ok")
    end

    proc
  end

  def kill_agent(proc, dir) do
    Logger.debug("Stopping gpg-agent")
    true = Process.stop(proc)

    %Result{out: out} = Porcelain.exec("pidof", ["gpg-agent"])

    out
    |> String.trim()
    |> String.split()
    |> Enum.each(fn pid ->
      if matches?(pid, dir) do
        %Result{status: 0} = Porcelain.exec("kill", [pid])
      end
    end)

    File.rm_rf!(dir)
  end

  def matches?(pid, dir) do
    case File.read("/proc/#{pid}/environ") do
      {:ok, data} -> String.contains?(data, dir)
      _ -> true
    end
  end

  def import_key(key_path) do
    gpg = System.get_env("GPG") || System.find_executable("gpg")
    passphrase = Application.get_env(:ex_gpgme, :test_passphrase)
    key_path = Path.expand(key_path)

    cmd = "echo #{passphrase} | #{gpg} --batch --yes --passphrase-fd 0 --import #{key_path}"

    %Result{status: 0, out: _out, err: _err} = Porcelain.shell(cmd)
    :ok
  end

  def agent_conf(_dir, pinentry) do
    """
    ignore-invalid-option allow-loopback-pinentry
    allow-loopback-pinentry
    ignore-invalid-option pinentry-mode
    pinentry-mode loopback
    pinentry-program #{pinentry}
    """
  end

  def tmp_dir! do
    path =
      System.tmp_dir!()
      |> Path.join("tmp.ex_gpgme-#{rand_string()}")

    :ok = File.mkdir_p!(path)
    %Result{status: 0} = Porcelain.exec("chmod", ["-R", "700", "#{path}"])
    path
  end

  #             _            _
  #  _ __  _ __(_)_   ____ _| |_ ___
  # | '_ \| '__| \ \ / / _` | __/ _ \
  # | |_) | |  | |\ V / (_| | ||  __/
  # | .__/|_|  |_| \_/ \__,_|\__\___|
  # |_|

  defp rand_string do
    :sha256
    |> :crypto.hash("#{:rand.uniform()} #{:rand.uniform()}")
    |> Base.encode16(case: :lower)
    |> String.slice(1..16)
  end
end
