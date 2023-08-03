defmodule HavenTest do
  use ExUnit.Case
  doctest Haven

  test "greets the world" do
    assert Haven.hello() == :world
  end
end
