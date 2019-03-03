defmodule ExGpgme.Bindings do
  @moduledoc """
  Documentation for ExGpgme.
  """

  use Rustler, otp_app: :ex_gpgme, crate: "gpgme_bindings"

  def list_keys(_ctx), do: :erlang.nif_error(:nif_not_loaded)
  def create_context, do: :erlang.nif_error(:nif_not_loaded)
end
