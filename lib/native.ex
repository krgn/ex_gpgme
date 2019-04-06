defmodule ExGpgme.Native do
  @moduledoc """
  Documentation for ExGpgme.
  """

  use Rustler,
    otp_app: :ex_gpgme,
    # load_data: Application.get_env(:ex_gpgme, :gnupg_home, System.get_env("GNUPGHOME")),
    crate: "gpgme_bindings"

  def key_list(_ctx), do: :erlang.nif_error(:nif_not_loaded)
  def key_id(_key_ref), do: :erlang.nif_error(:nif_not_loaded)
  def key_user_ids(_key_ref), do: :erlang.nif_error(:nif_not_loaded)
  def key_fingerprint(_key_ref), do: :erlang.nif_error(:nif_not_loaded)
  def key_can_encrypt(_key_ref), do: :erlang.nif_error(:nif_not_loaded)

  def context_create(_protocol, _path), do: :erlang.nif_error(:nif_not_loaded)
  def context_info(_ctx), do: :erlang.nif_error(:nif_not_loaded)
end
