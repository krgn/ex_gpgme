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
      end)
    end

    test "should return correct info", ctx do
      {:ok, context} = ExGpgme.Native.context_create(:openpgp, ctx[:gnupg_home])
      {:ok, info} = ExGpgme.Native.context_info(context)
      assert info.home == ctx[:gnupg_home]
      assert info.path == System.find_executable("gpg")
      assert info.protocol == :openpgp
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

    test "should return correct key_id", ctx do
      {:ok, context} = ExGpgme.Native.context_create(:openpgp, ctx[:gnupg_home])
      {:ok, [key]} = ExGpgme.Native.key_list(context)
      {:ok, "728052F947BD30B8"} = ExGpgme.Native.key_id(key)
    end

    test "should list of user ids", ctx do
      {:ok, context} = ExGpgme.Native.context_create(:openpgp, ctx[:gnupg_home])
      {:ok, [key]} = ExGpgme.Native.key_list(context)
      {:ok, [user] = user_ids} = ExGpgme.Native.key_user_ids(key)
      assert is_list(user_ids)
      assert is_map(user)
      assert user.name == "Foo McBar"
      assert user.email == "foo@mcbar.dev"
      assert user.comment == ""
      assert user.validity == :unknown
      assert user.origin == :unknown
      assert user.revoked == false
      assert user.invalid == false
      assert is_list(user.signatures)
    end

    test "should return fingerprint", ctx do
      {:ok, context} = ExGpgme.Native.context_create(:openpgp, ctx[:gnupg_home])
      {:ok, [key]} = ExGpgme.Native.key_list(context)
      {:ok, fingerprint} = ExGpgme.Native.key_fingerprint(key)
      assert fingerprint == "D1DBB4E18FF6FA6AFA040B07728052F947BD30B8"
    end

    test "should return can_encrypt", ctx do
      {:ok, context} = ExGpgme.Native.context_create(:openpgp, ctx[:gnupg_home])
      {:ok, [key]} = ExGpgme.Native.key_list(context)
      {:ok, can_encrypt} = ExGpgme.Native.key_can_encrypt(key)
      assert can_encrypt
    end

    test "should return can_sign", ctx do
      {:ok, context} = ExGpgme.Native.context_create(:openpgp, ctx[:gnupg_home])
      {:ok, [key]} = ExGpgme.Native.key_list(context)
      {:ok, can_sign} = ExGpgme.Native.key_can_sign(key)
      assert can_sign
    end

    test "should return can_certify", ctx do
      {:ok, context} = ExGpgme.Native.context_create(:openpgp, ctx[:gnupg_home])
      {:ok, [key]} = ExGpgme.Native.key_list(context)
      {:ok, can_certify} = ExGpgme.Native.key_can_certify(key)
      assert can_certify
    end

    test "should return can_authenticate", ctx do
      {:ok, context} = ExGpgme.Native.context_create(:openpgp, ctx[:gnupg_home])
      {:ok, [key]} = ExGpgme.Native.key_list(context)
      {:ok, can_authenticate} = ExGpgme.Native.key_can_authenticate(key)
      assert !can_authenticate
    end

    test "should return has_secret", ctx do
      {:ok, context} = ExGpgme.Native.context_create(:openpgp, ctx[:gnupg_home])
      {:ok, [key]} = ExGpgme.Native.key_list(context)
      {:ok, has_secret} = ExGpgme.Native.key_has_secret(key)
      assert !has_secret
    end

    test "should return is_revoked", ctx do
      {:ok, context} = ExGpgme.Native.context_create(:openpgp, ctx[:gnupg_home])
      {:ok, [key]} = ExGpgme.Native.key_list(context)
      {:ok, is_revoked} = ExGpgme.Native.key_is_revoked(key)
      assert !is_revoked
    end

    test "should return is_expired", ctx do
      {:ok, context} = ExGpgme.Native.context_create(:openpgp, ctx[:gnupg_home])
      {:ok, [key]} = ExGpgme.Native.key_list(context)
      {:ok, is_expired} = ExGpgme.Native.key_is_expired(key)
      assert !is_expired
    end

    @tag :focus
    test "should return is_disabled", ctx do
      {:ok, context} = ExGpgme.Native.context_create(:openpgp, ctx[:gnupg_home])
      {:ok, [key]} = ExGpgme.Native.key_list(context)
      {:ok, is_disabled} = ExGpgme.Native.key_is_disabled(key)
      assert !is_disabled
    end

    test "should return is_invalid", ctx do
      {:ok, context} = ExGpgme.Native.context_create(:openpgp, ctx[:gnupg_home])
      {:ok, [key]} = ExGpgme.Native.key_list(context)
      {:ok, is_invalid} = ExGpgme.Native.key_is_invalid(key)
      assert !is_invalid
    end

    test "should return is_qualified", ctx do
      {:ok, context} = ExGpgme.Native.context_create(:openpgp, ctx[:gnupg_home])
      {:ok, [key]} = ExGpgme.Native.key_list(context)
      {:ok, is_qualified} = ExGpgme.Native.key_is_qualified(key)
      assert !is_qualified
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
