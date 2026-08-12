# 과제 12: 동기 구현과 비동기 구현 비교

## 선수 조건

- 동기 서버의 기능과 실패 동작이 테스트로 고정되어 있다.
- thread-per-connection 모델의 장점과 한계를 설명할 수 있다.
- blocking I/O와 준비된 I/O의 개념을 조사했다.

## 학습 목표

- 동기와 비동기 실행 모델의 차이를 경험한다.
- task, cancellation과 backpressure를 코드로 확인한다.
- 측정 결과를 과장하지 않고 선택 기준을 세운다.

## 이번 과제에서 익힐 Rust

- `async fn`, `.await`와 `Future`가 실행 자체가 아니라 계산의 상태를 표현하는 방식
- runtime task와 OS thread 사이에서 `Send`, `Sync`가 다시 등장하는 이유
- task handle, `select`와 channel을 이용한 완료·취소 조정
- cancellation 시 local value의 `Drop`과 부분 완료된 I/O를 고려하는 방법
- `Instant`, atomic counter와 결과 struct로 측정값을 수집하는 방법

## 검색 키워드

- 언어: `Rust async fn Future await`, `Rust Future lazy polling`, `Rust async Send Sync error`
- dependency: `Rust Cargo.toml add dependency crate features`, `Tokio tutorial setup Cargo.toml features`
- runtime: `Tokio spawn task`, `Tokio AsyncReadExt AsyncWriteExt`, `Tokio select cancellation`, `Tokio graceful shutdown`
- 취소: `Rust cancellation safety async IO`, `Rust drop future cancellation`
- 측정: `Rust std time Instant benchmark`, `Rust AtomicUsize counter`, `benchmark warmup percentile latency`

## 문제

기존 동기 구현의 동작 계약을 유지하면서 일부 서버를 비동기 런타임으로 다시 구현한다. 단순히 코드를 변환하지 않고 두 실행 모델의 차이를 측정하고 설명한다.

## 제한 조건

- 동기 구현을 삭제하거나 덮어쓰지 않는다.
- 비교할 시나리오와 측정 항목을 먼저 정한다.
- 작은 부하 결과를 일반적인 성능 결론으로 과장하지 않는다.
- 프로토콜 의미와 실행 모델의 차이를 구분한다.
- 비동기 런타임의 문서와 blocking API 사용 규칙을 확인한다.
- 이 학습 과정의 기본 runtime은 Tokio로 한다. runtime 비교가 목적이 아니므로 첫 구현에서는 다른 runtime을 동시에 도입하지 않는다.

## 구현 순서

1. 비교할 기존 서버 하나를 선택한다.
2. 기능 계약과 실패 계약을 테스트로 고정한다.
3. 동시 연결 수, 처리량, 지연과 자원 사용 중 측정할 항목을 정한다.
4. 비동기 구현을 별도 경로로 만든다.
5. 정상, 느린 클라이언트와 종료 시나리오를 두 구현에서 실행한다.
6. 수치와 관찰 결과를 분리해 기록한다.

## 단계별 힌트

1. 먼저 동기 구현의 입력과 출력 테스트를 함수 경계에 고정해 async 전환과 프로토콜 변경을 동시에 하지 않는다.
2. Tokio 공식 tutorial의 setup을 따라 `Cargo.toml` dependency와 필요한 crate feature가 각각 무엇인지 확인한다. 처음부터 모든 feature를 켜기보다 runtime, macro, network와 I/O에 필요한 범위를 기록한다.
3. runtime 시작, socket accept, task spawn과 async read/write 네 가지를 작은 연결 실험으로 확인한다.
4. `.await`가 OS thread를 항상 새로 만들거나 항상 양보한다는 가정을 하지 말고 `Future` polling 모델을 먼저 확인한다.
5. 기존 blocking 함수가 async task 안에 남아 있으면 어떤 executor thread를 막는지 실험한다.
6. task를 취소했을 때 lock guard, buffer와 연결이 `Drop`되는 것만으로 프로토콜 상태까지 안전한지 구분한다.
7. 두 구현을 같은 입력, 같은 시간과 같은 측정 코드로 실행하고 평균 하나보다 시행 횟수와 분포를 함께 기록한다.

## 완료 조건

- 두 구현이 같은 기능 계약을 만족한다.
- 스레드, task, blocking, cancellation과 backpressure 차이를 설명한다.
- 같은 시나리오에서 얻은 비교 가능한 측정 결과가 있다.
- 어떤 조건에서 어느 구현을 선택할지 자신의 기준을 정리한다.
- 비동기 환경에서 blocking 작업이 미치는 영향을 설명한다.

## 직접 조사할 질문

- OS 스레드와 비동기 task는 어떤 관계인가?
- `.await`는 무엇을 보장하고 무엇을 보장하지 않는가?
- cancellation safety는 왜 필요한가?
- backpressure가 없으면 느린 소비자 때문에 어떤 일이 생기는가?
- 벤치마크에서 warm-up, 반복 횟수와 환경 차이를 왜 기록해야 하는가?

## 완료 기록

- 선택한 비교 대상과 이유:
- 비교 시나리오 및 측정 항목:
- 동기 구현 결과:
- 비동기 구현 결과:
- 선택 기준과 한계:

## 막혔을 때 질문 형식

> 과제 12를 진행 중이다. 동기/비동기 구현을 ___ 시나리오로 비교했고 측정값은 ___이다. 결론을 대신 내려주지 말고 비교가 공정하지 않은 조건만 지적해줘.
