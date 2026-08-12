# Rust Network Lab Roadmap

이 문서는 전체 학습 순서와 진행 상태만 관리한다. 각 문제의 요구사항, 제한 조건, 완료 기준과 조사할 질문은 [`assignments/`](./assignments/) 아래의 과제별 문서에 있다.

## 진행 규칙

- 과제는 번호 순서대로 진행한다.
- 현재 과제의 완료 조건을 모두 검증한 뒤 다음 과제로 넘어간다.
- 필수 과제 01~12를 먼저 진행하고 과제 13은 선택적으로 진행한다.
- 과제를 완료하면 이 문서의 체크박스를 갱신하고 학습 내용을 커밋한다.
- 막혔을 때는 문서 검색 → 작은 실험 → 단계별 힌트 요청 순서로 해결한다.

## Phase 1. TCP 스트림

- [ ] [과제 01: 단일 클라이언트 TCP Echo Server](./assignments/01-tcp-echo-server.md)
- [ ] [과제 02: 길이 기반 메시지 프레이밍](./assignments/02-length-prefixed-framing.md)
- [ ] [과제 03: 여러 TCP 클라이언트 동시 처리](./assignments/03-concurrent-tcp-clients.md)
- [ ] [과제 04: 프로토콜과 전송 계층 분리](./assignments/04-protocol-transport-separation.md)

## Phase 2. UDP 데이터그램

- [ ] [과제 05: UDP Echo Server](./assignments/05-udp-echo-server.md)

## Phase 3. HTTP와 stateless 처리

- [ ] [과제 06: 최소 HTTP/1.1 서버](./assignments/06-minimal-http-server.md)
- [ ] [과제 07: HTTP keep-alive와 요청 반복](./assignments/07-http-keep-alive.md)

## Phase 4. Stateful 프로토콜

- [ ] [과제 08: 상태 기반 채팅 서버](./assignments/08-stateful-chat-server.md)

## Phase 5. 미니 inetd

- [ ] [과제 09: 단일 서비스 TCP inetd](./assignments/09-single-service-inetd.md)
- [ ] [과제 10: 설정 기반 다중 서비스와 종료 관리](./assignments/10-multi-service-inetd.md)

## Phase 6. 전송 구현 교체

- [ ] [과제 11: OS 시스템 콜 기반 TCP 구현](./assignments/11-syscall-tcp-transport.md)

## Phase 7. 비교와 선택 과제

- [ ] [과제 12: 동기 구현과 비동기 구현 비교](./assignments/12-sync-async-comparison.md)
- [ ] [선택 과제 13: 사용자 공간 IPv4·UDP 스택](./assignments/13-userspace-ipv4-udp-stack.md)

## 현재 과제

[과제 01: 단일 클라이언트 TCP Echo Server](./assignments/01-tcp-echo-server.md)

과제 01을 완료하기 전에는 구조 확장, HTTP 구현이나 시스템 콜 구현을 시작하지 않는다. 먼저 TCP stream의 읽기, 쓰기와 종료 의미를 직접 경험한다.
