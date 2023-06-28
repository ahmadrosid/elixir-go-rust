defmodule WebCrawlerTest do
  use ExUnit.Case
  doctest WebCrawler

  test "greets the world" do
    assert WebCrawler.hello() == :world
  end
end
