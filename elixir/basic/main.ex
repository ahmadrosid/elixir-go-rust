# defmodule Counter do
#   def start do
#     initial_state = 0
#     spawn_link(__MODULE__, :loop, [initial_state])
#   end

#   def loop(state) do
#     receive do
#       {:increment} ->
#         new_state = state + 1
#         IO.puts("Current count: #{new_state}")
#         loop(new_state)
      
#       {:decrement} ->
#         new_state = state - 1
#         IO.puts("Current count: #{new_state}")
#         loop(new_state)
      
#       {:get_count, pid} ->
#         send(pid, state)
#         loop(state)

#       :terminate ->
#         IO.puts("Counter terminated.")
#     end
#   end
# end

# # Start the counter process
# counter_pid = Counter.start()

# # Send messages to the counter
# send(counter_pid, {:increment})
# send(counter_pid, {:increment})
# send(counter_pid, {:decrement})

# # Request the count
# send(counter_pid, {:get_count, self()})

# # Receive the count
# receive do
#   count ->
#     IO.puts("Received count: #{count}")
# end

# # Terminate the counter
# send(counter_pid, :terminate)

process_pid = spawn(fn ->
  IO.puts("Starting process")
  receive do
    {:print_hello} ->
      IO.puts("Hello 👋")
  end
end)

IO.puts("PID: #{inspect(process_pid)}")

send(process_pid, {:print_hello})
