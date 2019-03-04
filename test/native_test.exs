defmodule ExGpgme.Bindings.Test do
  use Test.GpgmeCase

  require Logger

  describe "Context" do
    test "should create successfully", ctx do
      Logger.debug(System.get_env("GNUPGHOME"))
      Logger.debug(inspect(ctx))
    end
  end
end
