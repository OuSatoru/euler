Seq.unfold (fun a -> Some(fst a + snd a, (snd a, fst a + snd a))) (0, 1)
|> Seq.takeWhile(fun x -> x <= 4000000)
|> Seq.filter(fun x -> x%2 = 0)
|> Seq.sum
|> printf "%d"
