# 과제 07: HTTP keep-alive와 요청 반복

## 선수 조건

- 과제 06의 제한된 HTTP 요청 한 건을 정확히 처리한다.
- 요청 하나의 byte 범위를 식별할 수 있다.

## 학습 목표

- TCP 연결 수명과 HTTP 요청 수명을 구분한다.
- 하나의 연결에서 요청을 연속해서 파싱한다.
- timeout과 자원 점유 문제를 이해한다.

## 이번 과제에서 익힐 Rust

- parser의 진행 상태를 struct 또는 enum으로 표현하는 방법
- 함수가 파싱 결과와 소비한 byte 수를 함께 반환하는 방법
- `Vec<u8>`에서 처리한 범위와 남은 범위를 소유권 손실 없이 관리하는 방법
- `Option<Duration>`과 socket timeout API를 사용하는 방법
- 반복문 안에서 요청 단위의 임시 상태를 새로 만드는 방법

## 검색 키워드

- 상태 표현: `Rust parser consumed bytes return tuple`, `Rust enum incomplete complete error`
- 버퍼: `Rust Vec drain split_off`, `Rust preserve unread bytes buffer`
- timeout: `Rust TcpStream set_read_timeout Duration`, `Rust io ErrorKind WouldBlock TimedOut`
- HTTP: `HTTP/1.1 persistent connections Connection close`, `HTTP request message body Content-Length boundary`, `HTTP pipelining request framing`

## 문제

하나의 TCP 연결에서 여러 HTTP 요청을 순서대로 처리한다. 연결은 유지되지만 각 요청의 애플리케이션 결과는 이전 요청의 숨은 상태에 의존하지 않게 한다.

## 제한 조건

- 한 요청의 끝과 다음 요청의 시작을 구분한다.
- 연결 종료 조건을 명시한다.
- 지원하지 않는 transfer encoding과 body 정책을 명시한다.
- 첫 구현에서는 `Transfer-Encoding`과 양수 `Content-Length` 요청을 거부하고 연결을 닫는다. keep-alive는 body가 없거나 `Content-Length: 0`인 요청만 지원한다.
- 무한 keep-alive와 느린 클라이언트의 위험을 기록한다.
- 요청마다 이전 파싱 상태가 잘못 재사용되지 않게 한다.

## 구현 순서

1. 현재 요청 파서가 소비한 byte 수를 확인할 수 있게 한다.
2. 다음 요청에 속한 남은 byte를 보존한다.
3. 연결 종료 요청과 서버 측 종료 정책을 정한다.
4. 한 연결에서 두 요청을 연속해서 보낸다.
5. 느린 요청 또는 아무것도 보내지 않는 연결의 위험을 실험한다.

## 단계별 힌트

1. 과제 06 parser가 성공 여부만 반환한다면, 이제 요청이 끝난 위치 또는 소비한 길이도 호출자에게 알려줘야 한다.
2. 한 번의 `read`에 두 요청이 들어올 수 있다. 첫 요청 뒤의 byte를 버리거나 첫 요청 데이터와 섞지 않는 버퍼 정책을 정한다. 일반적으로 다음 요청은 header 끝이 아니라 현재 요청의 body까지 소비한 뒤 시작한다.
3. 연결 반복의 바깥에는 연결 수명 상태를, 안쪽에는 매 요청마다 초기화할 파싱 상태를 둔다.
4. `Connection: close`, 정상 EOF, timeout과 잘못된 요청이 각각 어느 반복을 끝내야 하는지 표로 정리한다.
5. 표준 라이브러리의 timeout은 `Result`의 오류 종류로 관찰된다. 오류를 모두 같은 연결 실패로 취급하기 전에 `ErrorKind`를 확인한다.

## 완료 조건

- 하나의 연결에서 두 개 이상의 요청과 응답을 처리한다.
- 클라이언트 또는 서버가 종료를 요청하면 연결을 올바르게 닫는다.
- 연결 상태와 애플리케이션 상태를 구분해 설명할 수 있다.
- 요청 간에 이전 요청의 데이터가 잘못 섞이지 않는다.
- 연결을 무한히 점유하는 클라이언트에 대한 정책을 설명한다.
- 한 번에 들어온 두 개의 body 없는 요청을 분리하며, 양수 `Content-Length` 뒤에 다른 요청처럼 보이는 byte가 붙어도 이를 다음 요청으로 처리하지 않고 연결을 닫는다.

## 직접 조사할 질문

- HTTP/1.1의 기본 연결 정책은 무엇인가?
- 요청 body가 존재할 때 다음 요청의 시작을 어떻게 찾는가?
- connection timeout은 어떤 자원 고갈을 막는가?
- pipelining과 순차 요청 처리는 어떤 차이가 있는가?
- stateless는 서버가 어떤 상태도 보관하면 안 된다는 뜻인가?

## 완료 기록

- 연결 종료 조건:
- 연속 요청 byte를 관리한 방법:
- timeout 또는 자원 제한 정책:
- stateless와 connection state의 차이:
- 가장 오래 막혔던 문제와 원인:

## 막혔을 때 질문 형식

> 과제 07을 진행 중이다. 한 연결에서 첫 요청 후 ___ bytes가 남고 두 번째 요청에서 ___ 문제가 생긴다. 정답 대신 내가 구분해야 할 상태를 알려줘.
