# 선택 과제 13: 사용자 공간 IPv4·UDP 스택

## 선수 조건

- 필수 과제 01~12를 완료했다.
- 시스템 콜 기반 socket과 사용자 공간 네트워크 스택의 차이를 설명할 수 있다.
- raw packet 처리의 보안 및 시스템 위험을 이해하고 격리된 실습 환경을 준비했다.

## 학습 목표

- 네트워크 패킷의 binary layout을 직접 파싱한다.
- IPv4, ICMP와 UDP의 최소 규칙을 구현한다.
- 커널이 기존 socket API 아래에서 제공하던 기능을 식별한다.

## 문제

운영체제의 TCP/UDP 소켓 구현을 사용하는 대신 가상 네트워크 인터페이스에서 패킷을 읽어 최소 IPv4와 UDP 처리를 직접 구현한다. 이 과제는 별도 프로젝트로 분리해도 된다.

## 제한 조건

- 실제 물리 인터페이스보다 격리 가능한 TUN 또는 동등한 가상 인터페이스를 우선 사용한다.
- 구현할 RFC 범위와 지원하지 않는 기능을 먼저 명시한다.
- 길이, version, checksum과 protocol field를 검증하기 전에 payload를 신뢰하지 않는다.
- packet parser와 I/O를 분리한다.
- 최소 IPv4·ICMP·UDP 범위를 완료하기 전 TCP를 시작하지 않는다.
- 관리자 권한이 필요한 명령과 시스템 설정 변경을 문서화하고 최소화한다.

## 구현 순서

1. packet capture 또는 fixture로 IPv4 header layout을 관찰한다.
2. 순수 byte parser와 serializer를 테스트부터 작성한다.
3. version, header length, total length와 checksum을 검증한다.
4. ICMP Echo request와 response를 처리한다.
5. UDP header, port, length와 checksum 정책을 구현한다.
6. 가상 인터페이스에서 왕복 통신을 재현한다.
7. 최소 TCP state machine을 후속 과제로 진행할지 판단한다.

## 완료 조건

- raw byte에서 IPv4 packet의 주요 필드를 설명하고 검증한다.
- 잘린 packet, 잘못된 길이와 checksum 오류를 안전하게 거부한다.
- ICMP 또는 UDP 중 정한 범위의 왕복 통신을 재현한다.
- parser는 실제 TUN 장치 없이 fixture로 테스트할 수 있다.
- 과제 11과 사용자 공간 네트워크 스택의 차이를 설명한다.

## 직접 조사할 질문

- IPv4 header length와 total length는 각각 무엇을 의미하는가?
- network byte order를 parser 전체에서 어떻게 일관되게 유지할 것인가?
- one's complement checksum은 어떻게 계산되는가?
- TUN과 TAP은 어떤 계층의 데이터를 제공하는가?
- fragmentation을 지원하지 않을 경우 어떤 packet을 어떻게 거부할 것인가?
- TCP state machine을 시작하기 전에 어떤 추가 개념이 필요한가?

## 완료 기록

- 구현한 프로토콜 및 RFC 범위:
- 지원하지 않는 기능:
- 잘못된 packet 검증 사례:
- 왕복 통신 검증 방법:
- 커널 socket 구현과 직접 stack 구현의 차이:

## 막혔을 때 질문 형식

> 선택 과제 13을 진행 중이다. packet bytes와 내가 파싱한 header 값은 ___이다. 검증은 ___ 순서로 한다. 전체 parser를 주지 말고 잘못 해석한 field나 RFC 범위만 알려줘.
