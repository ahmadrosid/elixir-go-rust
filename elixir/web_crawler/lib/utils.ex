defmodule Utils do
  def measure(func) when is_function(func) do
    {microseconds, result} = :timer.tc(func)

    IO.puts(IO.ANSI.blue() <> "Finished in: #{microseconds / 1_000_000} seconds")

    result
  end

  def clean_url(uri) do
    uri = %{uri | query: nil}
    uri = %{uri | fragment: nil}
    uri
  end
end
