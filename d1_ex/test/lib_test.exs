defmodule LibTest do
  use ExUnit.Case
  doctest Lib

  test "1+99 counts 1 time" do
    assert Lib.accumulate(%{rotation: +1, num: 99}, %{acc: 1, count: 0}).count == 1
  end

  test "1+199 counts 2 times" do
    assert Lib.accumulate(%{rotation: +1, num: 199}, %{acc: 1, count: 0}).count == 2
  end

  test "0+50 counts 0 times" do
    assert Lib.accumulate(%{rotation: +1, num: 50}, %{acc: 0, count: 0}).count == 0
  end

  test "0+199 counts 1 times" do
    assert Lib.accumulate(%{rotation: +1, num: 199}, %{acc: 0, count: 0}).count == 1
  end

  test "0+99 counts 0 times" do
    assert Lib.accumulate(%{rotation: +1, num: 99}, %{acc: 0, count: 0}).count == 0
  end

  test "0+100 counts 1 time" do
    assert Lib.accumulate(%{rotation: +1, num: 100}, %{acc: 0, count: 0}).count == 1
  end

  test "0+200 counts 2 times" do
    assert Lib.accumulate(%{rotation: +1, num: 200}, %{acc: 0, count: 0}).count == 2
  end

  test "0+300 counts 3 times" do
    assert Lib.accumulate(%{rotation: +1, num: 300}, %{acc: 0, count: 0}).count == 3
  end

  test "20+50 counts 0 times" do
    assert Lib.accumulate(%{rotation: +1, num: 20}, %{acc: 50, count: 0}).count == 0
  end

  test "99+199 counts 2 times" do
    assert Lib.accumulate(%{rotation: +1, num: 199}, %{acc: 99, count: 0}).count == 2
  end

  test "99-99 counts 1 times" do
    assert Lib.accumulate(%{rotation: -1, num: 99}, %{acc: 99, count: 0}).count == 1
  end

  test "90+10 counts 1 times" do
    assert Lib.accumulate(%{rotation: +1, num: 10}, %{acc: 90, count: 0}).count == 1
  end

  test "90+20 counts 1 time" do
    assert Lib.accumulate(%{rotation: +1, num: 20}, %{acc: 90, count: 0}).count == 1
  end

  test "90+110 counts 2 times" do
    assert Lib.accumulate(%{rotation: +1, num: 110}, %{acc: 90, count: 0}).count == 2
  end

  test "90+120 counts 2 times" do
    assert Lib.accumulate(%{rotation: +1, num: 120}, %{acc: 90, count: 0}).count == 2
  end

  test "1-1 counts 1 time" do
    assert Lib.accumulate(%{rotation: -1, num: 1}, %{acc: 1, count: 0}).count == 1
  end

  test "1-2 counts 1 time" do
    assert Lib.accumulate(%{rotation: -1, num: 2}, %{acc: 1, count: 0}).count == 1
  end

  test "1-99 counts 1 time" do
    assert Lib.accumulate(%{rotation: -1, num: 99}, %{acc: 1, count: 0}).count == 1
  end

  test "1-100 counts 1 times" do
    assert Lib.accumulate(%{rotation: -1, num: 100}, %{acc: 1, count: 0}).count == 1
  end

  test "1-101 counts 2 times" do
    assert Lib.accumulate(%{rotation: -1, num: 101}, %{acc: 1, count: 0}).count == 2
  end

  test "1-199 counts 2 times" do
    assert Lib.accumulate(%{rotation: -1, num: 199}, %{acc: 1, count: 0}).count == 2
  end

  test "1-201 counts 3 times" do
    assert Lib.accumulate(%{rotation: -1, num: 201}, %{acc: 1, count: 0}).count == 3
  end

  test "1-102 counts 2 times" do
    assert Lib.accumulate(%{rotation: -1, num: 102}, %{acc: 1, count: 0}).count == 2
  end

  test "1-306 counts 4 times" do
    assert Lib.accumulate(%{rotation: -1, num: 306}, %{acc: 1, count: 0}).count == 4
  end

  test "29-30 counts 1 times" do
    assert Lib.accumulate(%{rotation: -1, num: 30}, %{acc: 29, count: 0}).count == 1
  end

  test "0-5 counts 0 times" do
    assert Lib.accumulate(%{rotation: -1, num: 5}, %{acc: 0, count: 0}).count == 0
  end

  test "0-100 counts 1 time" do
    assert Lib.accumulate(%{rotation: -1, num: 100}, %{acc: 0, count: 0}).count == 1
  end

  test "0-200 counts 2 times" do
    assert Lib.accumulate(%{rotation: -1, num: 200}, %{acc: 0, count: 0}).count == 2
  end

  test "0-201 counts 2 times" do
    assert Lib.accumulate(%{rotation: -1, num: 201}, %{acc: 0, count: 0}).count == 2
  end

  test "5-5 counts 1 times" do
    assert Lib.accumulate(%{rotation: -1, num: 5}, %{acc: 5, count: 0}).count == 1
  end

  test "5-10 counts 1 times" do
    assert Lib.accumulate(%{rotation: -1, num: 10}, %{acc: 5, count: 0}).count == 1
  end

  test "50-1000 counts 10 times" do
    assert Lib.accumulate(%{rotation: -1, num: 1000}, %{acc: 50, count: 0}).count == 10
  end

  test "50-1050 counts 11 times" do
    assert Lib.accumulate(%{rotation: -1, num: 1050}, %{acc: 50, count: 0}).count == 11
  end

  test "50+50 counts 1 time" do
    assert Lib.accumulate(%{rotation: 1, num: 50}, %{acc: 50, count: 0}).count == 1
  end

  test "50+1000 counts 10 times" do
    assert Lib.accumulate(%{rotation: 1, num: 1000}, %{acc: 50, count: 0}).count == 10
  end

  test "50+1050 counts 11 times" do
    assert Lib.accumulate(%{rotation: 1, num: 1050}, %{acc: 50, count: 0}).count == 11
  end

  test "99+1 counts 1 times" do
    assert Lib.accumulate(%{rotation: 1, num: 1}, %{acc: 99, count: 0}).count == 1
  end

  test "99+2 counts 1 times" do
    assert Lib.accumulate(%{rotation: 1, num: 2}, %{acc: 99, count: 0}).count == 1
  end

  test "99+101 counts 2 times" do
    assert Lib.accumulate(%{rotation: 1, num: 101}, %{acc: 99, count: 0}).count == 2
  end

  test "99+100 counts 1 times" do
    assert Lib.accumulate(%{rotation: 1, num: 100}, %{acc: 99, count: 0}).count == 1
  end

  test "99+801 counts 9 times" do
    assert Lib.accumulate(%{rotation: 1, num: 801}, %{acc: 99, count: 0}).count == 9
  end

  test "1-801 counts 9 times" do
    assert Lib.accumulate(%{rotation: -1, num: 801}, %{acc: 1, count: 0}).count == 9
  end

  test "99-801 counts 8 times" do
    assert Lib.accumulate(%{rotation: -1, num: 801}, %{acc: 99, count: 0}).count == 8
  end

  @tag only: true
  test "additional cases" do
    assert Lib.accumulate(%{rotation: -1, num: 30}, %{acc: 50, count: 0}).count == 0
    assert Lib.accumulate(%{rotation: -1, num: 60}, %{acc: 50, count: 0}).count == 1
    assert Lib.accumulate(%{rotation: -1, num: 260}, %{acc: 50, count: 0}).count == 3
    assert Lib.accumulate(%{rotation: 1, num: 30}, %{acc: 50, count: 0}).count == 0
    assert Lib.accumulate(%{rotation: 1, num: 60}, %{acc: 50, count: 0}).count == 1
    assert Lib.accumulate(%{rotation: 1, num: 360}, %{acc: 50, count: 0}).count == 4
    assert Lib.accumulate(%{rotation: -1, num: 10}, %{acc: 0, count: 0}).count == 0
    assert Lib.accumulate(%{rotation: -1, num: 100}, %{acc: 0, count: 0}).count == 1
    assert Lib.accumulate(%{rotation: 1, num: 10}, %{acc: 0, count: 0}).count == 0
    assert Lib.accumulate(%{rotation: 1, num: 100}, %{acc: 0, count: 0}).count == 1
    assert Lib.accumulate(%{rotation: -1, num: 68}, %{acc: 50, count: 0}).count == 1
    assert Lib.accumulate(%{rotation: -1, num: 30}, %{acc: 82, count: 0}).count == 0
    assert Lib.accumulate(%{rotation: 1, num: 48}, %{acc: 52, count: 0}).count == 1
    assert Lib.accumulate(%{rotation: -1, num: 5}, %{acc: 0, count: 0}).count == 0
    assert Lib.accumulate(%{rotation: 1, num: 60}, %{acc: 95, count: 0}).count == 1
    assert Lib.accumulate(%{rotation: -1, num: 55}, %{acc: 55, count: 0}).count == 1
    assert Lib.accumulate(%{rotation: -1, num: 1}, %{acc: 0, count: 0}).count == 0
    assert Lib.accumulate(%{rotation: -1, num: 99}, %{acc: 99, count: 0}).count == 1
    assert Lib.accumulate(%{rotation: 1, num: 14}, %{acc: 0, count: 0}).count == 0
    assert Lib.accumulate(%{rotation: -1, num: 82}, %{acc: 14, count: 0}).count == 1
  end
end
