# Chapter1

## Installation

### Install

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
> Rust is installed now. Great!

rustc --version
> rustc x.y.z (abcabcabc yyyy-mm-dd)
```

### Update

```bash
 rustup update
```

### Uninstall

```bash
rustup self uninstall
```

## Hello, World Test

### How to compile

rustc 는 rust 컴파일러로 실행가능한 바이너리 파일을 생성한다.

```bash
rustc main.rs
> main

./main
> Hello, World
```

운영체제와 상관없이 실행 가능하며, rust가 설치되어 있지 않은 클라이언트에서도 바이너리 파일을 실행할 수 있다.

### Cargo

cargo는 rust의 패키지 매니저이다. 필요한 라이브러리를 설치하거나 코드를 작성하는 데에 도움을 준다.

rust 설치 시 함께 설치되기 때문에 추가적인 작업은 필요 없다.

```bash
cargo --version // 설치 확인
```

**Createing a Project with cargo**

```bash
cargo new [PROJECT_NAME]
```

- [package]: project pacakge에 대한 정보 및 설정을 기입한다.
- [dependencies]: project에 필요한 라이브러리를 기입한다.

**src/main.rs**

자동으로 생성되는 [main.rs](http://main.rs) 파일로, src 폴더 아래에 존재한다.

**Building and Running Cargo Project**

Project Build: project를 debug/release 모드로 build하여 실행 파일들을 생성한다.

- release 모드는 최적화를 통해 코드 실행이 더 빠르다. 하지만 컴파일 시간이 길어진다.

```bash
cargo build // debug mode

cargo build --release // release mode
```

Project Run: Build 이후 파일 실행까지 한번에 진행한다.

- 이전에 build된 것에서 변경사항이 없는 경우, build 하지 않고 실행만 한다.

```bash
cargo run
```

Project Check: Project가 컴파일 가능한지 빠르게 검사한다.

```bash
cargo check
```

## etc

### rustfmt (Automatic Formatting)

rustfmt는 rust 컨벤션에 따라 코드를 Reformat한다.

- cargo 프레임워크 환경에서만 실행 가능하다.

**rustfmt 설치**

```bash
rustup component add rustfmt
```

**rustfmt 실행**

```bash
cargo fmt
```