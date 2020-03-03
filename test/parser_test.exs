defmodule Json.ParserTest do
  use ExUnit.Case, async: true

  import TestHelper
  import Json
  import Json.Native
  alias Json.Error

  test "reductions" do
    data = File.read!(Path.expand("../bench/data/issue90.json", __DIR__))

    data
    |> count_reductions(&Json.parse/1)
    |> IO.inspect()
  end

  test "large input" do
    data = File.read!(Path.expand("../bench/data/issue90.json", __DIR__))
    {:ok, expected} = decode_naive(data)
    assert expected == decode!(data)
  end

  test "threaded" do
    data = File.read!(Path.expand("../bench/data/issue90.json", __DIR__))
    {:ok, expected} = decode(data)
    {:ok, actual} = threaded_decode(data)
    assert actual == expected
  end

  test "numbers" do
    assert_raise Error, "Unexpected end of JSON", fn -> decode!("-") end
    assert_raise Error, "Unexpected character: - at (1:2)", fn -> decode!("--1") end
    assert_raise Error, "Unexpected character: 1 at (1:2)", fn -> decode!("01") end
    assert_raise Error, "Unexpected character: . at (1:1)", fn -> decode!(".1") end
    assert_raise Error, "Unexpected end of JSON", fn -> decode!("1.") end
    # FIXME: should be "Unexpected end of JSON input"
    assert_raise Error, "Unexpected end of JSON", fn -> decode!("1e") end
    # FIXME: should be "Unexpected end of JSON input"
    assert_raise Error, "Unexpected end of JSON", fn -> decode!("1.0e+") end

    assert decode!("0") == 0
    assert decode!("01") == 01
    assert decode!("1") == 1
    assert decode!("-0") == 0
    assert decode!("-1") == -1
    assert decode!("0.1") == 0.1
    assert decode!("-0.1") == -0.1
    assert decode!("0e0") == 0
    assert decode!("0E0") == 0
    assert decode!("1e0") == 1
    assert decode!("1E0") == 1
    assert decode!("1.0e0") == 1.0
    assert decode!("1e+0") == 1
    assert decode!("1.0e+0") == 1.0
    assert decode!("0.1e1") == 0.1e1
    assert decode!("0.1e-1") == 0.1e-1
    assert decode!("99.99e99") == 99.99e99
    assert decode!("-99.99e-99") == -99.99e-99
    assert decode!("123456789.123456789e123") == 123_456_789.123456789e123
  end

  test "strings" do
    assert_raise Error, "Invalid or unexpected token at position 1", fn -> decode!(~s(")) end
    assert_raise Error, "Invalid or unexpected token at position 3", fn -> decode!(~s("\\")) end
    assert_raise Error, "Unexpected token k in JSON at position 2", fn -> decode!(~s("\\k")) end
    # FIXME assert_raise Error, "Unexpected end of JSON", fn -> decode!(<<34, 128, 34>>) end
    # FIXME assert_raise Error, "Unexpected end of JSON", fn -> decode!(~s("\\u2603\\")) end
    assert_raise Error, "Invalid or unexpected token at position 1", fn ->
      decode!(~s("Here's a snowman for you: ☃. Good day!))
    end

    assert_raise Error, "Invalid or unexpected token at position 1", fn -> decode!(~s("𝄞)) end

    assert decode!(~s("\\"\\\\\\/\\b\\f\\n\\r\\t")) == ~s("\\/\b\f\n\r\t)
    assert decode!(~s("\\u2603")) == "☃"
    assert decode!(~s("\\u2028\\u2029")) == "\u2028\u2029"
    assert decode!(~s("\\uD834\\uDD1E")) == "𝄞"
    assert decode!(~s("\\uD834\\uDD1E")) == "𝄞"
    assert decode!(~s("\\uD799\\uD799")) == "힙힙"
    assert decode!(~s("✔︎")) == "✔︎"
  end

  test "objects" do
    assert_raise Error, ~r"Unexpected end of JSON input", fn -> decode!("{") end
    assert_raise Error, ~r"Unexpected end of JSON input", fn -> decode!("{,") end
    assert_raise Error, ~r"Unexpected token } in JSON", fn -> decode!(~s({"foo"})) end
    assert_raise Error, ~r"Unexpected end of JSON input", fn -> decode!(~s({"foo": "bar",})) end

    assert decode!("{}") == %{}
    assert decode!(~s({"foo": "bar"})) == %{"foo" => "bar"}

    expected = %{"foo" => "bar", "baz" => "quux"}
    assert decode!(~s({"foo": "bar", "baz": "quux"})) == expected

    expected = %{"foo" => %{"bar" => "baz"}}
    assert decode!(~s({"foo": {"bar": "baz"}})) == expected
  end

  test "arrays" do
    assert_raise Error, ~r"Unexpected end of JSON", fn -> decode!("[") end
    assert_raise Error, "Unexpected token , at position 1", fn -> decode!("[,") end
    # FIXME assert_raise Error, "Unexpected token ] at position 5", fn -> decode!("[1,]") end

    assert decode!("[]") == []
    assert decode!("[1, 2, 3]") == [1, 2, 3]
    assert decode!(~s(["foo", "bar", "baz"])) == ["foo", "bar", "baz"]
    assert decode!(~s([{"foo": "bar"}])) == [%{"foo" => "bar"}]
  end

  test "whitespace" do
    assert_raise Error, ~r"Unexpected end of JSON", fn -> decode!("") end
    assert_raise Error, ~r"Unexpected end of JSON", fn -> decode!("    ") end

    assert decode!("  [  ]  ") == []
    assert decode!("  {  }  ") == %{}

    assert decode!("  [  1  ,  2  ,  3  ]  ") == [1, 2, 3]

    expected = %{"foo" => "bar", "baz" => "quux"}
    assert decode!(~s(  {  "foo"  :  "bar"  ,  "baz"  :  "quux"  }  )) == expected
  end

  test "atom keys"
  # hash = :erlang.phash2(:crypto.strong_rand_bytes(8))
  # assert_raise ArgumentError, fn -> decode!(~s({"key#{hash}": null}), keys: :atoms!) end

  # assert decode!(~s({"foo": "bar"}), keys: :atoms) == %{foo: "bar"}
  # assert decode!(~s({"foo": "bar"}), keys: :atoms!) == %{foo: "bar"}
  # end
end
