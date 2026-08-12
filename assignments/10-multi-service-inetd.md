# 과제 10: 설정 기반 다중 서비스와 종료 관리

## 선수 조건

- 단일 서비스 inetd가 연결마다 프로세스를 실행하고 정리한다.
- listener, service와 child process의 책임을 설명할 수 있다.

## 학습 목표

- 설정과 런타임 실행 책임을 분리한다.
- 여러 listener와 자식 프로세스의 수명을 조정한다.
- graceful shutdown 정책을 설계한다.

## 이번 과제에서 익힐 Rust

- 실행 설정을 struct와 enum으로 모델링하고 문자열을 typed value로 변환하는 방법
- `FromStr`, `Result`와 iterator의 `collect`로 여러 설정을 검증하는 방법
- `HashSet` 또는 `HashMap`으로 주소 중복을 검사하는 방법
- channel, `AtomicBool`과 `JoinHandle`을 이용해 여러 실행 흐름에 종료를 전달하고 기다리는 방법
- `Duration`과 `Instant`로 제한 시간을 표현하는 방법

## 검색 키워드

- 설정: `Rust FromStr custom type`, `Rust iterator collect Result`, `Rust custom error enum Display Error`
- 검증: `Rust HashSet duplicate detection`, `Rust validate config before start`
- 종료 전달: `Rust mpsc shutdown signal threads`, `Rust Arc AtomicBool Ordering`, `Rust JoinHandle graceful shutdown`, `Rust TcpListener set_nonblocking accept WouldBlock shutdown`
- 시간: `Rust std time Duration Instant timeout`, `Rust child try_wait kill wait`

## 문제

여러 포트와 실행 프로그램을 설정으로 정의하고, 미니 inetd가 각 서비스를 독립적으로 실행하게 만든다. 서버 종료 시 listener와 자식 프로세스의 정리 정책을 구현한다.

## 제한 조건

- 설정 파싱과 서비스 실행을 분리한다.
- 잘못된 설정을 실행 전에 검증한다.
- 같은 주소 또는 포트 충돌을 명시적으로 보고한다.
- graceful shutdown의 범위와 제한 시간을 정의한다.
- 설정 형식을 위해 큰 프레임워크를 추가하지 않는다.
- 한 서비스의 실패를 전체 서버 실패로 볼 조건을 명시한다.

## 구현 순서

1. 필요한 설정 필드와 불변식을 정의한다.
2. 파싱한 설정을 실행 가능한 설정으로 검증한다.
3. 서비스마다 listener와 연결 처리 흐름을 시작한다.
4. 종료 신호가 새 연결 수락을 멈추게 한다.
5. 진행 중인 연결과 자식 프로세스에 정상 종료 시간을 제공한다.
6. 제한 시간을 넘긴 작업의 강제 종료 정책을 적용한다.

## 단계별 힌트

1. 파싱된 문자열과 실행 가능한 설정을 같은 타입으로 쓰지 말고, 검증 전후에 어떤 불변식이 달라지는지 적는다.
2. 모든 설정을 먼저 읽고 주소 중복, 빈 명령과 잘못된 인자를 검증한 뒤 listener를 하나씩 연다.
3. 여러 listener 중 하나가 bind에 실패했을 때 이미 열린 listener들의 소유자가 scope를 벗어나도록 하면 `Drop`이 정리를 돕는다.
4. 종료는 `새 accept 중단`, `진행 중 연결 대기`, `자식 프로세스 종료`의 세 단계로 나눠 각각 알림과 완료 확인 방법을 정한다.
5. 표준 라이브러리만으로 첫 버전을 만들 때는 OS signal 처리보다 테스트 가능한 channel이나 관리용 입력으로 종료를 시작해도 된다.
6. `AtomicBool`은 단순 상태 전달에, channel은 사건 전달에 적합하다. 어느 쪽을 쓰든 기다리는 thread를 깨우는 방법까지 함께 설계한다.
7. flag나 channel에 종료 상태를 기록하는 것만으로 blocking `accept`가 깨어나지는 않는다. `set_nonblocking`과 제한된 대기, readiness API 또는 명시적 wake-up 중 한 방식을 선택한다.

## 완료 조건

- 두 개 이상의 서비스가 서로 다른 포트에서 동작한다.
- 한 서비스 실패가 다른 서비스의 정상 연결을 불필요하게 종료하지 않는다.
- 새 연결 중단, 진행 중 연결과 자식 프로세스에 대한 종료 정책이 일관된다.
- 잘못된 설정은 부분 실행 전에 발견된다.
- 실행 계층, 서비스 관리와 데이터 전달 책임을 설명할 수 있다.
- 새 클라이언트가 더 오지 않는 상태에서도 종료 요청 후 정한 제한 시간 안에 listener 실행 흐름이 끝난다.

## 직접 조사할 질문

- 설정값 검증은 파싱 시점과 실행 시점 중 어디에서 해야 하는가?
- 서버 종료 신호를 여러 실행 흐름에 어떻게 전달할 수 있는가?
- 자식 프로세스에 정상 종료 기회를 준 뒤 강제 종료하려면 어떤 정책이 필요한가?
- 일부 listener만 bind에 실패하면 전체 시작은 어떻게 처리해야 하는가?
- shutdown 완료를 기다리는 주체는 누구인가?

## 완료 기록

- 설정 형식과 검증 규칙:
- 서비스 격리 정책:
- graceful shutdown 단계:
- 강제 종료 조건:
- 가장 오래 막혔던 문제와 원인:

## 막혔을 때 질문 형식

> 과제 10을 진행 중이다. 설정 검증은 ___ 단계에서 하고 서비스 실패 정책은 ___이다. ___ 상황에서 일부만 실행된다. 정답 설계 대신 원자적으로 시작하려면 무엇을 판단해야 하는지 질문해줘.
