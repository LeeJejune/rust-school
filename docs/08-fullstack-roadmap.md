# 08. Rust 기초에서 풀스택까지

목표는 **학습 기록 앱 하나를 끝까지 연결하는 것**입니다. 제목·학습 시간·완료 상태를 입력하고,
DB에 저장하며, 새로고침과 서버 재시작 후에도 기록을 조회할 수 있게 만듭니다.
다음 내용은 앞으로 구현할 계획입니다. 현재 저장소에 HTTP 서버·DB·웹 화면이 구현된 것은 아닙니다.

일정은 주 6~8시간, 기본적인 프로그래밍 경험이 있다는 가정의 초안입니다.
프로그래밍·웹 개발이 모두 처음이면 1~3주와 화면 단계에 시간을 더 써서 12~16주로 늘립니다.
각 주의 완료 기준을 통과한 뒤 다음 단계로 갑니다.

## 어떤 기술 조합으로 갈까

| 경로 | 구성 | 선택 이유 | 추가 학습 |
| --- | --- | --- | --- |
| A: 기본 추천 | Rust + Axum + Tokio + SQLx + PostgreSQL / React + TypeScript + Vite | Rust 서버 학습과 브라우저 학습을 나눠 진행하기 좋음 | 언어가 둘이며 JSON 계약을 관리해야 함 |
| B: 화면도 Rust | 같은 API·DB / Leptos CSR | Rust로 화면 상태와 이벤트까지 연습 | WASM 빌드와 Leptos 반응성 학습 |
| B 확장 | Leptos SSR + Axum 통합 + SQLx | 서버 HTML 렌더링·server functions까지 학습 | 서버/브라우저 빌드 구분과 hydration 이해 |

이 선택은 학습 부담을 고려한 제안입니다. 라이브러리 성능 순위를 뜻하지 않습니다.
**Rust 언어를 먼저 익히는 것이 목표면 A를 기본으로, 모든 애플리케이션 코드를 Rust로 쓰는 것이 목표면 B를 선택합니다.**
B도 HTML·CSS·DOM·HTTP 지식이 필요하며, 브라우저에는 WASM과 로더 등이 전달됩니다.
기초 과정에서 두 화면을 동시에 구현하지 않습니다.

