defmodule RustExpMathTest do
  use ExUnit.Case
  import RustExp.Math
  # doctest RustExp.Math

  test "greets the world" do
    assert hello() == :world
  end

  test "performs addition" do
    assert add(2, 3) == {:ok, 5}
  end

  test "performs subtraction" do
    assert subtract(5, 3) == {:ok, 2}
  end

  test "performs multiplication" do
    assert multiply(2, 3) == {:ok, 6}
  end

  test "performs division" do
    assert divide(10, 2) == {:ok, 5}
  end

  test "returns string passed as param" do
    assert repeat("hi") == {:ok, "hi"}
  end

  test "returns interpolated string" do
    assert put_string("chocolate") == {:ok, "You said: chocolate"}
  end
end
