defmodule ExGpgme do
  @moduledoc """
  Documentation for ExGpgme.
  """

  alias ExGpgme.Context

  defdelegate create(opts), to: Context
  defdelegate list_keys(context), to: Context
  defdelegate import_key(context, data), to: Context
  defdelegate encrypt(context, key, data), to: Context
  defdelegate encrypt_symmetric(context, passphrase, data), to: Context
  defdelegate decrypt(context, passphrase, data), to: Context
  defdelegate find_key(context, query), to: Context
end
