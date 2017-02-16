Stream.unfold({0, 1}, fn {a, b} -> {a, {b, a+b}} end)
|> Enum.take_while(fn x -> x <= 4000000 end)
|> Enum.filter(fn x -> rem(x, 2) == 0 end)
|> Enum.sum
|> IO.puts