각 프로젝트의 공식 문서:
[Axum](https://docs.rs/axum/latest/axum/), [Tokio](https://tokio.rs/tokio/tutorial),
[SQLx](https://docs.rs/sqlx/latest/sqlx/), [Vite](https://vite.dev/guide/),
[Leptos의 CSR/SSR 선택](https://book.leptos.dev/getting_started/index.html).

## 8주 진행표

| 주 | 공부할 것 | 구현할 결과 | 완료 기준 |
| --- | --- | --- | --- |
| 1 | Cargo, 타입, 함수, 조건·반복 | 예제 01, ex01 | 작은 함수를 작성하고 check·run·test 사용 |
| 2 | move, Copy/Clone, 참조, 가변 대여, 슬라이스 | 예제 02~03, ex02~03 | E0382·E0499·E0502를 clone 없이 설명·수정 |
| 3 | struct/enum, Option/Result, trait, iterator, 모듈, 테스트 | 예제 04~06, ex04~06, CLI 확장 | 기록 추가·목록·완료를 CLI로 처리하고 파일 저장 후 다시 읽음 |
| 4 | HTTP 요청·응답, JSON, async/await, Tokio, Axum | 메모리 저장 API | 생성·목록·수정·삭제와 잘못된 입력·없는 ID 응답 테스트 |
| 5 | SQL, PostgreSQL, SQLx, migration, connection pool | 영속화된 API | DB를 빈 상태에서 생성하고 서버 재시작 후 기록 유지 |
| 6 | HTML/CSS, 폼·상태·네트워크 / 선택한 UI 프레임워크 | 목록·추가·완료 화면 | 로딩·빈 목록·입력 오류·서버 실패를 화면에 표시 |
| 7 | API와 화면 연결, 필터·페이지 조회, 통합 검증 | 실제 CRUD 흐름 | 브라우저에서 생성→조회→완료→삭제, 새로고침 후 DB와 일치 |
| 8 | release 빌드, 환경 설정, 로그, 배포·복구 | 실행 가능한 데모와 운영 문서 | 새 환경에서 README만 보고 기동하고 health·저장·복구 확인 |

3주차의 JSON 파일 저장에는 Serde/serde_json을 추가합니다. 파일을 잠깐 안 쓰고 4주차로 가도 되지만,
메모리 데이터가 프로세스 종료 시 사라지는 이유를 먼저 설명할 수 있어야 합니다.
여러 프로세스가 같은 JSON 파일을 동시에 수정하는 구조까지 확장하지 않고 DB 단계로 넘어갑니다.

매주 학습은 공식 문서 1~2시간, 구현 3~4시간, 오류 재현·테스트·기록 1~2시간으로 나눕니다.

## 데이터가 흐르는 경로

```text
브라우저의 폼과 화면
    │ HTTP / JSON
    ▼
Axum handler: 요청 해석, 상태 코드, 응답 변환
    │
    ▼
서비스·도메인: 제목·시간 검증, 완료 처리
    │
    ▼
SQLx: 파라미터 바인딩, 조회·저장
    │
    ▼
PostgreSQL
```

소유권은 같은 프로세스 안에서 값의 사용을 관리합니다. HTTP를 건너는 Rust 참조가 생기는 것은 아닙니다.
JSON으로 직렬화해 바이트를 보내고 상대가 새 값을 만듭니다.
메모리 단계의 Task와 API 요청 DTO·DB 행 모델을 무조건 같은 타입으로 합칠 필요는 없습니다.
Serde 파싱 뒤에도 Task::new 같은 도메인 검증을 거치도록 합니다.

## API 계약 초안

| 요청 | 입력 | 성공 응답 | 실패 경로 |
| --- | --- | --- | --- |
| GET /health | 없음 | 200 | 프로세스 생존 확인; DB 준비 상태는 별도로 설계 |
| POST /api/tasks | title, minutes | 201 + 생성 기록 | 잘못된 JSON 400, 도메인 검증 422 |
| GET /api/tasks?status=todo&limit=20&offset=0 | 선택 필터·페이지 | 200 + 목록 | 잘못된 조회 조건 400 |
| PATCH /api/tasks/{id} | title 또는 status | 200 + 수정 기록 | 잘못된 JSON/ID 400, 없는 ID 404, 검증 실패 422 |
| DELETE /api/tasks/{id} | ID | 204 | 잘못된 ID 400, 없는 ID 404 |

```json
{
  "id": 1,
  "title": "소유권 공부",
  "minutes": 30,
  "status": "todo"
}
```

ID는 서버가 만들고, status는 `todo` 또는 `done`으로 직렬화합니다.
최초 생성 입력은 title·minutes만 받으며 상태는 todo로 시작합니다.
limit은 1~100, offset은 0 이상으로 제한하고 id 기준 정렬을 고정합니다.
PATCH는 일부 필드 수정으로 정의하고, 빈 수정 요청과 정의하지 않은 status를 거부합니다.
오류 본문은 `{"code":"invalid_minutes","message":"..."}` 형태로 통일하는 것을 목표로 합니다.
Axum 기본 rejection 응답과 목표 계약이 다를 수 있으므로 handler 경계에서 명시적으로 매핑하고 테스트합니다.

## DB 단계의 구체적인 과제

1. tasks 테이블에 id, title, minutes, status, created_at을 만듭니다.
2. title 공백 검사, minutes 범위, status 허용값을 애플리케이션과 DB에 정의합니다.
3. migration 파일을 버전 관리하고 새 DB에서도 처음부터 실행합니다.
4. SQLx 연결 풀을 AppState로 공유합니다. SQL은 문자열 연결 대신 파라미터 바인딩을 사용합니다.
5. 없음을 404로, DB 장애를 서버 오류로 바꾸되 내부 접속 정보를 응답에 넣지 않습니다.
6. SQLx의 `query!`류 매크로를 선택하면 빌드 시 DB 또는 준비된 offline 메타데이터가 필요한지도 확인합니다.
7. 동시 수정 시 최신 상태가 어떻게 정해지는지 요구사항을 기록합니다. 여러 행을 함께 바꾸는 기능을 추가할 때 transaction을 연습합니다.

## 화면 단계의 구체적인 과제

처음에는 목록, 추가 폼, 완료 버튼 세 가지로 시작합니다.
요청 성공 후 상태를 갱신하고, 중복 제출을 막으며, 실패 메시지를 표시합니다.
폼 label, 키보드 제출, focus, 좁은 화면도 확인합니다.
프런트 개발 서버와 API가 다른 origin이면 개발 proxy 또는 범위를 정한 CORS 설정을 사용합니다.
CORS는 인증 기능이 아니라 브라우저의 교차 출처 접근 규칙이라는 점을 구분합니다.

B를 택하면 Leptos CSR과 기존 API부터 연결합니다. SSR로 전환할 때는
server functions를 도입해 기존 REST 경계 중 무엇을 유지할지 결정합니다.
DB 접속 정보와 SQL 실행은 서버에서만 다루고 브라우저 빌드에 포함하지 않습니다.

## 8주차 배포 완료 기준

- [ ] 서버 release 빌드와 선택한 화면의 production 빌드 성공
- [ ] DB URL 같은 환경값을 `.env.example`에 키와 예시로 설명
- [ ] 새 DB migration → 서버 기동 → health 확인 → CRUD 검증 순서 기록
- [ ] HTTPS, 요청 로그, 오류 로그, graceful shutdown 확인
- [ ] 데이터 백업과 복구 또는 데모 DB 재생성 절차 기록
- [ ] 개인 기록을 공개 서버에서 다룬다면 로그인·인가를 구현한 뒤 공개
- [ ] 생성·완료·새로고침·삭제 시나리오를 재현할 수 있는 데모 설명

공개 로그인까지 8주 안에 넣기 어렵다면 로컬 데모를 먼저 완성하고, 다중 사용자·세션·인가를 후속 단계로 둡니다.
기본 CRUD가 동작한 뒤 검색, 통계, 실시간 알림 등 기능을 하나씩 추가합니다.

## 오늘 할 일: 60~90분

1. 환경을 설치하고 `cargo run --example 01_basics`를 실행합니다.
2. `02_ownership`에서 이동한 String을 다시 출력해 보고 E0382를 읽습니다.
3. `03_borrowing`에서 같은 값을 빌려 읽고 수정하는 순서를 확인합니다.
4. ex01~ex02를 풀고, “왜 String은 이동하지만 u32는 계속 쓸 수 있는지”를 세 문장으로 기록합니다.
