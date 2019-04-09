defmodule ExGpgme.Test do
  use Test.GpgmeCase

  describe "Context" do
    test "create", ctx do
      {:ok, context} = ExGpgme.create(path: ctx[:gnupg_home])
      assert context.home == ctx[:gnupg_home]
      assert context.path == System.find_executable("gpg")
      assert context.protocol == :openpgp
      assert is_reference(context.ref)
      assert is_binary(context.version)
    end
  end

  describe "Keys" do
    test "list", ctx do
      {:ok, context} = ExGpgme.create(path: ctx[:gnupg_home])
      [key] = ExGpgme.list_keys(context)

      %ExGpgme.Key{
        can_authenticate: false,
        can_certify: true,
        can_encrypt: true,
        can_sign: true,
        fingerprint: "D1DBB4E18FF6FA6AFA040B07728052F947BD30B8",
        has_secret: false,
        id: "728052F947BD30B8",
        is_disabled: false,
        is_expired: false,
        is_invalid: false,
        is_qualified: false,
        is_revoked: false,
        subkeys: [
          %{
            algorithm: "RSA",
            can_auth: false,
            can_certify: true,
            can_encrypt: false,
            can_sign: true,
            card_key: false,
            card_serial_number: :none,
            creation_time: 1_551_605_700,
            curve: :unknown,
            disabled: false,
            expiration_time: 1_614_677_700,
            expired: false,
            fingerprint: "D1DBB4E18FF6FA6AFA040B07728052F947BD30B8",
            invalid: false,
            length: 2048,
            revoked: false,
            secret: false
          },
          %{
            algorithm: "RSA",
            can_auth: false,
            can_certify: false,
            can_encrypt: true,
            can_sign: false,
            card_key: false,
            card_serial_number: :none,
            creation_time: 1_551_605_700,
            curve: :unknown,
            disabled: false,
            expiration_time: 1_614_677_700,
            expired: false,
            fingerprint: "0F6AD6B73701F318DB8B5F4D2148F0B7FA6258CA",
            invalid: false,
            length: 2048,
            revoked: false,
            secret: false
          }
        ],
        user_ids: [
          %{
            comment: "",
            email: "foo@mcbar.dev",
            invalid: false,
            name: "Foo McBar",
            origin: :unknown,
            revoked: false,
            signatures: [],
            tofu_info: :none,
            validity: :unknown
          }
        ]
      } = key
    end

    test "import public key", ctx do
      {:ok, context} = ExGpgme.create(path: ctx[:gnupg_home])
      keys = ExGpgme.list_keys(context)
      assert length(keys) == 1

      public_key = File.read!("test/data/boaty_mcboatface/public.asc")
      {:ok, result} = ExGpgme.import_key(context, public_key)

      keys = ExGpgme.list_keys(context)
      assert length(keys) == 2

      %{
        considered: 1,
        imported: 1,
        imports: [
          %{
            fingerprint: "BB6700D8CFF4EDA5E6E233093722D688D77C1C10",
            result: :ok,
            status: "NEW"
          }
        ],
        new_revocations: 0,
        new_subkeys: 0,
        new_user_ids: 0,
        not_imported: 0,
        secret_considered: 0,
        secret_imported: 0,
        secret_unchanged: 0,
        unchanged: 0,
        without_user_id: 0
      } = result
    end

    test "import private/public key pair", ctx do
      {:ok, context} = ExGpgme.create(path: ctx[:gnupg_home])
      keys = ExGpgme.list_keys(context)
      assert length(keys) == 1

      public_key = File.read!("test/data/boaty_mcboatface/private.asc")
      {:ok, result} = ExGpgme.import_key(context, public_key)

      keys = ExGpgme.list_keys(context)
      assert length(keys) == 2

      %{
        considered: 1,
        imported: 1,
        imports: [
          %{fingerprint: "BB6700D8CFF4EDA5E6E233093722D688D77C1C10", result: :ok, status: "NEW"},
          %{
            fingerprint: "BB6700D8CFF4EDA5E6E233093722D688D77C1C10",
            result: :ok,
            status: "NEW | SECRET"
          }
        ],
        new_revocations: 0,
        new_subkeys: 0,
        new_user_ids: 0,
        not_imported: 0,
        secret_considered: 1,
        secret_imported: 1,
        secret_unchanged: 0,
        unchanged: 0,
        without_user_id: 0
      } = result
    end

    test "should not import trash", ctx do
      {:ok, context} = ExGpgme.create(path: ctx[:gnupg_home])
      keys = ExGpgme.list_keys(context)
      assert length(keys) == 1

      public_key = "funky bogus key"
      {:ok, result} = ExGpgme.import_key(context, public_key)

      keys = ExGpgme.list_keys(context)
      assert length(keys) == 1

      %{
        considered: 0,
        imported: 0,
        imports: [],
        new_revocations: 0,
        new_subkeys: 0,
        new_user_ids: 0,
        not_imported: 0,
        secret_considered: 0,
        secret_imported: 0,
        secret_unchanged: 0,
        unchanged: 0,
        without_user_id: 0
      } = result
    end

    test "find key by email", ctx do
      {:ok, context} = ExGpgme.create(path: ctx[:gnupg_home])
      {:ok, key} = ExGpgme.find_key(context, "foo@mcbar.dev")

      %ExGpgme.Key{
        can_authenticate: false,
        can_certify: true,
        can_encrypt: true,
        can_sign: true,
        fingerprint: "D1DBB4E18FF6FA6AFA040B07728052F947BD30B8",
        has_secret: false,
        id: "728052F947BD30B8",
        is_disabled: false,
        is_expired: false,
        is_invalid: false,
        is_qualified: false,
        is_revoked: false,
        subkeys: [
          %{
            algorithm: "RSA",
            can_auth: false,
            can_certify: true,
            can_encrypt: false,
            can_sign: true,
            card_key: false,
            card_serial_number: :none,
            creation_time: 1_551_605_700,
            curve: :unknown,
            disabled: false,
            expiration_time: 1_614_677_700,
            expired: false,
            fingerprint: "D1DBB4E18FF6FA6AFA040B07728052F947BD30B8",
            invalid: false,
            length: 2048,
            revoked: false,
            secret: false
          },
          %{
            algorithm: "RSA",
            can_auth: false,
            can_certify: false,
            can_encrypt: true,
            can_sign: false,
            card_key: false,
            card_serial_number: :none,
            creation_time: 1_551_605_700,
            curve: :unknown,
            disabled: false,
            expiration_time: 1_614_677_700,
            expired: false,
            fingerprint: "0F6AD6B73701F318DB8B5F4D2148F0B7FA6258CA",
            invalid: false,
            length: 2048,
            revoked: false,
            secret: false
          }
        ],
        user_ids: [
          %{
            comment: "",
            email: "foo@mcbar.dev",
            invalid: false,
            name: "Foo McBar",
            origin: :unknown,
            revoked: false,
            signatures: [],
            tofu_info: :none,
            validity: :unknown
          }
        ]
      } = key
    end

    test "find key by name", ctx do
      {:ok, context} = ExGpgme.create(path: ctx[:gnupg_home])
      {:ok, key} = ExGpgme.find_key(context, "Foo McBar")

      %ExGpgme.Key{
        can_authenticate: false,
        can_certify: true,
        can_encrypt: true,
        can_sign: true,
        fingerprint: "D1DBB4E18FF6FA6AFA040B07728052F947BD30B8",
        has_secret: false,
        id: "728052F947BD30B8",
        is_disabled: false,
        is_expired: false,
        is_invalid: false,
        is_qualified: false,
        is_revoked: false,
        subkeys: [
          %{
            algorithm: "RSA",
            can_auth: false,
            can_certify: true,
            can_encrypt: false,
            can_sign: true,
            card_key: false,
            card_serial_number: :none,
            creation_time: 1_551_605_700,
            curve: :unknown,
            disabled: false,
            expiration_time: 1_614_677_700,
            expired: false,
            fingerprint: "D1DBB4E18FF6FA6AFA040B07728052F947BD30B8",
            invalid: false,
            length: 2048,
            revoked: false,
            secret: false
          },
          %{
            algorithm: "RSA",
            can_auth: false,
            can_certify: false,
            can_encrypt: true,
            can_sign: false,
            card_key: false,
            card_serial_number: :none,
            creation_time: 1_551_605_700,
            curve: :unknown,
            disabled: false,
            expiration_time: 1_614_677_700,
            expired: false,
            fingerprint: "0F6AD6B73701F318DB8B5F4D2148F0B7FA6258CA",
            invalid: false,
            length: 2048,
            revoked: false,
            secret: false
          }
        ],
        user_ids: [
          %{
            comment: "",
            email: "foo@mcbar.dev",
            invalid: false,
            name: "Foo McBar",
            origin: :unknown,
            revoked: false,
            signatures: [],
            tofu_info: :none,
            validity: :unknown
          }
        ]
      } = key
    end

    test "find key should return error on bogus input", ctx do
      {:ok, context} = ExGpgme.create(path: ctx[:gnupg_home])
      {:error, :not_found} = ExGpgme.find_key(context, "Dr. Baz")
    end
  end

  describe "Signature Notations" do
    test "list signature notations", ctx do
      {:ok, context} = ExGpgme.create(path: ctx[:gnupg_home])
      {:ok, notations} = ExGpgme.signature_notations(context)
      assert is_list(notations)
    end

    test "add signature notation", ctx do
      {:ok, context} = ExGpgme.create(path: ctx[:gnupg_home])
      {:ok, notations} = ExGpgme.signature_notations(context)
      assert length(notations) == 0

      name = "foo"
      value = "bar"
      flags = [:human_readable, :critical]
      :ok = ExGpgme.add_signature_notation(context, name, value, flags)

      {:ok, notations} = ExGpgme.signature_notations(context)
      [%{critical: true, human_readable: true, name: "foo", value: "bar"}] = notations
    end

    @tag :focus
    test "clear signature notations", ctx do
      {:ok, context} = ExGpgme.create(path: ctx[:gnupg_home])
      {:ok, notations} = ExGpgme.signature_notations(context)
      assert length(notations) == 0

      name = "foo"
      value = "bar"
      flags = [:human_readable, :critical]
      :ok = ExGpgme.add_signature_notation(context, name, value, flags)

      {:ok, notations} = ExGpgme.signature_notations(context)
      assert length(notations) == 1

      :ok = ExGpgme.clear_signature_notations(context)

      {:ok, notations} = ExGpgme.signature_notations(context)
      assert length(notations) == 0
    end
  end

  describe "Encryption/Decryption" do
    alias ExGpgme.Key

    test "should encrypt/decrypt data with public key", ctx do
      {:ok, context} = ExGpgme.create(path: ctx[:gnupg_home])

      public_key = File.read!("test/data/boaty_mcboatface/private.asc")
      {:ok, _} = ExGpgme.import_key(context, public_key)

      {:ok, key} = ExGpgme.find_key(context, "foo@mcbar")

      message = """
      Hello Boaty,

      I hope that everything is well, and I hope to see you soon.

      Yours sincerely,

      Foo
      """

      {:ok, cipher_text} = ExGpgme.encrypt(context, key, message)

      assert String.starts_with?(cipher_text, "-----BEGIN PGP MESSAGE-----")
      passphrase = Application.get_env(:ex_gpgme, :test_passphrase)
      {:ok, decrypted} = ExGpgme.decrypt(context, passphrase, cipher_text)

      assert message == decrypted
    end

    test "should encrypt/decrypt data with symmetric key", ctx do
      {:ok, context} = ExGpgme.create(path: ctx[:gnupg_home])

      message = """
      Hello Foo,

      Thank you for your message. I shall meet you soon.

      Yours truly,

      Boaty
      """

      shared_passphrase = Application.get_env(:ex_gpgme, :test_passphrase)

      {:ok, cipher_text} = ExGpgme.encrypt_symmetric(context, shared_passphrase, message)

      assert String.starts_with?(cipher_text, "-----BEGIN PGP MESSAGE-----")

      {:ok, decrypted} = ExGpgme.decrypt(context, shared_passphrase, cipher_text)

      assert message == decrypted
    end
  end
end
