# 과제 06: 최소 HTTP/1.1 서버

## 선수 조건

- TCP가 메시지 경계를 제공하지 않는다는 점을 설명할 수 있다.
- 길이 또는 구분자를 기준으로 입력을 누적해 본 경험이 있다.

## 학습 목표

- HTTP/1.1 요청과 응답의 최소 wire format을 이해한다.
- TCP 위에서 헤더 경계를 직접 찾는다.
- stateless 애플리케이션 요청의 의미를 이해한다.

## 이번 과제에서 익힐 Rust

- `b"..."` byte string literal과 UTF-8 `String`의 차이
- `Vec<u8>`에 여러 번 읽은 데이터를 누적하고 slice로 검사하는 방법
- `Option`과 `match`로 아직 경계를 못 찾은 상태를 표현하는 방법
- struct와 enum으로 파싱 결과 또는 제한된 method를 표현하는 방법
- `format!` 결과와 body byte 길이를 구분하는 방법

## 검색 키워드

- 언어: `Rust byte string literal`, `Rust Vec extend_from_slice`, `Rust Option match`, `Rust enum parser result`
- byte 검색: `Rust slice windows position byte sequence`, `Rust find CRLF CRLF bytes`
- HTTP 규칙: `HTTP/1.1 request line CRLF`, `HTTP Content-Length bytes`, `HTTP status line response format`
- 변환: `Rust from_utf8 byte slice`, `Rust parse integer from bytes`

## 문제

TCP 위에서 HTTP/1.1 요청을 직접 읽고, `GET /health`에는 성공 응답을, 알 수 없는 경로에는 404 응답을 반환한다.

## 제한 조건

- HTTP 서버 crate와 파서 crate를 사용하지 않는다.
- 요청과 응답을 UTF-8 문자열 하나로 무조건 변환하지 않는다.
- 헤더 끝을 정확히 판단한다.
- 요청 헤더의 최대 크기를 정한다.
- 처음에는 연결당 요청 하나만 처리해도 된다.
- 지원하는 method, target과 body 범위를 명시한다.

## 구현 순서

1. 지원할 요청 형식과 응답 형식을 예시 byte 수준으로 적는다.
2. 제한 크기 안에서 header terminator가 올 때까지 읽는다.
3. request line의 method, target과 version을 구분한다.
4. `/health`와 알 수 없는 경로의 응답을 만든다.
5. body byte 수와 헤더의 길이가 일치하는지 검증한다.
6. 불완전하거나 지원하지 않는 요청의 정책을 추가한다.

## 단계별 힌트

1. 처음부터 HTTP 전체 parser를 만들지 말고 지원할 request line, header 종료 표시와 두 응답의 raw bytes를 종이에 적는다.
2. 읽을 때마다 받은 범위만 `Vec<u8>`에 붙인다. 누적 길이가 최대 header 크기를 넘었는지 경계를 찾기 전에 확인한다.
3. header 종료는 byte sequence 검색 문제다. slice의 `windows`와 iterator의 `position`이 무엇을 반환하는지 조사한다.
4. request line처럼 ASCII여야 하는 작은 구간만 검증해 문자열로 보거나 byte slice 그대로 비교한다. 전체 요청을 무조건 `String`으로 만들지 않는다.
5. parser가 `완성`, `더 필요함`, `잘못된 요청`을 구분해야 한다면 `Option` 하나로 충분한지 별도 enum이 필요한지 판단한다.
6. 응답의 `Content-Length`는 Rust 문자열의 문자 수가 아니라 실제 body byte 수를 기준으로 계산한다.

## 완료 조건

- 브라우저 또는 `curl`에서 `/health` 응답을 확인할 수 있다.
- 알 수 없는 경로가 명시적인 404 응답을 반환한다.
- 응답의 status line, header 구분과 body 길이가 올바르다.
- 불완전하거나 너무 큰 요청을 정의한 방식으로 종료한다.
- 요청이 여러 TCP 읽기로 나뉘어도 header 경계를 찾는다.

## 직접 조사할 질문

- HTTP request line과 header는 어떤 byte sequence로 구분되는가?
- `Content-Length`가 실제 body byte 수와 일치해야 하는 이유는 무엇인가?
- TCP 읽기 한 번으로 전체 HTTP 요청이 온다고 가정하면 왜 안 되는가?
- HTTP가 stateless라는 말은 TCP 연결 상태와 어떻게 다른가?
- 임의의 요청 byte를 전부 UTF-8이라고 가정하면 어떤 문제가 있는가?

## 완료 기록

- 지원하는 HTTP 범위:
- 헤더 최대 크기와 그 이유:
- 불완전한 요청 처리 정책:
- `curl`로 검증한 사례:
- 가장 오래 막혔던 문제와 원인:

## 막혔을 때 질문 형식

> 과제 06을 진행 중이다. 받은 HTTP bytes는 ___이고 내가 찾은 경계는 ___이다. 예상 응답과 실제 응답은 각각 ___이다. 파서 코드를 주지 말고 잘못 이해한 규칙만 알려줘.
