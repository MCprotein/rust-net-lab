# 과제 09: 단일 서비스 TCP inetd

## 선수 조건

- 다중 TCP 연결과 양방향 스트림 I/O를 처리할 수 있다.
- 프로토콜 규칙과 전송 책임이 분리되어 있다.
- 프로세스의 표준 입력, 출력과 종료 상태의 기본 개념을 조사했다.

## 학습 목표

- 소켓과 외부 프로세스의 표준 입출력을 연결한다.
- 네트워크 프로토콜을 모르는 범용 서비스 실행기를 만든다.
- 프로세스 수명과 연결 수명을 함께 관리한다.

## 이번 과제에서 익힐 Rust

- builder 형태의 `std::process::Command` API를 사용하는 방법
- `Stdio::piped`, `Child`, `ChildStdin`, `ChildStdout`의 소유권 관계
- `Option<T>` 안의 pipe를 `take`해 별도 실행 흐름으로 옮기는 방법
- `std::io::copy`와 `TcpStream::try_clone`으로 두 방향 I/O를 구성하는 방법
- `Child::wait`로 자식 프로세스의 종료와 자원 수거를 확인하는 방법

## 검색 키워드

- 프로세스: `Rust std process Command spawn`, `Rust Stdio piped Child stdin stdout`, `Rust Child wait zombie`
- 소유권: `Rust Option take ChildStdin`, `Rust move child stdout to thread`
- 스트림 연결: `Rust std io copy`, `Rust TcpStream try_clone`, `Rust TcpStream shutdown Write`
- 종료: `Rust bidirectional copy EOF process pipe`, `Rust child kill wait`

## 문제

정해진 TCP 포트에서 연결을 기다리다가 클라이언트가 접속하면 외부 프로그램을 실행한다. 클라이언트 입력은 자식 프로세스의 표준 입력으로, 자식 프로세스의 표준 출력은 클라이언트로 전달한다.

## 제한 조건

- 처음에는 서비스 하나와 TCP만 지원한다.
- inetd는 애플리케이션 프로토콜의 내용을 해석하지 않는다.
- 프로세스 생성 실패와 연결 실패를 구분한다.
- 클라이언트 또는 자식 프로세스가 먼저 종료되는 두 경우를 처리한다.
- 자식 프로세스가 좀비 프로세스로 남지 않게 한다.
- 실행할 프로그램과 인자는 코드 또는 단순 설정으로 명시한다.

## 구현 순서

1. 연결 하나와 프로세스 하나의 수명 관계를 그림으로 정리한다.
2. 연결 수락 시 외부 프로그램을 실행한다.
3. socket → child stdin 방향을 전달한다.
4. child stdout → socket 방향을 전달한다.
5. 한 방향의 EOF와 반대 방향의 종료 정책을 정한다.
6. 자식 프로세스의 종료 상태와 자원을 수거한다.
7. 여러 연결이 서로 독립적인 프로세스를 만드는지 확인한다.

## 단계별 힌트

1. 먼저 네트워크 없이 `Command`로 작은 프로그램을 실행하고 piped stdin/stdout에 byte를 넣고 받는 실험을 별도로 만든다.
2. `child.stdin`과 `child.stdout`이 `Option`인 이유를 확인하고, field를 꺼낼 때 부분 이동 오류가 발생하면 `take`를 조사한다.
3. socket → child stdin과 child stdout → socket은 동시에 진행될 수 있어야 한다. 한 함수에서 순서대로 `copy`하면 어떤 방향이 먼저 막히는지 생각한다.
4. 두 방향에서 같은 `TcpStream`을 써야 할 때 `try_clone`이 새 연결을 만드는지 같은 소켓 handle을 복제하는지 문서를 확인한다.
5. 한 방향의 EOF 뒤에 pipe 또는 socket의 write half를 닫아야 상대편이 종료를 인식할 수 있는지 실험한다.
6. data copy가 끝났다고 자식 프로세스가 수거된 것은 아니다. 최종 소유자가 `wait`를 정확히 한 번 호출하게 한다.

## 완료 조건

- 접속할 때마다 설정된 외부 프로그램이 새로 실행된다.
- 양방향 데이터 전달을 관찰할 수 있다.
- 연결 종료 후 관련 자원이 정리된다.
- 여러 연결이 서로 다른 프로세스와 연결된다.
- 네트워크 책임과 프로세스 책임이 분리되어 있다.

## 직접 조사할 질문

- 자식 프로세스의 stdin, stdout과 stderr는 어떻게 구성되는가?
- socket과 pipe의 읽기·쓰기 방향은 어떻게 대응하는가?
- 한 방향의 EOF를 다른 방향에 어떻게 전달해야 하는가?
- 프로세스 종료 상태는 누가 수거해야 하는가?
- `inetd`가 각 서비스 프로토콜을 몰라도 되는 이유는 무엇인가?

## 완료 기록

- 연결과 자식 프로세스의 수명 관계:
- 양방향 전달 구조:
- EOF 및 종료 정책:
- 프로세스 수거 방법:
- 가장 오래 막혔던 문제와 원인:

## 막혔을 때 질문 형식

> 과제 09를 진행 중이다. socket과 child pipe를 ___ 방식으로 연결했고 ___가 먼저 종료되면 ___ 현상이 생긴다. 전체 코드를 주지 말고 종료 전파에서 확인할 항목만 알려줘.
