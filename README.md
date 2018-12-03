# rust-jsonparser-prettyprint-benchmark

## Why
To enjoy Rust

## Serde StreamDeserializer vs Pest JsonParser.
* SAX style Json 파싱.
* [pest](https://github.com/pest-parser/pest) : 소개문서에 파싱속도는 가장 빠르다 함.
* [serde](https://github.com/serde-rs/serde) : Rust 제공 프레임워크다.
* [nom](https://github.com/Geal/nom) : 코드 내 매크로로 문법을 정의해야 해서 개인 취향은 아님.

## 사전준비 
**rust nightly** => cargo bench 하기 쉽다.
```bash
curl https://sh.rustup.rs -sSf | sh
rustup update
rustup install nightly
```

또는 

```bash
rustup update
rustup default nightly
```

## 결과 => serde win!
```bash
cargo bench
...
running 4 tests
test pretty_print_pest1  ... bench:      40,202 ns/iter (+/- 1,097)
test pretty_print_pest2  ... bench:     147,521 ns/iter (+/- 3,320)
test pretty_print_serde1 ... bench:      24,277 ns/iter (+/- 1,157)
test pretty_print_serde2 ... bench:      79,667 ns/iter (+/- 2,671)
```

## 참고

### Pest
* 파싱자체는 빠르지만 파싱 결과를 읽을 때 부가 작업이 필요하다.
* Escaped unicode 문자열을 UTF8으로 출력하려면 별도 처리가 필요하다.
* [PEG](https://en.wikipedia.org/wiki/Parsing_expression_grammar) 을 작성하면 별도 tokenizer 코드가 필요 없다.

### Serde
* Rust 공식 serialize/deserialize 프레임워크다.
* Escaped unicode 문자열은 UTF8으로 자동 변환해준다.
* 기본 제공되지 않는 커스텀 파서 작성이 상대적으로 어렵다.
* Json object key는 기본 정렬되어 리턴된다. (Streaming 파싱이라 말하기가..)

## Pretty print 해보기

### pest
```
cargo build --release
curl -s https://raw.githubusercontent.com/freestrings/rust-jsonparser-prettyprint-benchmark/master/benches/data1.json \
    | ./target/release/rjq pest
```

### serde
```
cargo build --release
curl -s https://raw.githubusercontent.com/freestrings/rust-jsonparser-prettyprint-benchmark/master/benches/data1.json \
    | ./target/release/rjq serde
```