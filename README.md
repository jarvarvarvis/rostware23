# rostware23

Vollständige Re-Implementierung unseres [Java-Clients](https://github.com/goldos24/weichware23) und der Spiellogik in Rust.

Dieser Client und unser alter Java-Client verwenden (abgesehen von einigen Gewichten) die gleichen Bewertungsfunktionen und den gleichen Suchalgorithmus (PVS mit Iterative Deepening, Aspiration Search und Transposition Tables). 

Im Ordner [logic](src/logic) ist die Implementierung vorzufinden.

## Cient ausführen

Debug-Target:
```
cargo run
```

Release-Target (leicht längere Compilezeit wegen Compiler-Optimierungen und LTO, aber entsprechend bessere Performance):
```
cargo run --release
```

## Client für das Wettkampfsystem bauen

Rustup-Target hinzufügen (falls noch nicht installiert):
```sh
rustup target add x86_64-unknown-linux-musl
```

Von dort an kann die Binary mit dem folgenden Cargo-Alias gebuildet werden:
```sh
cargo build-contest
```

Um den Client hochzuladen, muss die Binary mit einem `start.sh`-Script in einer Zip-Datei gebundelt werden.
