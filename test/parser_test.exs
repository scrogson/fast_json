defmodule Json.ParserTest do
  use ExUnit.Case, async: true

  import Json
  alias Json.Error

  test "reductions" do
    :erlang.system_monitor(self(), [{:long_schedule, 2}])
    on_exit fn ->
      IO.inspect Process.info(self(), :messages)
    end

    data = File.read!(Path.expand("../bench/data/issue90.json", __DIR__))
    count_reductions(data, &parse!/1)
  end

  defp count_reductions(data, fun) do
    parent = self()
    pid = spawn(fn ->
      me = self()
      start = :os.timestamp()
      r0 = Process.info(me, :reductions)
      _decoded = fun.(data)
      t = :timer.now_diff(:os.timestamp(), start)
      r1 = Process.info(me, :reductions)
      send(parent, {me, {t, r0, r1}})
    end)

    receive do
      {^pid, result} ->
        IO.inspect result
    end
  end

  test "numbers" do
    assert_raise Error, "Unexpected number in JSON at position 1", fn -> parse!("-") end
    assert_raise Error, "Unexpected number in JSON at position 3", fn -> parse!("--1") end
    # FIXME assert_raise Error, "Invalid Number", fn -> parse!("01") end
    assert_raise Error, "Unexpected token . at position 0", fn -> parse!(".1") end
    # FIXME assert_raise Error, "Invalid Number", fn -> parse!("1.") end
    # FIXME: should be "Unexpected end of JSON input"
    assert_raise Error, "Unexpected number in JSON at position 2", fn -> parse!("1e") end
    # FIXME: should be "Unexpected end of JSON input"
    assert_raise Error, "Unexpected number in JSON at position 5", fn -> parse!("1.0e+") end

    assert parse!("0") == 0
    assert parse!("01") == 01
    assert parse!("1") == 1
    assert parse!("-0") == 0
    assert parse!("-1") == -1
    assert parse!("0.1") == 0.1
    assert parse!("-0.1") == -0.1
    assert parse!("0e0") == 0
    assert parse!("0E0") == 0
    assert parse!("1e0") == 1
    assert parse!("1E0") == 1
    assert parse!("1.0e0") == 1.0
    assert parse!("1e+0") == 1
    assert parse!("1.0e+0") == 1.0
    assert parse!("0.1e1") == 0.1e1
    assert parse!("0.1e-1") == 0.1e-1
    assert parse!("99.99e99") == 99.99e99
    assert parse!("-99.99e-99") == -99.99e-99
    assert parse!("123456789.123456789e123") == 123456789.123456789e123
  end

  test "strings" do
    assert_raise Error, "Invalid or unexpected token at position 1", fn -> parse!(~s(")) end
    assert_raise Error, "Invalid or unexpected token at position 3", fn -> parse!(~s("\\")) end
    assert_raise Error, "Unexpected token k in JSON at position 2", fn -> parse!(~s("\\k")) end
    #FIXME assert_raise Error, "Unexpected end of JSON", fn -> parse!(<<34, 128, 34>>) end
    #FIXME assert_raise Error, "Unexpected end of JSON", fn -> parse!(~s("\\u2603\\")) end
    assert_raise Error, "Invalid or unexpected token at position 1", fn -> parse!(~s("Here's a snowman for you: ☃. Good day!)) end
    assert_raise Error, "Invalid or unexpected token at position 1", fn -> parse!(~s("𝄞)) end

    #FIXME assert parse!(~s("\\"\\\\\\/\\b\\f\\n\\r\\t")) == ~s("\\/\b\f\n\r\t)
    #FIXME assert parse!(~s("\\u2603")) == "☃"
    #FIXME assert parse!(~s("\\u2028\\u2029")) == "\u2028\u2029"
    #FIXME assert parse!(~s("\\uD834\\uDD1E")) == "𝄞"
    #FIXME assert parse!(~s("\\uD834\\uDD1E")) == "𝄞"
    #FIXME assert parse!(~s("\\uD799\\uD799")) == "힙힙"
    #FIXME assert parse!(~s("✔︎")) == "✔︎"
  end

  test "objects" do
    assert_raise Error, ~r"Unexpected end of JSON input", fn -> parse!("{") end
    assert_raise Error, ~r"Unexpected end of JSON input", fn -> parse!("{,") end
    assert_raise Error, ~r"Unexpected token } in JSON", fn -> parse!(~s({"foo"})) end
    assert_raise Error, ~r"Unexpected end of JSON input", fn -> parse!(~s({"foo": "bar",})) end

    assert parse!("{}") == %{}
    assert parse!(~s({"foo": "bar"})) == %{"foo" => "bar"}

    expected = %{"foo" => "bar", "baz" => "quux"}
    assert parse!(~s({"foo": "bar", "baz": "quux"})) == expected

    expected = %{"foo" => %{"bar" => "baz"}}
    assert parse!(~s({"foo": {"bar": "baz"}})) == expected
  end

  test "arrays" do
    assert_raise Error, ~r"Unexpected end of JSON", fn -> parse!("[") end
    assert_raise Error, "Unexpected token , at position 1", fn -> parse!("[,") end
    #FIXME assert_raise Error, "Unexpected token ] at position 5", fn -> parse!("[1,]") end

    assert parse!("[]") == []
    assert parse!("[1, 2, 3]") == [1, 2, 3]
    assert parse!(~s(["foo", "bar", "baz"])) == ["foo", "bar", "baz"]
    assert parse!(~s([{"foo": "bar"}])) == [%{"foo" => "bar"}]
  end

  test "whitespace" do
    assert_raise Error, ~r"Unexpected end of JSON", fn -> parse!("") end
    assert_raise Error, ~r"Unexpected end of JSON", fn -> parse!("    ") end

    assert parse!("  [  ]  ") == []
    assert parse!("  {  }  ") == %{}

    assert parse!("  [  1  ,  2  ,  3  ]  ") == [1, 2, 3]

    expected = %{"foo" => "bar", "baz" => "quux"}
    assert parse!(~s(  {  "foo"  :  "bar"  ,  "baz"  :  "quux"  }  )) == expected
  end

  test "atom keys"
    #hash = :erlang.phash2(:crypto.strong_rand_bytes(8))
    #assert_raise ArgumentError, fn -> parse!(~s({"key#{hash}": null}), keys: :atoms!) end

    #assert parse!(~s({"foo": "bar"}), keys: :atoms) == %{foo: "bar"}
    #assert parse!(~s({"foo": "bar"}), keys: :atoms!) == %{foo: "bar"}
  #end
end
