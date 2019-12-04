import Control.Monad (guard)
import Data.List (group)


decimal :: [Int] -> Int
decimal digits = sum (zipWith (\n p -> n * 10 ^ p) (reverse digits) [0..])


candidates :: [[Int]]
candidates = [[a,b,c,d,e,f] | a <- [1..9], b <- [a..9], c <- [b..9], d <- [c..9], e <- [d..9], f <- [e..9]]


hasRepetition :: Eq a => [a] -> Bool
hasRepetition xs = any (\(a,b) -> a == b) (zip xs (tail xs))


hasDouble :: Eq a => [a] -> Bool
hasDouble xs = any (\g -> length g == 2) (group xs)


solutions :: ([Int] -> Bool) -> Int
solutions pred = length . dropWhile (< 130254) . takeWhile (<= 678275) . map decimal . filter pred $ candidates


main :: IO ()
main = do
    putStrLn ("Part one: " ++ show (solutions hasRepetition))
    putStrLn ("Part two: " ++ show (solutions hasDouble))

