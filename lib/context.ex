defmodule ExGpgme.Context do
  @moduledoc """
  """

  alias __MODULE__
  alias ExGpgme.Key

  @type t :: Context
  @type path :: binary()
  @type reason :: :foo

  defstruct home: nil,
            path: nil,
            version: nil,
            protocol: nil,
            ref: nil

  @spec create(path()) :: {:ok, t()} | {:error, reason()}
  @doc """
  Create a new Gpgme context to work with. Pass it the home directory
  path, or have it use the current users $HOME/.gnupg

  ## Example

      iex> {:ok, context} = ExGpgme.Context.create()
      iex> assert context.home == Path.join(System.get_env("HOME"), ".gnupg")
      true

  """
  def create(path) when is_binary(path) do
    case ExGpgme.Native.context_create(:openpgp, path) do
      {:ok, ref} ->
        case ExGpgme.Native.context_info(ref) do
          {:ok, info} ->
            context = %Context{
              home: info.home,
              path: info.path,
              version: info.version,
              protocol: info.protocol,
              ref: ref
            }

            {:ok, context}
        end

      error ->
        error
    end
  end

  @spec list_keys(t()) :: [Key.t()]
  @doc """
  List all keys for a Context.
  """
  def list_keys(%Context{} = context) do
    case ExGpgme.Native.key_list(context.ref) do
      {:ok, keys} -> Enum.map(keys, fn ref -> Key.from(ref) end)
      error -> error
    end
  end

  @spec encrypt(t(), Key.t(), binary()) :: {:ok, binary()} | {:error, atom()}
  @doc """
  Encrypt some passed string with the given Key.
  """
  def encrypt(%Context{} = context, %Key{} = key, data) when is_binary(data) do
    ExGpgme.Native.context_encrypt(context.ref, key.fingerprint, data)
  end

  @spec decrypt(t(), binary(), binary()) :: {:ok, binary()} | {:error, atom()}
  @doc """
  Decrypt some passed string with the given passphrase.
  """
  def decrypt(%Context{} = context, passphrase, data)
      when is_binary(data) and is_binary(passphrase) do
    ExGpgme.Native.context_decrypt(context.ref, passphrase, data)
  end
end
