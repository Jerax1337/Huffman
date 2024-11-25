Мини-архиватор, реализованный на языке программирования Rust. Программа позволяет сжимать и распаковывать файлы, используя алгоритм Хаффмана — один из самых известных алгоритмов сжатия без потерь.
./target/release/Project -f ./target/release/input.txt -o ./target/release/compressed.txt -c (Команда для сжатия файла)
./target/release/Project -f ./target/release/compressed.txt -o ./target/release/output.txt -c - (Команда для распаковки) (не доделал, не работает)
