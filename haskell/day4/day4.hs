import Data.List (group)


hasRepetition :: Eq a => [a] -> Bool
hasRepetition xs = any (\(a,b) -> a == b) (zip xs (tail xs))


hasDouble :: Eq a => [a] -> Bool
hasDouble xs = any (\g -> length g == 2) (group xs)


solutions :: [[Int]] -> Int
solutions = length . dropWhile (< 130254) . takeWhile (<= 678275) . map decimal
  where
    decimal digits = sum (zipWith (\n p -> n * 10 ^ p) (reverse digits) [0..])


main :: IO ()
main = do
    let candidates = [[a,b,c,d,e,f] | a <- [1..9], b <- [a..9], c <- [b..9], d <- [c..9], e <- [d..9], f <- [e..9]]

    let withRepetitions = filter hasRepetition candidates
    let withDoubles = filter hasDouble withRepetitions

    putStrLn ("Part one: " ++ show (solutions withRepetitions))
    putStrLn ("Part two: " ++ show (solutions withDoubles))

