defmodule ExGpgme.Bindings do
  @moduledoc """
  Documentation for ExGpgme.
  """

  use Rustler, otp_app: :ex_gpgme, crate: "gpgme_bindings"

  # When your NIF is loaded, it will override this function.
  def list_keys, do: :erlang.nif_error(:nif_not_loaded)
end
