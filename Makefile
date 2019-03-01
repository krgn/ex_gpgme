build:
	@nix-shell -p gettext -p pkgconfig -p gpgme --run "mix compile"

iex:
	@nix-shell -p gettext -p pkgconfig -p gpgme --run "iex -S mix"

release:
	@nix-shell -p gettext -p pkgconfig -p gpgme --run "env MIX_ENV=release mix compile"
