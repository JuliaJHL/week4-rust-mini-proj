# A spelling corrector based on the Levenshtein distance
* It could accept a word and return the corrected word.
* I use `count_1w.txt` as the English word list/frequency data.

## project setup
1. Clone the repo:
```
git clone https://github.com/JuliaJHL/week4-rust-mini-proj.git
```
2. cd into the project:
```
cd week4-rust-mini-proj
```
3. compile the project
```
cargo build --release
```
4. run the project
```
cargo run
```

## examples
It would prompt `Enter a word:` when you run the project.

When you misspelled a word, it will give the corrected word.
![wrong](https://github.com/JuliaJHL/imgs_readme/blob/main/rustmini/wrong.png)
When the spelling is correct, it will prompt `Correct spelling`.
![right](https://github.com/JuliaJHL/imgs_readme/blob/main/rustmini/right.png)

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [count_1w.txt](https://norvig.com/ngrams/)
