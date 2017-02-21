# really slow!!
defmodule Prime do
  def not_prime(2), do: false
  def not_prime(num) when num > 2 do
    2..num-1 |> Enum.any?(fn prime -> rem(num, prime) == 0 end)
  end
  def primes(mx) do
    2..mx |> Stream.filter(fn num -> !not_prime(num) end)
  end
end


Prime.primes(600851475143)
|> Enum.filter(fn num -> rem(600851475143, num) == 0 end)
|> Enum.to_list
|> List.last
|> IO.puts
