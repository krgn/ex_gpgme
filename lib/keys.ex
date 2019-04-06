defmodule ExGpgme.Key do
  @moduledoc """
  Documentation for Key.
  """

  alias __MODULE__

  @type t :: Key

  defstruct id: nil,
            fingerprint: nil,
            has_secret: false,
            can_encrypt: false,
            can_sign: false,
            can_certify: false,
            can_authenticate: false,
            is_revoked: false,
            is_expired: false,
            is_disabled: false,
            is_invalid: false,
            is_qualified: false,
            user_ids: []

  @doc """
  Get a key map.
  """
  def from(ref) when is_reference(ref) do
    {:ok, id} = ExGpgme.Native.key_id(ref)
    {:ok, fingerprint} = ExGpgme.Native.key_fingerprint(ref)
    {:ok, has_secret} = ExGpgme.Native.key_has_secret(ref)
    {:ok, can_encrypt} = ExGpgme.Native.key_can_encrypt(ref)
    {:ok, can_sign} = ExGpgme.Native.key_can_sign(ref)
    {:ok, can_certify} = ExGpgme.Native.key_can_certify(ref)
    {:ok, can_authenticate} = ExGpgme.Native.key_can_authenticate(ref)
    {:ok, is_revoked} = ExGpgme.Native.key_is_revoked(ref)
    {:ok, is_expired} = ExGpgme.Native.key_is_expired(ref)
    {:ok, is_disabled} = ExGpgme.Native.key_is_disabled(ref)
    {:ok, is_invalid} = ExGpgme.Native.key_is_invalid(ref)
    {:ok, is_qualified} = ExGpgme.Native.key_is_qualified(ref)
    {:ok, user_ids} = ExGpgme.Native.key_user_ids(ref)

    %Key{
      id: id,
      fingerprint: fingerprint,
      has_secret: has_secret,
      can_encrypt: can_encrypt,
      can_sign: can_sign,
      can_certify: can_certify,
      can_authenticate: can_authenticate,
      is_revoked: is_revoked,
      is_expired: is_expired,
      is_disabled: is_disabled,
      is_invalid: is_invalid,
      is_qualified: is_qualified,
      user_ids: user_ids
    }
  end
end
