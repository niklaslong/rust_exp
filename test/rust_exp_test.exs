defmodule RustExpTest do
  use ExUnit.Case
  doctest RustExp

  test "greets the world" do
    assert RustExp.hello() == :world
  end
end
