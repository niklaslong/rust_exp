defmodule RustExp.Math do
  @moduledoc """
  Documentation for RustExp.
  """

  @doc """
  Hello world.

  ## Examples

      iex> RustExp.hello
      :world

  """
  def hello do
    :world
  end

  use Rustler, otp_app: :rust_exp, crate: :rustexp_math
  def err(), do: throw :nif_not_loaded
  # When your NIF is loaded, it will override this function.
  
  def add(_a, _b), do: err()
  def subtract(_a, _b), do: err()
  def multiply(_a, _b), do: err()
  def devide(_a, _b), do: err()
end
