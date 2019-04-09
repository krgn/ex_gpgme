defmodule ExGpgme do
  @moduledoc """
  Documentation for ExGpgme.
  """

  alias ExGpgme.Context

  defdelegate create(opts), to: Context
  defdelegate list_keys(context), to: Context
  defdelegate import_key(context, data), to: Context
  # defdelegate epxort_key(context, key), to: Context
  defdelegate encrypt(context, key, data), to: Context
  defdelegate encrypt_symmetric(context, passphrase, data), to: Context
  defdelegate decrypt(context, passphrase, data), to: Context
  # defdelegate encrypt_and_sign(context, key, data), to: Context
  # defdelegate decrypt_and_verify(context, passphrase, data), to: Context
  defdelegate find_key(context, query), to: Context

  defdelegate signature_notations(context), to: Context
  defdelegate add_signature_notation(context, name, value, flags), to: Context
  defdelegate clear_signature_notations(context), to: Context

  defdelegate sender(context), to: Context
  defdelegate set_sender(context, sender), to: Context
  defdelegate clear_sender(context), to: Context

  # TODO:
  # defdelegate sign(context, mode = :normal|:clear|:detached, key, data), to: Context
  # defdelegate verify(context, mode = :normal|:detached|:opaque, passphrase, data), to: Context
  # defdelegate signers
  # defdelegate add_signer
  # defdelegate clear_signers
end
