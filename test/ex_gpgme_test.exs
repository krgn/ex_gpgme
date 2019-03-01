defmodule ExGpgmeTest do
  use ExUnit.Case
  doctest ExGpgme

  test "greets the world" do
    assert ExGpgme.hello() == :world
  end
end
