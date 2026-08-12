# 선택 과제 13: 사용자 공간 IPv4·UDP 스택

## 선수 조건

- 필수 과제 01~12를 완료했다.
- 시스템 콜 기반 socket과 사용자 공간 네트워크 스택의 차이를 설명할 수 있다.
- raw packet 처리의 보안 및 시스템 위험을 이해하고 격리된 실습 환경을 준비했다.

## 학습 목표

- 네트워크 패킷의 binary layout을 직접 파싱한다.
- IPv4, ICMP와 UDP의 최소 규칙을 구현한다.
- 커널이 기존 socket API 아래에서 제공하던 기능을 식별한다.

## 이번 과제에서 익힐 Rust

- byte slice에서 고정 크기 field를 안전하게 꺼내는 pattern
- shift, mask와 bitwise 연산으로 packed field를 해석하는 방법
- `checked_add`, `checked_mul`과 범위 검증으로 parser overflow를 막는 방법
- `TryFrom<&[u8]>`, struct와 error enum으로 binary parser 계약을 표현하는 방법
- iterator의 `fold` 또는 명시적 반복문으로 checksum을 계산하는 방법
- serializer가 소유 buffer를 만들지 caller buffer에 쓰게 할지 선택하는 방법

## 검색 키워드

- byte parsing: `Rust parse byte slice fixed array TryInto`, `Rust binary parser TryFrom slice`, `Rust checked arithmetic parser`
- bit 연산: `Rust bit shift mask u8`, `Rust wrapping add checksum`
- protocol: `RFC 791 IPv4 header`, `RFC 792 ICMP echo`, `RFC 768 UDP`, `internet checksum one's complement`
- 가상 인터페이스 공통: `TUN TAP layer 3 layer 2 difference`
- Linux 실습 경로: `Linux TUN /dev/net/tun Rust`, `Linux tuntap documentation`
- macOS 실습 경로: `Apple NEPacketTunnelProvider Network Extension entitlement`, `macOS packet tunnel Xcode app extension`
- 테스트: `Rust packet parser fixture hex bytes`, `Rust property test parser malformed input`

## 문제

먼저 실제 장치 없이 packet fixture로 최소 IPv4와 UDP parser·serializer를 구현한다. 그다음 운영체제의 TCP/UDP 소켓 대신 가상 네트워크 인터페이스에서 packet을 읽고 쓰는 플랫폼 통합을 별도 단계로 진행한다. 이 과제는 별도 프로젝트로 분리해도 된다.

## 제한 조건

- 실제 물리 인터페이스보다 격리 가능한 TUN 또는 동등한 가상 인터페이스를 우선 사용한다.
- packet parser·serializer 완료와 실제 가상 인터페이스 연결을 서로 다른 checkpoint로 관리한다.
- 실제 packet I/O는 `Linux VM + TUN` 또는 `macOS Network Extension` 중 하나를 선택한다. 평범한 Cargo CLI만으로 두 경로가 같다고 가정하지 않는다.
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
6. portable checkpoint로 parser와 serializer의 정상·오류 fixture 테스트를 완료한다.
7. 실제 I/O를 진행한다면 Linux VM의 TUN 또는 macOS Network Extension 중 한 경로의 권한, host 구성과 packet 입출력 방식을 먼저 문서화한다.
8. 선택한 가상 인터페이스에서 왕복 통신을 재현한다.
9. 최소 TCP state machine을 후속 과제로 진행할지 판단한다.

## 단계별 힌트

1. 실제 가상 인터페이스를 열기 전에 고정된 packet fixture만 받는 순수 parser부터 만든다.
2. 최소 header 길이를 확인하기 전에는 version/IHL field 외의 index에 접근하지 않는다.
3. IHL과 total length를 byte 단위로 바꿀 때 overflow, 최소값과 실제 slice 길이를 순서대로 검증한다.
4. 여러 byte 정수는 직접 shift하거나 `from_be_bytes`를 사용할 수 있다. parser 전체에서 한 방식을 일관되게 선택한다.
5. checksum 계산 함수는 packet parser와 분리해 정상, 한 bit 손상과 홀수 길이 입력을 독립적으로 테스트한다.
6. 실제 I/O 계층은 `읽은 packet → parser → protocol 처리 → serializer → write`만 조정하게 하고 header 규칙을 직접 알지 않게 한다.
7. macOS의 공개 Network Extension 경로는 Xcode app extension과 entitlement가 필요한 별도 플랫폼 통합이다. Cargo CLI 실습을 우선하려면 격리된 Linux VM의 TUN 경로를 선택한다.

## 완료 조건

- raw byte에서 IPv4 packet의 주요 필드를 설명하고 검증한다.
- 잘린 packet, 잘못된 길이와 checksum 오류를 안전하게 거부한다.
- parser는 실제 TUN 장치 없이 fixture로 테스트할 수 있다.
- 과제 11과 사용자 공간 네트워크 스택의 차이를 설명한다.
- **Portable checkpoint:** serializer 결과가 다시 parser를 통과하고 정상·잘린 packet·잘못된 길이·checksum 오류 fixture 테스트가 모두 통과한다.
- **Platform I/O checkpoint:** 선택한 실행 환경의 권한과 설정을 문서화하고 ICMP 또는 UDP 중 정한 범위의 왕복 통신을 재현한다.

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
