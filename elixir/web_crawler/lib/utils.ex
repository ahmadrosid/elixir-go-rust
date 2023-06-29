defmodule Utils do
  def measure(func) when is_function(func) do
    {microseconds, result} = :timer.tc(func)

    IO.puts(IO.ANSI.blue() <> "Finished in: #{microseconds / 1_000_000} seconds")

    memory = :erlang.memory()
    print_memory(memory)
    result
  end

  def print_memory(memory) do
    total_bytes = memory[:total]
    process_bytes = memory[:processes]
    system_bytes = memory[:system]

    total_mb = bytes_to_megabytes(total_bytes)
    process_mb = bytes_to_megabytes(process_bytes)
    system_mb = bytes_to_megabytes(system_bytes)

    IO.puts("Total: #{total_mb} MB")
    IO.puts("Processes: #{process_mb} MB")
    IO.puts("System: #{system_mb} MB")
  end

  def bytes_to_megabytes(bytes) do
    megabytes = bytes / (1024 * 1024)
    Float.round(megabytes, 4)
  end

  def clean_url(uri) do
    uri = %{uri | query: nil}
    uri = %{uri | fragment: nil}
    uri
  end
end
