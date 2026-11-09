# 00. 개발 환경과 IDE

## 추천 조합

| 도구 | 역할 | 선택 기준 |
| --- | --- | --- |
| rustup | Rust 도구 버전 관리 | 공식 설치 경로 |
| rustc | Rust 컴파일러 | 소스 코드를 검사하고 컴파일 |
| Cargo | 빌드·실행·테스트·의존성 관리 | 작업은 대부분 cargo로 수행 |
| rustfmt / Clippy | 포맷 / 코드 진단 | 저장소 설정과 함께 사용 |
| VS Code + rust-analyzer | 편집·타입 정보·자동 완성 | 기본 추천 |
| CodeLLDB | 중단점·변수 관찰 | VS Code에서 디버깅할 때 |
| RustRover | Cargo·디버깅 등이 통합된 IDE | JetBrains 도구에 익숙할 때 |

VS Code의 Rust 지원은 [Microsoft 안내](https://code.visualstudio.com/docs/languages/rust)와
[rust-analyzer 설치 문서](https://rust-analyzer.github.io/book/installation.html)를 따릅니다.
RustRover는 [공식 제품 페이지](https://www.jetbrains.com/rust/)에서 비상업 용도 무료 이용을 안내합니다.
상업적 사용 여부에 따른 조건은 사용 시점의 라이선스를 확인하세요.

## macOS 설치 순서

1. 터미널에서 `xcode-select -p`로 Command Line Tools가 있는지 확인합니다.
   없다면 `xcode-select --install`로 설치합니다.
2. [Rust 공식 설치 페이지](https://rust-lang.org/tools/install/)의 rustup 설치 명령을 실행합니다.
3. 터미널을 새로 열고 아래를 확인합니다.

```sh
rustup update stable
rustup component add rustfmt clippy rust-src
rustc --version
cargo --version
```

Windows는 공식 설치 페이지의 rustup 설치 프로그램과 MSVC 빌드 도구 안내를 따릅니다.
Linux는 배포판의 C 링커·개발 도구와 rustup을 준비합니다.

## VS Code에서 열기

1. `Cargo.toml`이 있는 저장소 루트 폴더를 엽니다.
2. `.vscode/extensions.json`에 있는 rust-analyzer, CodeLLDB, Even Better TOML을 설치합니다.
3. `cargo check`를 실행한 뒤 함수 위에 마우스를 올려 타입과 설명을 확인합니다.
4. 예제의 `main` 위에 뜨는 Run / Debug를 사용합니다.
5. `src/main.rs`를 디버깅하려면 rust-analyzer의 Debug 동작을 사용하고, 실행 인수로
   `"소유권 공부"`, `30`을 지정합니다. 중단점을 `Task::new`에 걸고 title의 이동을 관찰합니다.

확장 설치는 저장소 추천 목록을 읽어 직접 선택합니다. 프로젝트 설정은 저장 시 포맷과 Clippy 진단을 켭니다.
`rust-toolchain.toml`의 stable은 시간이 지나면 바뀝니다. 재현성이 필요한 배포 단계에서는
검증한 버전으로 고정하고 업데이트를 별도 변경으로 다루세요.

## 자주 쓰는 명령

```sh
cargo check                         # 실행 파일 생성 없이 빠른 타입 검사
cargo run --example 02_ownership     # 예제 실행
cargo test                          # 코드 및 문서 테스트
cargo fmt                           # 포맷 적용
cargo clippy --all-targets           # 개선할 코드 확인
rustc --explain E0382                # 오류 코드 해설
rustup doc --book                    # 설치된 공식 Book 열기
```

도구가 없다고 나오면 터미널을 다시 열고 PATH에 `~/.cargo/bin`이 포함됐는지 확인합니다.
처음 설치하거나 도구 채널을 갱신할 때는 네트워크가 필요합니다.
