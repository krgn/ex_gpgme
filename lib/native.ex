defmodule ExGpgme.Native do
  @moduledoc """
  Documentation for ExGpgme.
  """

  use Rustler,
    otp_app: :ex_gpgme,
    # load_data: Application.get_env(:ex_gpgme, :gnupg_home, System.get_env("GNUPGHOME")),
    crate: "gpgme_bindings"

  # key related
  def key_list(_ctx),
    do: :erlang.nif_error(:nif_not_loaded)

  def key_id(_key_ref),
    do: :erlang.nif_error(:nif_not_loaded)

  def key_user_ids(_key_ref),
    do: :erlang.nif_error(:nif_not_loaded)

  def key_subkeys(_key_ref),
    do: :erlang.nif_error(:nif_not_loaded)

  def key_fingerprint(_key_ref),
    do: :erlang.nif_error(:nif_not_loaded)

  def key_can_encrypt(_key_ref),
    do: :erlang.nif_error(:nif_not_loaded)

  def key_can_sign(_key_ref),
    do: :erlang.nif_error(:nif_not_loaded)

  def key_can_certify(_key_ref),
    do: :erlang.nif_error(:nif_not_loaded)

  def key_can_authenticate(_key_ref),
    do: :erlang.nif_error(:nif_not_loaded)

  def key_has_secret(_key_ref),
    do: :erlang.nif_error(:nif_not_loaded)

  def key_is_revoked(_key_ref),
    do: :erlang.nif_error(:nif_not_loaded)

  def key_is_expired(_key_ref),
    do: :erlang.nif_error(:nif_not_loaded)

  def key_is_disabled(_key_ref),
    do: :erlang.nif_error(:nif_not_loaded)

  def key_is_invalid(_key_ref),
    do: :erlang.nif_error(:nif_not_loaded)

  def key_is_qualified(_key_ref),
    do: :erlang.nif_error(:nif_not_loaded)

  # context related
  def context_create(_protocol, _path),
    do: :erlang.nif_error(:nif_not_loaded)

  def context_info(_ctx),
    do: :erlang.nif_error(:nif_not_loaded)

  def context_import(_ctx, _data),
    do: :erlang.nif_error(:nif_not_loaded)

  def context_encrypt(_ctx, _key, _data),
    do: :erlang.nif_error(:nif_not_loaded)

  def context_encrypt_symmetric(_ctx, _passphrase, _data),
    do: :erlang.nif_error(:nif_not_loaded)

  def context_decrypt(_ctx, _key, _data),
    do: :erlang.nif_error(:nif_not_loaded)

  def context_find_key(_ctx, _query),
    do: :erlang.nif_error(:nif_not_loaded)

  def context_signature_notations(_ctx),
    do: :erlang.nif_error(:nif_not_loaded)

  def context_add_signature_notation(_ctx, _name, _value, _flags),
    do: :erlang.nif_error(:nif_not_loaded)

  def context_clear_signature_notations(_ctx),
    do: :erlang.nif_error(:nif_not_loaded)

  def context_sender(_ctx),
    do: :erlang.nif_error(:nif_not_loaded)

  def context_set_sender(_ctx, _sender),
    do: :erlang.nif_error(:nif_not_loaded)

  def context_clear_sender(_ctx),
    do: :erlang.nif_error(:nif_not_loaded)

  def context_signers(_ctx),
    do: :erlang.nif_error(:nif_not_loaded)

  def context_add_signer(_ctx, _signers),
    do: :erlang.nif_error(:nif_not_loaded)

  def context_clear_signers(_ctx),
    do: :erlang.nif_error(:nif_not_loaded)
end
