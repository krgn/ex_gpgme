defmodule ExGpgme do
  @moduledoc """
  Documentation for ExGpgme.
  """

  alias ExGpgme.Key
  alias ExGpgme.Context

  def create(path: gnupg_home) do
    Context.create(gnupg_home)
  end

  def list_keys(%Context{} = context) do
    Context.list_keys(context)
  end

  def encrypt(%Context{} = context, %Key{} = key, data) when is_binary(data) do
    Context.encrypt(context, key, data)
  end

  def decrypt(%Context{} = context, passphrase, data)
      when is_binary(passphrase) and is_binary(data) do
    Context.decrypt(context, passphrase, data)
  end
end
