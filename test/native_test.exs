defmodule ExGpgme.Bindings.Test do
  use Test.GpgmeCase

  require Logger

  describe "Context" do
    test "should create successfully", ctx do
      {:ok, context} = ExGpgme.Native.context_create(:openpgp, ctx[:gnupg_home])
      assert is_reference(context)
    end

    test "should fail on unsupported protocol", ctx do
      :unsupported_protocol = ExGpgme.Native.context_create(:foobar, ctx[:gnupg_home])
    end

    test "should allow creating multiple contexts " do
      setups = [
        Test.GpgmeCase.setup_agent(),
        Test.GpgmeCase.setup_agent()
      ]

      Enum.map(setups, fn {_agent, dir} ->
        {:ok, context} = ExGpgme.Native.context_create(:openpgp, dir)
        assert is_reference(context)
      end)

      Enum.each(setups, fn {agent, dir} ->
        Test.GpgmeCase.kill_agent(agent, dir)
        |> IO.inspect(label: "RESULT")
      end)
    end

    test "should return correct info", ctx do
      {:ok, context} = ExGpgme.Native.context_create(:openpgp, ctx[:gnupg_home])
      {:ok, info} = ExGpgme.Native.context_info(context)
      assert info.home == ctx[:gnupg_home]
      assert info.path == System.find_executable("gpg")
      assert is_binary(info.version)
      assert is_binary(info.required_version)
    end
  end

  describe "Keys" do
    test "should list keys", ctx do
      {:ok, context} = ExGpgme.Native.context_create(:openpgp, ctx[:gnupg_home])
      {:ok, keys} = ExGpgme.Native.key_list(context)
      assert length(keys) == 1
    end
  end

  #   test "should import key"
  #   test "should export key"
  #   test "should sign key"
  #   test "should verify"
  # end

  # describe "Encryption" do
  #   test "should encrypt (simple)"
  #   test "should decrypt (symmetric)"
  # end
end
