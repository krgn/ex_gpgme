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
  def create(path: path) when is_binary(path) do
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

  @spec encrypt_symmetric(t(), binary(), binary()) :: {:ok, binary()} | {:error, atom()}
  @doc """
  Encrypt some passed string with the given Key.
  """
  def encrypt_symmetric(%Context{} = context, passphrase, data)
      when is_binary(passphrase) and is_binary(data) do
    ExGpgme.Native.context_encrypt_symmetric(context.ref, passphrase, data)
  end

  @spec decrypt(t(), binary(), binary()) :: {:ok, binary()} | {:error, atom()}
  @doc """
  Decrypt some passed string with the given passphrase.
  """
  def decrypt(%Context{} = context, passphrase, data)
      when is_binary(data) and is_binary(passphrase) do
    ExGpgme.Native.context_decrypt(context.ref, passphrase, data)
  end

  @spec import_key(t(), binary()) :: {:ok, map()} | {:error, atom()}
  @doc """
  Decrypt some passed string with the given passphrase.
  """
  def import_key(%Context{} = context, data) when is_binary(data) do
    ExGpgme.Native.context_import(context.ref, data)
  end

  @spec find_key(t(), binary()) :: {:ok, Key.t()} | {:error, atom()}
  @doc """
  Decrypt some passed string with the given passphrase.
  """
  def find_key(%Context{} = context, query) do
    case ExGpgme.Native.context_find_key(context.ref, query) do
      :not_found -> {:error, :not_found}
      {:ok, ref} -> {:ok, ExGpgme.Key.from(ref)}
    end
  end

  @spec signature_notations(t()) :: {:ok, list()} | {:error, atom()}
  @doc """
  Return a list of signers.
  """
  def signature_notations(%Context{} = context) do
    ExGpgme.Native.context_signature_notations(context.ref)
  end

  @spec add_signature_notation(t(), binary(), binary(), keyword()) ::
          {:ok, list()} | {:error, atom()}
  @doc """
  Return a list of add_signers.
  """
  def add_signature_notation(%Context{} = context, name, value, flags) do
    ExGpgme.Native.context_add_signature_notation(context.ref, name, value, flags)
  end

  @spec clear_signature_notations(t()) :: {:ok, list()} | {:error, atom()}
  @doc """
  Return a list of clear_signers.
  """
  def clear_signature_notations(%Context{} = context) do
    ExGpgme.Native.context_clear_signature_notations(context.ref)
  end

  @spec sender(t()) :: {:ok, :none} | {:error, atom()}
  @doc """
  Return currently active sender
  """
  def sender(%Context{} = context) do
    case ExGpgme.Native.context_sender(context.ref) do
      :error -> {:ok, :none}
      sender -> {:ok, sender}
    end
  end

  @spec clear_sender(t()) :: {:ok, :none} | {:error, atom()}
  @doc """
  Return currently active clear_sender
  """
  def clear_sender(%Context{} = context) do
    ExGpgme.Native.context_clear_sender(context.ref)
  end

  @spec set_sender(t(), binary()) :: {:ok, :none} | {:ok, binary()} | {:error, :not_found}
  @doc """
  Return currently active set_sender
  """
  def set_sender(%Context{} = context, sender) do
    case ExGpgme.Native.context_set_sender(context.ref, sender) do
      :error -> {:error, :not_found}
      :ok -> {:ok, sender}
    end
  end

  # @spec signers(t()) :: {:ok, list()} | {:error, atom()}
  # @doc """
  # Return a list of signers.
  # """
  # def signers(%Context{} = context) do
  #   ExGpgme.Native.context_signers(context.ref)
  # end
end
