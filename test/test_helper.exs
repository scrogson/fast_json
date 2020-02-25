ExUnit.start()

defmodule TestHelper do

  def count_reductions(data, fun) do
    parent = self()

    pid =
      spawn(fn ->
        me = self()
        start = System.system_time(:millisecond)
        {_, r0} = Process.info(me, :reductions)

        _ = fun.(data)

        done = System.system_time(:millisecond)
        {_, r1} = Process.info(me, :reductions)

        send(parent, {me, time: (done - start), starting_reds: r0, ending_reds: r1, diff: r1 - r0})
      end)

    receive do
      {^pid, result} -> result
    end
  end
end
