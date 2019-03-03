defmodule ExGpgme.Bindings.Test do
  use Test.GpgmeCase

  describe "Context" do
    test "should create successfully", ctx do
      IO.inspect(ctx, label: "CONTEXT")
    end
  end
end
