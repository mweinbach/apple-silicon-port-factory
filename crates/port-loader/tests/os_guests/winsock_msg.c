#include "check.h"
#include <winsock2.h>
#include <ws2tcpip.h>
#include <mswsock.h>
#include <mstcpip.h>

static void *extension(SOCKET s, GUID guid) {
    void *p = NULL;
    DWORD n = 0;
    if (WSAIoctl(s, SIO_GET_EXTENSION_FUNCTION_POINTER, &guid, sizeof(guid), &p, sizeof(p), &n, NULL, NULL) || n != sizeof(p)) return NULL;
    return p;
}
int main(void) {
    WSADATA data;
    CHECK(!WSAStartup(MAKEWORD(2,2), &data));
    SOCKET udp = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP), tcp = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP);
    CHECK(udp != INVALID_SOCKET && tcp != INVALID_SOCKET);
    CHECK(extension(tcp, (GUID)WSAID_ACCEPTEX) && extension(tcp, (GUID)WSAID_CONNECTEX));
    CHECK(extension(tcp, (GUID)WSAID_GETACCEPTEXSOCKADDRS) && extension(tcp, (GUID)WSAID_TRANSMITFILE));
    LPFN_WSARECVMSG receive = extension(udp, (GUID)WSAID_WSARECVMSG);
    LPFN_WSASENDMSG send_msg = extension(udp, (GUID)WSAID_WSASENDMSG);
    CHECK(receive && send_msg);
    struct tcp_keepalive ka = {1, 5000, 2000};
    DWORD n = 0;
    OVERLAPPED configured = {0};
    configured.hEvent = CreateEventW(NULL, TRUE, FALSE, NULL);
    CHECK(configured.hEvent && !WSAIoctl(tcp, SIO_KEEPALIVE_VALS, &ka, sizeof(ka), NULL, 0, &n, &configured, NULL));
    CHECK(WaitForSingleObject(configured.hEvent, 1000) == WAIT_OBJECT_0 && n == 0);
    CloseHandle(configured.hEvent);
    int opt = 0, size = sizeof(opt);
    CHECK(!getsockopt(tcp, SOL_SOCKET, SO_KEEPALIVE, (char *)&opt, &size) && opt);
    closesocket(tcp);

    SOCKADDR_IN loop = {0}, from = {0};
    loop.sin_family = AF_INET;
    loop.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    CHECK(!bind(udp, (SOCKADDR *)&loop, sizeof(loop)));
    size = sizeof(loop);
    CHECK(!getsockname(udp, (SOCKADDR *)&loop, &size));
    DWORD on = 1;
    CHECK(!setsockopt(udp, IPPROTO_IP, IP_PKTINFO, (char *)&on, sizeof(on)));
    char input[16], control[128], outbound[WSA_CMSG_SPACE(sizeof(IN_PKTINFO))] = {0};
    WSABUF chunks[] = {{2, "ab"}, {3, "cde"}}, result = {sizeof(input), input};
    WSAMSG tx = {0}, rx = {0};
    tx.name = (SOCKADDR *)&loop; tx.namelen = sizeof(loop);
    tx.lpBuffers = chunks; tx.dwBufferCount = 2;
    tx.Control.buf = outbound; tx.Control.len = sizeof(outbound);
    WSACMSGHDR *h = WSA_CMSG_FIRSTHDR(&tx);
    h->cmsg_len = WSA_CMSG_LEN(sizeof(IN_PKTINFO)); h->cmsg_level = IPPROTO_IP; h->cmsg_type = IP_PKTINFO;
    ((IN_PKTINFO *)WSA_CMSG_DATA(h))->ipi_addr = loop.sin_addr;

    rx.name = (SOCKADDR *)&from; rx.namelen = sizeof(from);
    rx.lpBuffers = &result; rx.dwBufferCount = 1; rx.Control.buf = control; rx.Control.len = sizeof(control);
    OVERLAPPED ov = {0};
    ov.hEvent = CreateEventW(NULL, TRUE, FALSE, NULL);
    CHECK(receive(udp, &rx, NULL, &ov, NULL) == SOCKET_ERROR && WSAGetLastError() == WSA_IO_PENDING);
    CHECK(!send_msg(udp, &tx, 0, &n, NULL, NULL) && n == 5);
    CHECK(WaitForSingleObject(ov.hEvent, 5000) == WAIT_OBJECT_0);
    DWORD flags = 0;
    CHECK(WSAGetOverlappedResult(udp, &ov, &n, FALSE, &flags) && n == 5 && !memcmp(input, "abcde", 5));
    CHECK(from.sin_port == loop.sin_port && from.sin_addr.s_addr == loop.sin_addr.s_addr);
    h = WSA_CMSG_FIRSTHDR(&rx);
    CHECK(h && h->cmsg_level == IPPROTO_IP && h->cmsg_type == IP_PKTINFO && h->cmsg_len == WSA_CMSG_LEN(sizeof(IN_PKTINFO)));
    IN_PKTINFO *pi = (IN_PKTINFO *)WSA_CMSG_DATA(h);
    CHECK(pi->ipi_addr.s_addr == loop.sin_addr.s_addr && pi->ipi_ifindex != 0 && !rx.dwFlags);
    tx.Control.len = 0; tx.Control.buf = NULL;
    CHECK(!WSASendMsg(udp, &tx, 0, &n, NULL, NULL));
    rx.Control.len = 1; rx.namelen = sizeof(from); rx.dwFlags = 0;
    CHECK(receive(udp, &rx, &n, NULL, NULL) == SOCKET_ERROR && WSAGetLastError() == WSAEMSGSIZE && (rx.dwFlags & MSG_CTRUNC));
    CloseHandle(ov.hEvent); closesocket(udp); WSACleanup();
    puts("PASS winsock_msg");
    return 0;
}
