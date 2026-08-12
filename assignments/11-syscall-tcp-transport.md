# 과제 11: OS 시스템 콜 기반 TCP 구현

## 선수 조건

- 과제 01~10의 프로토콜과 애플리케이션 동작이 테스트로 고정되어 있다.
- 전송 계층 교체 경계와 파일 디스크립터 개념을 설명할 수 있다.
- 현재 운영체제의 socket API 문서를 조사할 준비가 되어 있다.

## 학습 목표

- OS 소켓 ABI를 직접 호출한다.
- `unsafe` 경계와 안전한 Rust wrapper의 불변식을 설계한다.
- 기존 프로토콜을 바꾸지 않고 전송 구현을 교체한다.

## 이번 과제에서 익힐 Rust

- `unsafe` block과 `unsafe extern "C"` 선언이 각각 맡는 책임
- `#[repr(C)]`, raw pointer, C 정수 타입과 ABI 경계
- `RawFd`, `OwnedFd`, `AsRawFd`와 `FromRawFd`의 소유권 차이
- `Drop`과 RAII로 descriptor를 정확히 한 번 닫는 방법
- `std::io::Error::last_os_error`로 OS 오류를 Rust 오류로 옮기는 방법
- `MaybeUninit`과 출력 인자를 안전하게 초기화하는 규칙

## 검색 키워드

- FFI: `Rust 2024 unsafe extern C`, `Rust FFI repr C raw pointer`, `Rust Nomicon FFI`
- descriptor: `Rust std os fd OwnedFd RawFd FromRawFd`, `Rust OwnedFd drop close`
- 오류: `Rust io Error last_os_error`, `Rust syscall returns minus one errno`
- 출력 인자: `Rust MaybeUninit FFI output parameter`
- OS API: `macOS socket bind listen accept man page`, `Darwin sockaddr_in socklen_t byte order`, `macOS SO_NOSIGPIPE EPIPE SIGPIPE setsockopt`

## 문제

현재 프로토콜과 애플리케이션의 동작 테스트를 유지하면서, `std::net` 대신 OS 소켓 시스템 콜을 직접 호출하는 TCP 전송 구현을 추가한다.

## 제한 조건

- 구현 전에 기존 `std::net` 전송의 동작 계약을 테스트로 고정한다.
- 새 전송 구현 내부에서는 `TcpListener`와 `TcpStream`을 사용하지 않는다.
- `unsafe` 코드는 작은 경계에 가두고 각 전제조건을 설명한다.
- 파일 디스크립터 소유권과 정확히 한 번 닫히는 규칙을 정의한다.
- 프로토콜 모듈을 시스템 콜 구현 때문에 수정하지 않는다.
- 끊어진 socket에 쓰더라도 OS signal이 서버 프로세스 전체를 종료시키지 않고 연결별 `Result` 오류로 관찰되게 한다.
- 우선 현재 개발 운영체제 하나만 명시적으로 지원해도 된다.
- 외부 FFI crate 사용 여부와 직접 선언 범위를 먼저 결정하고 이유를 기록한다.

## 구현 순서

1. 두 전송 구현이 공유해야 할 외부 동작 테스트를 작성한다.
2. 현재 운영체제의 socket ABI, 타입과 오류 규칙을 조사한다.
3. raw file descriptor의 소유권 wrapper를 만든다.
4. socket 생성, 주소 binding과 listening을 단계별로 구현한다.
5. 연결 수락과 stream read/write 동작을 구현한다.
6. 모든 실패 경로에서 descriptor가 정확히 정리되는지 확인한다.
7. 기존 프로토콜을 두 전송 구현으로 각각 실행한다.

## 단계별 힌트

1. 먼저 지원 OS와 FFI 선언을 직접 쓸지 검증된 저수준 crate를 쓸지 결정한다. 이 선택은 `std::net`을 쓰는 것과는 다른 문제다.
2. 각 syscall의 C 시그니처를 기억으로 작성하지 말고 현재 OS header 또는 man page와 Rust 쪽 타입 크기를 대조한다.
3. `socket` 성공 직후 raw descriptor를 소유하는 wrapper로 옮겨 이후 모든 `?` 경로에서 자동 정리되게 한다.
4. `FromRawFd`는 소유권을 만든다는 안전 계약이 있으므로 같은 descriptor에 두 번 호출하지 않는다.
5. syscall 반환값을 확인한 직후 `last_os_error`를 만들어야 다른 호출이 errno를 바꾸지 않는다.
6. `accept`처럼 커널이 메모리에 값을 써주는 함수는 buffer 초기화 여부와 실제로 초기화된 범위를 구분한다.
7. `unsafe` 함수 밖에는 안전한 Rust 타입과 `Result`만 보이게 하고, 각 unsafe block 위에 포인터·길이·소유권 전제를 기록한다.
8. macOS에서는 `SO_NOSIGPIPE`를 조사해 연결이 끊긴 뒤 송신할 때 `SIGPIPE` 대신 `EPIPE`가 반환되도록 한다. 지원 OS가 다르면 그 OS의 동등한 정책을 확인한다.

## 완료 조건

- `std::net` 구현과 시스템 콜 구현 중 하나를 실행 시점 또는 빌드 구성에서 선택할 수 있다.
- 두 구현이 같은 외부 동작 테스트를 통과한다.
- 오류 번호가 Rust 오류 경계로 손실 없이 전달된다.
- 부분 읽기, 부분 쓰기, 중단된 시스템 콜과 연결 종료를 처리한다.
- 프로토콜 로직 변경 없이 기존 TCP 과제 동작을 재현한다.
- descriptor 이중 close 또는 누수가 없다는 근거를 제시한다.
- 상대가 먼저 연결을 끊은 뒤 송신해도 프로세스가 종료되지 않고 해당 연결에서 Rust I/O 오류가 반환된다.

## 직접 조사할 질문

- `socket`, `bind`, `listen`, `accept`, `recv`, `send`, `close`는 각각 무엇을 책임지는가?
- `sockaddr_in`, 주소 길이와 byte order는 어떻게 표현되는가?
- Rust 값의 소유권과 OS 파일 디스크립터 소유권은 어떻게 연결해야 하는가?
- 시스템 콜이 `EINTR`, `EAGAIN` 또는 부분 성공을 반환하면 무엇을 해야 하는가?
- `unsafe`가 필요한 이유와 안전한 wrapper가 보장해야 할 불변식은 무엇인가?
- 시스템 콜 wrapper와 프로토콜 로직 사이에서 오류를 어떻게 표현할 것인가?
- 현재 OS에서 끊어진 socket 쓰기가 `SIGPIPE`로 프로세스를 종료하지 않게 하려면 어떤 socket 옵션 또는 송신 flag가 필요한가?

## 완료 기록

- 지원 운영체제와 ABI 범위:
- `unsafe` 경계의 불변식:
- descriptor 소유권 및 정리 규칙:
- 두 구현의 공통 검증 결과:
- 가장 오래 막혔던 문제와 원인:

## 막혔을 때 질문 형식

> 과제 11을 진행 중이다. 호출 중인 syscall은 ___이고 인자/반환값을 ___라고 이해했다. 오류는 ___이며 descriptor 소유권은 ___에 있다. 코드를 작성하지 말고 잘못된 ABI 전제만 점검해줘.
